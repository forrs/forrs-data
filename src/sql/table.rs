use crate::{id::Id, Category, Post, Topic, User};

pub trait Table: Sized {
    fn tablename() -> &'static str;
    fn name_field() -> Option<&'static str>;
    fn id(&self) -> Id;
}

impl Table for Category {
    fn tablename() -> &'static str {
        "Category"
    }
    fn name_field() -> Option<&'static str> {
        Some("name")
    }
    fn id(&self) -> Id {
        self.id
    }
}

impl Table for Post {
    fn tablename() -> &'static str {
        "Post"
    }
    fn name_field() -> Option<&'static str> {
        None
    }
    /// TODO: Actually figure out Post Ids.
    fn id(&self) -> Id {
        self.id.id
    }
}

impl Table for Topic {
    fn tablename() -> &'static str {
        "Topic"
    }
    fn name_field() -> Option<&'static str> {
        Some("title")
    }
    fn id(&self) -> Id {
        self.id
    }
}

impl Table for User {
    fn tablename() -> &'static str {
        r#""User""#
    }
    fn name_field() -> Option<&'static str> {
        Some("name")
    }
    fn id(&self) -> Id {
        self.id
    }
}
