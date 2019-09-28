use serde::{Deserialize, Serialize};

#[cfg(any(
    target_pointer_width = "64",
    target_pointer_width = "128",
    feature = "big-numbers"
))]
pub type Id = u64;
#[cfg(not(any(
    target_pointer_width = "64",
    target_pointer_width = "128",
    feature = "big-numbers"
)))]
pub type Id = u32;

pub type DateTime = chrono::NaiveDateTime;

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
