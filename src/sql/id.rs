use crate::model::id::*;
use std::error::Error;
use tokio_postgres::to_sql_checked;
use tokio_postgres::types::{FromSql, IsNull, ToSql, Type};

impl<'a> FromSql<'a> for Id {
    fn from_sql(ty: &Type, raw: &'a [u8]) -> Result<Self, Box<dyn Error + Sync + Send>> {
        let value = <Inner as FromSql>::from_sql(ty, raw)?;
        Ok(Self(value))
    }
    fn accepts(ty: &Type) -> bool {
        <Inner as FromSql>::accepts(ty)
    }
}

impl ToSql for Id {
    fn to_sql(&self, ty: &Type, out: &mut Vec<u8>) -> Result<IsNull, Box<dyn Error + Sync + Send>> {
        <Inner as ToSql>::to_sql(&self.0, ty, out)
    }
    fn accepts(ty: &Type) -> bool {
        <Inner as ToSql>::accepts(ty)
    }
    to_sql_checked!();
}
