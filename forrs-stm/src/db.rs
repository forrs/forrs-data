use futures::FutureExt;
use serde::{Deserialize, Serialize};
use tokio_postgres::{types::ToSql, Config as PGConfig, Error, NoTls, ToStatement};

use crate::{id::Id, FromRow, IdField, Insert, SelectAll, SelectById};

/// Simple configuration struct for a [`Client`].
#[derive(Serialize, Deserialize)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub db_name: String,
    pub user: String,
    pub password: String,
}
impl From<&Config> for tokio_postgres::Config {
    fn from(s: &Config) -> tokio_postgres::Config {
        let mut c = tokio_postgres::Config::new();
        c.host(&s.host)
            .port(s.port)
            .dbname(&s.db_name)
            .user(&s.user)
            .password(&s.password);
        c
    }
}

/// A strongly typed wrapper around a [`tokio_postgres::Client`].
pub struct Client {
    inner: tokio_postgres::Client,
}

#[cfg(feature = "with-rocket")]
mod rocket {
    use crate::db::{Client, Config};
    use rocket::{
        http::Status,
        request::{FromRequestAsync, FromRequestFuture, Outcome, Request, State},
        try_outcome,
    };

    impl<'a, 'r> FromRequestAsync<'a, 'r> for &'a Client {
        type Error = String;

        fn from_request<'fut>(
            request: &'a Request<'r>,
        ) -> FromRequestFuture<'fut, Self, Self::Error>
        where
            'a: 'fut,
        {
            Box::pin(async move {
                let db_conf = try_outcome!(request.guard::<State<Config>>().map_failure(|_| (
                    Status::InternalServerError,
                    "No managed db::Config found!".to_string()
                )));

                match request.local_cache_async(Client::connect(&*db_conf)).await {
                    Ok(s) => Outcome::Success(s),
                    Err(e) => Outcome::Failure((Status::InternalServerError, e.to_string())),
                }
            })
        }
    }
}

impl Client {
    /// Attempts to connect to a Postgres server.
    /// On success, this spawns the Connection on the tokio-postgres threadpool
    /// and returns Self wrapping the obtained Client.
    pub async fn connect(conf: &Config) -> Result<Self, Error> {
        let (client, connection) = PGConfig::from(conf).connect(NoTls).await?;
        let connection = connection.map(|r| {
            if let Err(e) = r {
                eprintln!("connection error: {}", e);
            }
        });
        tokio::spawn(connection);
        Ok(Self { inner: client })
    }
    /// Creates a wrapper around an existing Client.
    pub async fn from_client(client: tokio_postgres::Client) -> Self {
        Self { inner: client }
    }

    /// Attempts to fetch a single item from the database.
    /// If no matching row was found, `Ok(None)` is returned.
    /// If multiple matching rows are found, the first one is returned.
    pub async fn fetch_item_opt<'a, T, S>(
        &'a self,
        stmt: &'a S,
        params: &'a [&'a (dyn ToSql + Sync)],
    ) -> Result<Option<T>, Error>
    where
        T: FromRow,
        S: ToStatement + ?Sized,
    {
        self.inner
            .query(stmt, params)
            .await?
            .first()
            .map(|row| T::from_row(&row))
            .transpose()
    }

    /// Attempts to fetch the item with the given Id from the database.
    /// If such an item doesn't exist, `Ok(None)` is returned.
    pub async fn fetch_item_by_id<T>(&self, id: Id) -> Result<Option<T>, Error>
    where
        T: FromRow + SelectById,
    {
        self.fetch_item_opt(T::SELECT_BY_ID, &[&id]).await
    }

    /// Attempts to fetch multiple items from the database.
    pub async fn fetch_items<'a, T, S>(
        &'a self,
        stmt: &'a S,
        params: &'a [&'a (dyn ToSql + Sync)],
    ) -> Result<Vec<T>, Error>
    where
        T: FromRow,
        S: ToStatement + ?Sized,
    {
        self.inner
            .query(stmt, params)
            .await?
            .iter()
            .map(|row| T::from_row(row))
            .collect()
    }

    /// Attempts to fetch all items from a table.
    pub async fn fetch_all_items<T>(&self) -> Result<Vec<T>, Error>
    where
        T: SelectAll + FromRow,
    {
        self.fetch_items(T::SELECT_ALL, &[]).await
    }

    /// Attempts to insert an item into the database.
    pub async fn insert<'a, T>(&'a self, item: &'a T) -> Result<Id, Error>
    where
        T: Insert + IdField,
    {
        self.inner
            .query(T::INSERT_STMT, &item.insert_params())
            .await
            .and_then(|rows| rows[0].try_get(T::ID_FIELD))
    }
}
