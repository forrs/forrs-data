use crate::{id::Id, Category, Post, Topic, User};

pub trait Table: Sized {
    const TABLENAME: &'static str;
    const NAME_FIELD: Option<&'static str>;
    fn id(&self) -> Id;
}

impl Table for Category {
    const TABLENAME: &'static str = "Category";
    const NAME_FIELD: Option<&'static str> = Some("name");
    fn id(&self) -> Id {
        self.id
    }
}

impl Table for Post {
    const TABLENAME: &'static str = "Post";
    const NAME_FIELD: Option<&'static str> = None;
    /// TODO: Actually figure out Post Ids.
    fn id(&self) -> Id {
        self.id.id
    }
}

impl Table for Topic {
    const TABLENAME: &'static str = "Topic";
    const NAME_FIELD: Option<&'static str> = Some("title");
    fn id(&self) -> Id {
        self.id
    }
}

impl Table for User {
    const TABLENAME: &'static str = r#""User""#;
    const NAME_FIELD: Option<&'static str> = Some("name");
    fn id(&self) -> Id {
        self.id
    }
}
