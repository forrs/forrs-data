use crate::model::*;
use tokio_postgres::{Error, Row};

pub trait FromRow: Sized {
    fn from_row(row: &Row) -> Result<Self, Error>;
}

impl FromRow for User {
    fn from_row(row: &Row) -> Result<Self, Error> {
        Ok(User {
            id: row.try_get("id")?,
            name: row.try_get("name")?,
            pw_hash: row.try_get("pw_hash")?,
            registered_at: row.try_get("registered_at")?,
        })
    }
}

impl FromRow for Category {
    fn from_row(row: &Row) -> Result<Self, Error> {
        Ok(Category {
            id: row.try_get("id")?,
            name: row.try_get("name")?,
        })
    }
}

impl FromRow for Topic {
    fn from_row(row: &Row) -> Result<Self, Error> {
        Ok(Topic {
            id: row.try_get("id")?,
            category_id: row.try_get("category_id")?,
            creator_id: row.try_get("creator_id")?,
            created_at: row.try_get("created_at")?,
            edited_at: row.try_get("edited_at")?,
            title: row.try_get("title")?,
        })
    }
}

impl FromRow for PostId {
    fn from_row(row: &Row) -> Result<Self, Error> {
        Ok(PostId {
            topic_id: row.try_get("topic_id")?,
            id: row.try_get("id")?,
        })
    }
}

impl FromRow for Post {
    fn from_row(row: &Row) -> Result<Self, Error> {
        Ok(Post {
            id: PostId::from_row(row)?,
            author_id: row.try_get("author_id")?,
            created_at: row.try_get("created_at")?,
            edited_at: row.try_get("edited_at")?,
            content: row.try_get("content")?,
        })
    }
}
