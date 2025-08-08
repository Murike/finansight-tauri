#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Activity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub value: f64,
    pub medium: MonetaryMedium,
    pub operation: Operation,
    pub description: String,
    pub date: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug)]
#[serde(rename_all = "camelCase")]
pub struct MonetaryMedium {
    pub name: String,
    pub id: String,
    pub is_valid_for_credit: bool,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tag {
    pub id: Option<String>,
    pub name: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StaticFilter {
    pub id: String,
    pub initial_date: String,
    pub final_date: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityFilter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub final_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Operation {
    Credit,
    Debit,
}
