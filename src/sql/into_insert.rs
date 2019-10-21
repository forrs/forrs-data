use crate::model::{Category, User};
use tokio_postgres::{types::ToSql, Client, Error, Statement};

pub trait IntoInsert: Sized {
    fn insert_stmt() -> &'static str;
    fn insert_params<'a>(&'a self) -> Vec<&'a (dyn ToSql + Sync)>;
}

impl IntoInsert for User {
    fn insert_stmt() -> &'static str {
        r#"INSERT INTO "User" (name, pw_hash) VALUES($1, $2)"#
    }
    fn insert_params<'a>(&'a self) -> Vec<&'a (dyn ToSql + Sync)> {
        vec![&self.name, &self.pw_hash]
    }
}

impl IntoInsert for Category {
    fn insert_stmt() -> &'static str {
        "INSERT INTO Category (name) VALUES($1)"
    }
    fn insert_params<'a>(&'a self) -> Vec<&'a (dyn ToSql + Sync)> {
        vec![&self.name]
    }
}
