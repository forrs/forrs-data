use crate::model::*;
use forrs_stm::{IdField, SelectAll, SelectById};

impl IdField for Category {}
impl SelectAll for Category {
    const SELECT_ALL: &'static str = "SELECT * FROM Category";
}
impl SelectById for Category {
    const SELECT_BY_ID: &'static str = "SELECT * FROM Category WHERE id = $1";
}

impl IdField for Topic {}
impl SelectAll for Topic {
    const SELECT_ALL: &'static str = "SELECT * FROM Topic";
}
impl SelectById for Topic {
    const SELECT_BY_ID: &'static str = "SELECT * FROM Topic WHERE id = $1";
}

impl IdField for Post {}
impl SelectAll for Post {
    const SELECT_ALL: &'static str = "SELECT * FROM Post";
}

impl IdField for User {}
impl SelectAll for User {
    const SELECT_ALL: &'static str = r#"SELECT * FROM "User""#;
}
impl SelectById for User {
    const SELECT_BY_ID: &'static str = r#"SELECT * FROM "User" where id = $1"#;
}
