use super::{DateTime, Id};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Copy, Clone, PartialEq)]
pub struct PostId {
    pub topic_id: Id,
    pub id: u32,
}
#[derive(Serialize, Deserialize)]
pub struct Post {
    pub id: PostId,
    pub author_id: Id,
    pub created_at: DateTime,
    pub edited_at: Option<DateTime>,
    pub content: String,
}
