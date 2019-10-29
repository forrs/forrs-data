use crate::model::id::*;
use bytes::BytesMut;
use std::error::Error;
use tokio_postgres::types::{to_sql_checked, FromSql, IsNull, ToSql, Type};

impl From<Signed> for Id {
    fn from(v: Signed) -> Id {
        let inner = if v < 0 {
            (v + Signed::max_value() + 1) as Inner
        } else {
            (v as Inner) + Signed::max_value() as Inner + 1
        };
        Id(inner)
    }
}
impl From<Id> for Signed {
    fn from(id: Id) -> Signed {
        if id.0 > Signed::max_value() as Inner {
            (id.0 - (Signed::max_value() as Inner) - 1) as Signed
        } else {
            id.0 as Signed - Signed::max_value() - 1
        }
    }
}

impl<'a> FromSql<'a> for Id {
    fn from_sql(ty: &Type, raw: &'a [u8]) -> Result<Self, Box<dyn Error + Sync + Send>> {
        let actual = <Signed as FromSql>::from_sql(ty, raw)?;
        Ok(Self::from(actual))
    }
    fn accepts(ty: &Type) -> bool {
        <Signed as FromSql>::accepts(ty)
    }
}

impl ToSql for Id {
    fn to_sql(
        &self,
        ty: &Type,
        out: &mut BytesMut,
    ) -> Result<IsNull, Box<dyn Error + Sync + Send>> {
        <Signed as ToSql>::to_sql(&Signed::from(*self), ty, out)
    }
    fn accepts(ty: &Type) -> bool {
        <Signed as ToSql>::accepts(ty)
    }
    to_sql_checked!();
}

#[cfg(test)]
mod tests {
    use crate::model::id::*;
    #[test]
    fn correct_display() {
        assert_eq!(
            format!("{}", Id::from(Signed::min_value())),
            "0".to_string()
        );
        assert_eq!(
            format!("{}", Id::from(0 as Signed)),
            format!("{}", Signed::max_value() as Inner + 1)
        );
        assert_eq!(
            format!("{}", Id::from(Signed::max_value())),
            format!("{}", Inner::max_value())
        );
    }
    #[test]
    fn correct_from_id() {
        assert_eq!(Signed::from(Id(0)), Signed::min_value());
        assert_eq!(Signed::from(Id(Inner::max_value())), Signed::max_value());
        assert_eq!(Signed::from(Id(Signed::max_value() as Inner)), -1);
        assert_eq!(Signed::from(Id(Signed::max_value() as Inner + 1)), 0);
        assert_eq!(Signed::from(Id(Signed::max_value() as Inner + 2)), 1);
    }
    #[test]
    fn corrent_from_inner() {
        assert_eq!(Id::from(Signed::min_value()).0, 0);
        assert_eq!(Id::from(Signed::max_value()).0, Inner::max_value());
        assert_eq!(Id::from(-1 as Signed).0, Signed::max_value() as Inner);
        assert_eq!(Id::from(0 as Signed).0, Signed::max_value() as Inner + 1);
        assert_eq!(Id::from(1 as Signed).0, Signed::max_value() as Inner + 2);
    }
}
