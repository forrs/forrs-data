use crate::model::User;
use mysql_async::{params, prelude::Queryable, BoxFuture, Conn, Stmt};

pub trait Insertable {
    fn params(&self) -> Vec<(String, mysql_common::value::Value)>;
    fn into_params(self) -> Vec<(String, mysql_common::value::Value)>;
    fn insert_stmt() -> &'static str;
    fn prepare_insert(conn: Conn) -> BoxFuture<Stmt<Conn>> {
        conn.prepare(Self::insert_stmt())
    }
}

impl Insertable for User {
    fn params(&self) -> Vec<(String, mysql_common::value::Value)> {
        params! {
            "name" => self.name.clone(),
            "pw_hash" => self.pw_hash.clone(),
            "registered_at" => self.registered_at
        }
    }
    fn into_params(self) -> Vec<(String, mysql_common::value::Value)> {
        params! {
            "name" => self.name,
            "pw_hash" => self.pw_hash,
            "registered_at" => self.registered_at
        }
    }
    fn insert_stmt() -> &'static str {
        "INSERT INTO User (name, pw_hash, registered_at)
            VALUES (:name, :pw_hash, :registered_at)"
    }
}
