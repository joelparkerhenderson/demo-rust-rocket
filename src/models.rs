#[derive(Debug, Queryable)]
pub struct Item {
    pub id: i32,
    pub demo_boolean: Option<bool>,
    pub demo_int: Option<i32>,
    pub demo_json: Option<serde_json::Value>,
    pub demo_text: Option<String>,
    pub demo_timestamp: Option<chrono::NaiveDateTime>,
}
