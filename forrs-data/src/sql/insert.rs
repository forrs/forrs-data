use crate::model::{Category, User};
use forrs_stm::Insert;
use tokio_postgres::types::ToSql;

impl Insert for User {
    const INSERT_STMT: &'static str =
        r#"INSERT INTO "User" (name, pw_hash) VALUES($1, $2) RETURNING id"#;
    fn insert_params<'a>(&'a self) -> Vec<&'a (dyn ToSql + Sync)> {
        vec![&self.name, &self.pw_hash]
    }
}

impl Insert for Category {
    const INSERT_STMT: &'static str = "INSERT INTO Category (name) VALUES($1) RETURNING id";
    fn insert_params<'a>(&'a self) -> Vec<&'a (dyn ToSql + Sync)> {
        vec![&self.name]
    }
}
