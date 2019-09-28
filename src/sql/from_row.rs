use crate::model::*;
use mysql_async::{FromRowError, Row};

pub trait FromRow: Sized {
    fn from_row(row: Row) -> Result<Self, FromRowError>;
}

impl FromRow for User {
    fn from_row(row: Row) -> Result<Self, FromRowError> {
        let (id, name, pw_hash, registered_at) = mysql_async::from_row_opt(row)?;
        Ok(User {
            id,
            name,
            pw_hash,
            registered_at,
        })
    }
}

impl FromRow for Category {
    fn from_row(row: Row) -> Result<Self, FromRowError> {
        let (id, name) = mysql_async::from_row_opt(row)?;
        Ok(Category { id, name })
    }
}

impl FromRow for Topic {
    fn from_row(row: Row) -> Result<Self, FromRowError> {
        let (id, category_id, creator_id, created_at, edited_at, title) =
            mysql_async::from_row_opt(row)?;
        Ok(Topic {
            id,
            category_id,
            creator_id,
            created_at,
            edited_at,
            title,
        })
    }
}

impl FromRow for Post {
    fn from_row(row: Row) -> Result<Self, FromRowError> {
        let (topic_id, id, author_id, created_at, edited_at, content) =
            mysql_async::from_row_opt(row)?;
        let id = PostId { id, topic_id };
        Ok(Post {
            id,
            author_id,
            created_at,
            edited_at,
            content,
        })
    }
}
