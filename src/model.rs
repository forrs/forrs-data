use serde::{Deserialize, Serialize};

pub mod id;
use id::*;

pub type DateTime = chrono::DateTime<chrono::Utc>;

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: Id,
    pub name: String,
    pub pw_hash: String,
    pub registered_at: DateTime,
    // TODO: More fields...
}
#[derive(Serialize, Deserialize)]
pub struct Category {
    pub id: Id,
    pub name: String,
}
#[derive(Serialize, Deserialize)]
pub struct Topic {
    pub id: Id,
    pub category_id: Option<Id>,
    pub creator_id: Id,
    pub created_at: DateTime,
    pub edited_at: Option<DateTime>,
    pub title: String,
}

mod post;
pub use post::*;
