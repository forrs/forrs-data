use tokio_postgres::{types::ToSql, Error, Row};

pub trait FromRow: Sized {
    fn from_row(row: &Row) -> Result<Self, Error>;
}

/// Defines how to insert an item.
pub trait Insert: Sized {
    /// The SQL statement used to insert an item as a new row.  
    /// **IMPORTANT NOTE**: forrs-stm expects this statement to return an `Id` value.
    const INSERT_STMT: &'static str;
    /// Returns the INSERT statement's parameters, which must match the
    /// indexed parameters in `INSERT_STMT`.
    fn insert_params<'a>(&'a self) -> Vec<&'a (dyn ToSql + Sync)>;
}

/// Defines how to select all items from a table.
pub trait SelectAll: Sized {
    const SELECT_ALL: &'static str;
}

/// Defines how to select an item by its id.
pub trait SelectById: Sized {
    const SELECT_BY_ID: &'static str;
}

/// Defines which field of an item is its id.
pub trait IdField: Sized {
    /// The database field containing the item's id.
    /// Defaults to `"id"` if omitted.
    const ID_FIELD: &'static str = "id";
}
