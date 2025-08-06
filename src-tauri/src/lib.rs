use rusqlite::Connection;
use std::sync::Mutex;
use tauri::{Manager, State};
use uuid::Uuid;

pub struct DbState {
    db: Mutex<Option<Connection>>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct Activity {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    value: f64,
    medium: MonetaryMedium,
    operation: Operation,
    description: String,
    date: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<Vec<Tag>>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug)]
#[serde(rename_all = "camelCase")]
struct MonetaryMedium {
    name: String,
    id: String,
    is_valid_for_credit: bool,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct Tag {
    id: Option<String>,
    name: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
enum Operation {
    Credit,
    Debit,
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
            FOREIGN KEY (medium_id) REFERENCES monetary_medium(id)
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
fn get_activities<'a>(state: State<'a, DbState>) -> Result<Vec<Activity>, String>{
    let db_guard = state.db.lock().unwrap();
    let conn = db_guard.as_ref().ok_or("Couldn't recover database connection")?;

    // Query to get all activities with their associated monetary medium
    let mut stmt = conn.prepare(
        "SELECT a.id, a.value, a.medium_id, a.operation, a.description, a.date, 
                m.name as medium_name, m.is_valid_for_credit 
         FROM activity a 
         JOIN monetary_medium m ON a.medium_id = m.id"
    ).map_err(|e| e.to_string())?;

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
                name: row.get(6)?,
                is_valid_for_credit: row.get(7)?
            },
            operation,
            description: row.get(4)?,
            date: row.get(5)?,
            tags: None  // Will be populated below
        })
    }).map_err(|e| e.to_string())?;

    let mut activities = Vec::new();
    
    // Process each activity and fetch its tags
    for activity_result in activity_iter {
        let mut activity = activity_result.map_err(|e| e.to_string())?;
        
        println!("Current activity: {:#?}", activity.description);


        // Fetch tags for this activity
        if let Some(activity_id) = &activity.id {
            let tags = get_activity_tags(&conn, activity_id).map_err(|e| e.to_string())?;
            activity.tags = if tags.is_empty() { None } else { Some(tags) };
        }
        
        activities.push(activity);
    }

    Ok(activities)
}

// Helper function to get tags for a specific activity
fn get_activity_tags(conn: &Connection, activity_id: &str) -> Result<Vec<Tag>, rusqlite::Error> {
    let mut stmt = conn.prepare(
        "SELECT t.id, t.name 
         FROM tag t 
         JOIN activity_tag at ON t.id = at.tag_id 
         WHERE at.activity_id = ?"
    )?;
    
    let tag_iter = stmt.query_map([activity_id], |row| {
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
        "INSERT INTO activity (id, value, medium_id, operation, description, date) 
         VALUES (?, ?, ?, ?, ?, ?)",
        [
            &activity_id,
            &activity.value.to_string(),
            &activity.medium.id,
            &match activity.operation {
                Operation::Credit => "credit".to_string(),
                Operation::Debit => "debit".to_string(),
            },
            &activity.description,
            &activity.date,
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
        .invoke_handler(tauri::generate_handler![greet, get_activities, add_activity, get_monetary_media, delete_activity])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
