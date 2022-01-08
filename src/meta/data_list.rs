use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct MetaData {
  title: String,
  description: String,
  created_at: String,
}

#[derive(Serialize, Deserialize)]
pub struct DataList {
  data: Vec<MetaData>,
}
