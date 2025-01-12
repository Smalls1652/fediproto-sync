use diesel::{
    AsExpression,
    deserialize::{self, FromSql, FromSqlRow},
    query_builder::QueryId,
    serialize::{self, IsNull, ToSql},
    sql_types::*
};

/// Custom type for Diesel to handle UUIDs in both SQLite and PostgreSQL.
#[derive(Copy, Clone, Debug, SqlType, QueryId)]
#[diesel(postgres_type(name = "uuid"))]
#[diesel(sqlite_type(name = "Text"))]
pub struct MultiBackendUuid;

/// Proxy type for `uuid::Uuid` to implement `FromSql` and `ToSql` for use with
/// Diesel.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, FromSqlRow, AsExpression)]
#[diesel(sql_type = MultiBackendUuid)]
pub struct UuidProxy(pub uuid::Uuid);

// `UuidProxy` - SQLite `FromSql` and `ToSql` implementations.

impl FromSql<MultiBackendUuid, diesel::sqlite::Sqlite> for UuidProxy {
    /// Parse a `uuid::Uuid` from a SQLite `Text` column.
    ///
    /// ## Arguments
    ///
    /// * `bytes` - The raw value of the column.
    fn from_sql(
        bytes: <diesel::sqlite::Sqlite as diesel::backend::Backend>::RawValue<'_>
    ) -> deserialize::Result<Self> {
        let from_sql_value = <String as FromSql<Text, diesel::sqlite::Sqlite>>::from_sql(bytes)?;
        let uuid = uuid::Uuid::parse_str(&from_sql_value)?;

        Ok(UuidProxy(uuid))
    }
}

impl ToSql<MultiBackendUuid, diesel::sqlite::Sqlite> for UuidProxy {
    /// Serialize a `uuid::Uuid` to a SQLite `Text` column.
    ///
    /// ## Arguments
    ///
    /// * `out` - The output buffer to write the serialized value to.
    fn to_sql<'b>(
        &'b self,
        out: &mut serialize::Output<'b, '_, diesel::sqlite::Sqlite>
    ) -> serialize::Result {
        out.set_value(self.0.to_string());
        Ok(IsNull::No)
    }
}

// `UuidProxy` - PostgreSQL `FromSql` and `ToSql` implementations.

impl FromSql<MultiBackendUuid, diesel::pg::Pg> for UuidProxy {
    /// Parse a `uuid::Uuid` from a PostgreSQL `uuid` column.
    ///
    /// ## Arguments
    ///
    /// * `value` - The raw value of the column.
    ///
    /// ## Note
    ///
    /// This implementation leverages the existing `FromSql` implementation for
    /// `uuid::Uuid` for the PostgreSQL backend.
    fn from_sql(value: diesel::pg::PgValue<'_>) -> deserialize::Result<Self> {
        let uuid = <uuid::Uuid as FromSql<Uuid, diesel::pg::Pg>>::from_sql(value)?;

        Ok(UuidProxy(uuid))
    }
}

impl ToSql<MultiBackendUuid, diesel::pg::Pg> for UuidProxy {
    /// Serialize a `uuid::Uuid` to a PostgreSQL `uuid` column.
    ///
    /// ## Arguments
    ///
    /// * `out` - The output buffer to write the serialized value to.
    ///
    /// ## Note
    ///
    /// This implementation leverages the existing `ToSql` implementation for
    /// `uuid::Uuid` for the PostgreSQL backend.
    fn to_sql<'b>(
        &'b self,
        out: &mut diesel::serialize::Output<'b, '_, diesel::pg::Pg>
    ) -> serialize::Result {
        <uuid::Uuid as ToSql<Uuid, diesel::pg::Pg>>::to_sql(&self.0, out)?;

        Ok(IsNull::No)
    }
}
