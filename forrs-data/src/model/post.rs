use forrs_stm::id::Id;
use serde::{Deserialize, Serialize};

use super::DateTime;

#[derive(Serialize, Deserialize, Copy, Clone, PartialEq)]
pub struct PostId {
    pub topic_id: Id,
    pub id: Id,
}
#[derive(Serialize, Deserialize)]
pub struct Post {
    pub id: PostId,
    pub author_id: Id,
    pub created_at: DateTime,
    pub edited_at: Option<DateTime>,
    pub content: String,
}
