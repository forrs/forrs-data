use crate::model::{User, Category};
use tokio_postgres::{types::ToSql, Statement, Error, Client};

#[allow(clippy::needless_lifetimes)]
pub async fn into_insert<'a, T: IntoInsert>(
    item: &'a T,
    client: &mut Client
) -> Result<(Statement, Vec<&'a dyn ToSql>), Error> {
    let params = item.insert_params();
    let stmt = client.prepare(T::insert_stmt()).await?;
    Ok((stmt, params))
}

pub trait IntoInsert: Sized {
    fn insert_stmt() -> &'static str;
    fn insert_params<'a>(&'a self) -> Vec<&'a dyn ToSql>;
}

impl IntoInsert for User {
    fn insert_stmt() -> &'static str {
        r#"INSERT INTO "User" (name, pw_hash) VALUES($1, $2)"#
    }
    fn insert_params<'a>(&'a self) -> Vec<&'a dyn ToSql> {
        vec![&self.name, &self.pw_hash]
    }
}

impl IntoInsert for Category {
    fn insert_stmt() -> &'static str {
        "INSERT INTO Category (name) VALUES($1)"
    }
    fn insert_params<'a>(&'a self) -> Vec<&'a dyn ToSql> {
        vec![&self.name]
    }
}