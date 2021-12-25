use serde::{Deserialize, Serialize};

use crate::models::{Id, Response, Snippet};

pub type SearchResponse = Response<SearchResult>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SearchResult {
    pub kind: String,
    pub etag: String,
    pub id: Id,
    pub snippet: Snippet
}
