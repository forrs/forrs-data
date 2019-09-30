use crate::model::User;
use tokio_postgres::{impls::Prepare, types::ToSql, Client};

trait IntoInsert: Sized {
    fn into_insert<'a>(&'a self, conn: &mut Client) -> (Prepare, Vec<&'a dyn ToSql>);
}

impl IntoInsert for User {
    fn into_insert<'a>(&'a self, conn: &mut Client) -> (Prepare, Vec<&'a dyn ToSql>) {
        (
            conn.prepare(r#"INSERT INTO "User" (name, pw_hash) VALUES($1, $2)"#),
            vec![&self.name, &self.pw_hash],
        )
    }
}
