use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MarkerCollection {
    pub code: i8,
    pub result: Vec<Marker>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Marker {
    pub id: String,
    pub code: String,
    pub name: String,
    pub marker_id: String,
    pub __text: Option<Vec<String>>,
}
