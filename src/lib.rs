use serde::{Deserialize, Serialize};

pub type Id = i64;
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
    pub category_id: Id,
    pub creator_id: Id,
    pub created_at: DateTime,
    pub edited_at: Option<DateTime>,
    pub title: String,
}
#[derive(Serialize, Deserialize)]
pub struct Post {
    // TODO: Compound Id field, maybe we could use a struct here
    pub topic_id: Id,
    pub id: u32,
    pub author_id: Id,
    pub created_at: DateTime,
    pub edited_at: Option<DateTime>,
    pub content: String,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}