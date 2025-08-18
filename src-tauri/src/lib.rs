mod models;

use rusqlite::{params, Connection};
use std::sync::Mutex;
use tauri::{Manager, State};
use uuid::Uuid;
use crate::models::{Activity, MonetaryMedium, Tag, StaticFilter, Operation, ActivityFilter};


pub struct DbState {
    db: Mutex<Option<Connection>>,
}


fn db_init(app_handle: &tauri::AppHandle) -> Result<Connection, rusqlite::Error> {
    // Get the path to the app's data directory
    let app_dir = app_handle.path().app_data_dir().expect("Failed to get app data dir");
    // Ensure the directory exists
    std::fs::create_dir_all(&app_dir).expect("Failed to create app data dir");
    
    // The path to the database file
    let db_path = app_dir.join("finansight.db");

    // Open a connection to the database
    let conn = Connection::open(db_path)?;

    // conn.execute("DROP TABLE IF EXISTS activity_tag;", [])?;
    // conn.execute("DROP TABLE IF EXISTS filter_tag;", [])?;
    // conn.execute("DROP TABLE IF EXISTS tag;", [])?;
    // conn.execute("DROP TABLE IF EXISTS activity;", [])?;
    // conn.execute("DROP TABLE IF EXISTS monetary_medium;", [])?;
    // conn.execute("DROP TABLE IF EXISTS static_filters;", [])?;

    // Create the 'users' table if it doesn't exist
    conn.execute(
        "CREATE TABLE IF NOT EXISTS monetary_medium (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            is_valid_for_credit INTEGER NOT NULL
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS tag (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL UNIQUE
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS activity (
            id TEXT PRIMARY KEY,
            value REAL NOT NULL,
            medium_id TEXT NOT NULL,
            operation TEXT NOT NULL CHECK (operation IN ('credit', 'debit')),
            description TEXT NOT NULL,
            date TEXT NOT NULL,
            parent_id TEXT,
            FOREIGN KEY (medium_id) REFERENCES monetary_medium(id),
            FOREIGN KEY (parent_id) REFERENCES activity(id)
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS activity_tag (
            activity_id TEXT NOT NULL,
            tag_id TEXT NOT NULL,
            PRIMARY KEY (activity_id, tag_id),
            FOREIGN KEY (activity_id) REFERENCES activity(id),
            FOREIGN KEY (tag_id) REFERENCES tag(id)
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS static_filters (
            id TEXT PRIMARY KEY,
            initial_date TEXT NOT NULL,
            final_date TEXT NOT NULL
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS filter_tag (
            filter_id TEXT NOT NULL,
            tag_id TEXT NOT NULL,
            PRIMARY KEY (filter_id, tag_id),
            FOREIGN KEY (filter_id) REFERENCES static_filters(id),
            FOREIGN KEY (tag_id) REFERENCES tag(id)
        )",
        [],
    )?;

    Ok(conn)
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}




#[tauri::command]
async fn get_monetary_media<'a>(state: State<'a, DbState>) -> Result<Vec<MonetaryMedium>, String> {
  let db_guard = state.db.lock().unwrap();
  let conn = db_guard.as_ref().ok_or("Couldn't recover database connection.")?;
    
  let mut stmt = conn.prepare("SELECT * from monetary_medium").map_err(|e| e.to_string())?;
  let monetary_data_itr = stmt.query_map([], |row| {
    Ok(MonetaryMedium {
        id: row.get(0)?,
        name: row.get(1)?,
        is_valid_for_credit: row.get(2)?
    })
  }).map_err(|e| e.to_string())?;

//   let monetary_data: Vec<MonetaryMedium> = monetary_data.collect().map_err(|e| e.to_string())?;
  
  let mut monetary_data = Vec::new();
  for datum in monetary_data_itr {
    monetary_data.push(datum.map_err(|e| e.to_string())?);
  }

  Ok(monetary_data)
}

#[tauri::command]
fn get_activities<'a>(state: State<'a, DbState>, filters: Option<ActivityFilter>) -> Result<Vec<Activity>, String>{
    let db_guard = state.db.lock().unwrap();
    let conn = db_guard.as_ref().ok_or("Couldn't recover database connection")?;

    let where_clause : String = match &filters {
        Some(filter) => {
            let mut where_statements : Vec<String> = Vec::new();
            
            if let Some(date) = &filter.initial_date {
                where_statements.push(format!(" a.date > '{}' ", *date));
            }

            if let Some(date) = &filter.final_date {
                where_statements.push(format!(" a.date < '{}' ", *date));
            }

            if !where_statements.is_empty() {
                format!(" WHERE {} ", where_statements.join(" AND "))
            } else {
                String::from("")
            }
        },
        None => String::from("")
    };
    
    let formatted_query = format!("
        SELECT a.id, a.value, a.medium_id, a.operation, a.description, a.date, a.parent_id,
            m.name as medium_name, m.is_valid_for_credit 
        FROM activity a 
        JOIN monetary_medium m ON a.medium_id = m.id
        {}", &where_clause);

    let mut stmt = conn.prepare(&formatted_query).map_err(|e| e.to_string())?;

    let activity_iter = stmt.query_map([], |row| {
        let operation = match (row.get::<_, String>(3)?).as_str() {
            "credit" => Ok(Operation::Credit),
            "debit" => Ok(Operation::Debit),
            _ => Err(rusqlite::Error::InvalidQuery)
        }?;
        
        Ok(Activity {
            id: Some(row.get(0)?),
            value: row.get(1)?,
            medium: MonetaryMedium {
                id: row.get(2)?,
                name: row.get(7)?,
                is_valid_for_credit: row.get(8)?
            },
            operation,
            description: row.get(4)?,
            date: row.get(5)?,
            parent_id: row.get(6)?,
            tags: None  // Will be populated below
        })
    }).map_err(|e| e.to_string())?;

    let mut activities = Vec::new();
    
    // Process each activity and fetch its tags
    for activity_result in activity_iter {
        let mut activity = activity_result.map_err(|e| e.to_string())?;

        // Fetch tags for this activity
        if let Some(activity_id) = &activity.id {
            let tags = get_activity_tags(&conn, activity_id).map_err(|e| e.to_string())?;
            activity.tags = if tags.is_empty() { None } else { Some(tags) };
        }

        // Apply filter rules:
        // 1) No filter -> include
        // 2) Filter with no tags -> include
        // 3) Filter with tags -> include only if all filter tag ids are present in activity tags
        match &filters {
            None => activities.push(activity),
            Some(filter) => {
                match &filter.tags {
                    None => activities.push(activity),
                    Some(filter_tags) if filter_tags.is_empty() => activities.push(activity),
                    Some(filter_tags) => {
                        if let Some(activity_tags) = &activity.tags {
                            let is_subset = filter_tags
                                .iter()
                                .any(|ft| activity_tags.iter().any(|at| at.id == ft.id));
                            if is_subset {
                                activities.push(activity);
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(activities)
}

// Helper function to get tags for a specific activity
fn get_activity_tags(conn: &Connection, activity_id: &str) -> Result<Vec<Tag>, rusqlite::Error> {
    get_related_tags(&conn, "SELECT t.id, t.name 
         FROM tag t 
         JOIN activity_tag at ON t.id = at.tag_id 
         WHERE at.activity_id = ?", activity_id)
}

#[tauri::command]
fn get_static_filters(state: State<DbState>) -> Result<Vec<StaticFilter>, String> {
    let mut db_guard = state.db.lock().unwrap();
    let conn = db_guard.as_mut().ok_or("Failure connecting to database")?;

    let mut stmt = conn.prepare("SELECT * FROM static_filters").map_err(|e| e.to_string())?;
    let filter_iter = stmt.query_map([], |row| {
        Ok(StaticFilter {
            id: row.get(0)?,
            initial_date: row.get(1)?,
            final_date: row.get(2)?,
            tags: None,
        })
    }).map_err(|e| e.to_string())?;

    let mut filters = Vec::new();
    for filter in filter_iter {
        let mut local_filter = filter.map_err(|e| e.to_string())?;

        let tags = get_filter_tags(&conn, &local_filter.id).map_err(|e| e.to_string())?;
        local_filter.tags = if tags.is_empty() { None } else { Some(tags) };
        filters.push(local_filter);
    }
    
    Ok(filters)
}

fn get_filters_actual(conn: &Connection, filter_id: Option<&str>) -> Result<Vec<StaticFilter>, String> {
    let formatted_query = if let Some(id) = filter_id {
        format!("SELECT * FROM static_filters WHERE id = {} ", id)
    } else {
        format!("SELECT * FROM static_filters")
    };

    let mut stmt = conn.prepare(&formatted_query).map_err(|e| e.to_string())?;
    let filter_iter = stmt.query_map([], |row| {
        Ok(StaticFilter {
            id: row.get(0)?,
            initial_date: row.get(1)?,
            final_date: row.get(2)?,
            tags: None,
        })
    }).map_err(|e| e.to_string())?;

    let mut filters = Vec::new();
    for filter in filter_iter {
        let mut local_filter = filter.map_err(|e| e.to_string())?;

        let tags = get_filter_tags(&conn, &local_filter.id).map_err(|e| e.to_string())?;
        local_filter.tags = if tags.is_empty() { None } else { Some(tags) };
        filters.push(local_filter);
    }

    Ok(filters)
}

fn get_filter_tags(conn: &Connection, filter_id: &str) -> Result<Vec<Tag>, rusqlite::Error> {
    get_related_tags(&conn, "SELECT t.id, t.name 
         FROM tag t 
         JOIN filter_tag at ON t.id = at.tag_id 
         WHERE at.filter_id = ?", filter_id)
}


#[tauri::command]
fn update_filter_daterange(state: State<DbState>, filter: StaticFilter) -> Result<StaticFilter, String> {
    let mut db_guard = state.db.lock().unwrap();
    let conn = db_guard.as_mut().ok_or("Failure in recovering connection for query processing")?;

    conn.execute("UPDATE static_filters SET initial_date = ?, final_date = ?  WHERE id = ?", [&filter.initial_date, &filter.final_date, &filter.id]).map_err(|e| e.to_string())?;

    Ok(filter)
}


#[tauri::command]
fn add_filter_tag(state: State<DbState>, filter_id: &str, tag_id: &str) -> Result<StaticFilter, String> {
    let mut db_guard = state.db.lock().unwrap();
    let conn = db_guard.as_mut().ok_or("Failure in recovering connection for query processing")?;

    conn.execute("INSERT INTO filter_tag VALUES (?, ?)", [&filter_id, &tag_id]).map_err(|e| e.to_string())?;
    let filters = get_filters_actual(&conn, Some(filter_id)).map_err(|e| e.to_string())?;
    let filter = filters.into_iter().next().ok_or("Can't fetch updated filter from database.")?;

    Ok(filter)
}


#[tauri::command]
fn remove_filter_tag(state: State<DbState>, filter_id: &str, tag_id: &str) -> Result<StaticFilter, String> {
    let mut db_guard = state.db.lock().unwrap();
    let conn = db_guard.as_mut().ok_or("Failure in recovering connection for query processing")?;

    conn.execute("DELETE FROM filter_tag WHERE filter_id = ? AND tag_id = ?", [&filter_id, &tag_id]).map_err(|e| e.to_string())?;
    let filters = get_filters_actual(&conn, Some(filter_id)).map_err(|e| e.to_string())?;
    let filter = filters.into_iter().next().ok_or("Can't fetch updated filter from database.")?;

    Ok(filter)
}

fn get_related_tags(conn: &Connection, query: &str, reference_id: &str) -> Result<Vec<Tag>, rusqlite::Error> {
    let mut stmt = conn.prepare(query
    )?;
    
    let tag_iter = stmt.query_map([reference_id], |row| {
        Ok(Tag {
            id: Some(row.get(0)?),
            name: row.get(1)?
        })
    })?;
    
    let mut tags = Vec::new();
    for tag in tag_iter {
        tags.push(tag?);
    }
    
    Ok(tags)
}

#[tauri::command]
fn get_suggestion_tags(state: State<DbState>) -> Result<Vec<Tag>, String> {
    let mut db_guard = state.db.lock().unwrap();
    let conn = db_guard.as_mut().ok_or("Failure connecting to database")?;

    let mut stmt = conn.prepare("SELECT * FROM tag LIMIT 100").map_err(|e| e.to_string())?;
    let tag_iter = stmt.query_map([], |row| {
        Ok(Tag {
            id: Some(row.get(0)?),
            name: row.get(1)?
        })
    }).map_err(|e| e.to_string())?;

    let mut tags = Vec::new();
    for tag in tag_iter {
        tags.push(tag.map_err(|e| e.to_string())?);
    }
    
    Ok(tags)
}


#[tauri::command]
async fn delete_activity<'a>(state: State<'a, DbState>, activity_id: String) -> Result<usize, String> {
    let mut db_guard = state.db.lock().unwrap();
    let conn = db_guard.as_mut().ok_or("Failure connecting to database")?;

    conn.execute("DELETE FROM activity_tag WHERE activity_id = ?",
                [&activity_id]).map_err(|e| e.to_string())?;

    let rows_updated = conn.execute("DELETE FROM activity WHERE id = ?",
                [&activity_id]).map_err(|e| e.to_string())?;
    
    Ok(rows_updated)
}


#[tauri::command]
fn add_activity(state: State<DbState>, activity: Activity) -> Result<String, String> {
    let mut db_guard = state.db.lock().unwrap();
    let conn = db_guard.as_mut().ok_or("Failure connecting to database")?;
    
    // Generate a UUID for the new activity if not provided
    let activity_id = activity.id.unwrap_or_else(|| Uuid::new_v4().to_string());
    
    // Insert the activity
    conn.execute(
        "INSERT INTO activity (id, value, medium_id, operation, description, date, parent_id) 
         VALUES (?, ?, ?, ?, ?, ?, ?)",
         params![
            &activity_id,
            &activity.value.to_string(),
            &activity.medium.id,
            &match activity.operation {
                Operation::Credit => "credit".to_string(),
                Operation::Debit => "debit".to_string(),
            },
            &activity.description,
            &activity.date,
            activity.parent_id,
        ],
    ).map_err(|e| e.to_string())?;
    
    let mut activity_tags : Vec<(String, String)> = Vec::new();
    // Insert tags if any
    if let Some(tags) = &activity.tags {
        for tag in tags {
            let tag_id = match &tag.id {
                Some(id) => id.clone(),
                None => {
                    // Generate a new ID for this tag
                    let new_tag_id = Uuid::new_v4().to_string();
                    
                    // Try to insert the tag (might fail if name already exists)
                    conn.execute(
                        "INSERT OR IGNORE INTO tag (id, name) VALUES (?, ?)",
                        [&new_tag_id, &tag.name],
                    ).map_err(|e| e.to_string())?;

                    conn.query_row(
                            "SELECT id FROM tag WHERE name = ?",
                            [&tag.name],
                            |row| row.get(0),
                        ).map_err(|e| e.to_string())?
                }
            };
            
            activity_tags.push((activity_id.clone(), tag_id));
        }

        // Link the tags to the activity using a batch insert
        if !activity_tags.is_empty() {
            // Build a parameterized query with multiple value sets
            let placeholders: Vec<String> = (0..activity_tags.len())
                .map(|_| String::from("(?, ?)"))
                .collect();
            let values_clause = placeholders.join(", ");
            let sql = format!("INSERT OR REPLACE INTO activity_tag (activity_id, tag_id) VALUES {}", values_clause);
            
            // Flatten the vector of tuples into a vector of parameters
            let params: Vec<&dyn rusqlite::ToSql> = activity_tags.iter()
                .flat_map(|(activity_id, tag_id)| vec![activity_id as &dyn rusqlite::ToSql, tag_id as &dyn rusqlite::ToSql])
                .collect();
                
            conn.execute(&sql, &params[..]).map_err(|e| e.to_string())?;
        }
    }
    
    // Return the ID of the newly created activity
    Ok(activity_id)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(DbState{
            db: Mutex::new(None)
        })
        .setup(|app| {
            let state = app.state::<DbState>();
            let conn = db_init(app.handle()).expect("Database initialization failed");
            *state.db.lock().unwrap() = Some(conn);

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, 
                                                 get_activities, 
                                                 add_activity, 
                                                 get_monetary_media, 
                                                 delete_activity, 
                                                 get_suggestion_tags,
                                                 get_static_filters,
                                                 add_filter_tag,
                                                 update_filter_daterange,
                                                 remove_filter_tag])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
