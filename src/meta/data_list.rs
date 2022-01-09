use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct MetaData {
  pub title: String,
  pub description: String,
  pub emoji: String,
  pub is_published: bool,
  pub created_at: String,
}

#[derive(Serialize, Deserialize)]
pub struct MetaDataList(Vec<MetaData>);
impl Iterator for MetaDataList {
  type Item = MetaData;
  fn next(&mut self) -> Option<Self::Item> {
    self.0.pop()
  }
}
