use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GroupCollection {
    pub code: i8,
    pub result: Vec<Group>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Group {
    pub id: String,
    pub name: String,
    pub member_ids: Vec<String>,
    pub created_at: String,
    pub updated_at: String,
}
