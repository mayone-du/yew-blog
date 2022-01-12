use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ArticleMetaData {
  pub title: String,
  pub description: String,
  pub emoji: String,
  pub is_published: bool,
  pub created_at: String,
}

#[derive(Serialize, Deserialize)]
pub struct ArticleMetaDataList(Vec<ArticleMetaData>);
impl Iterator for ArticleMetaDataList {
  type Item = ArticleMetaData;
  fn next(&mut self) -> Option<Self::Item> {
    self.0.pop()
  }
}

// TODO: Others
pub struct OtherMetaData {}