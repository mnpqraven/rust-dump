use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PatchResponse {
    pub code: i8,
    pub result: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Permission {
    pub user_id: Option<String>,
    pub group_id: Option<String>,
    pub id: String,
    // TODO: try to implement Vec<Maker>
    pub marker_ids: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserAuth {
    pub account: String,
    pub password: String,
}
