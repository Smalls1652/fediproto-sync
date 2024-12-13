pub mod core;
pub mod models;
pub mod operations;
pub mod schema;
pub mod type_impls;

use diesel::{
    backend::Backend,
    connection::Connection,
    deserialize::{self, FromSql},
    serialize::{self, IsNull, ToSql},
    sql_types::HasSqlType,
    PgConnection,
    QueryResult,
    SqliteConnection
};
use fediproto_sync_lib::{
    config,
    error::{FediProtoSyncError, FediProtoSyncErrorKind}
};
use type_impls::{MultiBackendUuid, UuidProxy};

/// A multi-backend enum to use with Diesel. Supports PostgreSQL and SQLite.
#[derive(diesel::MultiConnection)]
pub enum AnyConnection {
    /// A PostgreSQL connection.
    Postgres(diesel::PgConnection),

    /// A SQLite connection.
    SQLite(diesel::SqliteConnection)
}

pub fn create_database_connection(
    database_url: &str,
    database_type: config::DatabaseType
) -> Result<AnyConnection, FediProtoSyncError> {
    let db_connection = match database_type {
        config::DatabaseType::Postgres => {
            AnyConnection::Postgres(PgConnection::establish(&database_url).map_err(|e| {
                FediProtoSyncError::with_source(
                    "Failed to connect to database.",
                    FediProtoSyncErrorKind::DatabaseConnectionError,
                    Box::new(e)
                )
            })?)
        }

        config::DatabaseType::SQLite => {
            AnyConnection::SQLite(SqliteConnection::establish(&database_url).map_err(|e| {
                FediProtoSyncError::with_source(
                    "Failed to connect to database.",
                    FediProtoSyncErrorKind::DatabaseConnectionError,
                    Box::new(e)
                )
            })?)
        }
    };

    Ok(db_connection)
}

// We have to implement `HasSqlType` for `MultiBackendUuid` since it's not a
// built-in Diesel type.
impl HasSqlType<MultiBackendUuid> for MultiBackend {
    fn metadata(lookup: &mut Self::MetadataLookup) -> Self::TypeMetadata {
        MultiBackend::lookup_sql_type::<MultiBackendUuid>(lookup)
    }
}

impl FromSql<MultiBackendUuid, MultiBackend> for UuidProxy {
    /// Parse a `uuid::Uuid` from a `MultiBackendUuid` column.
    ///
    /// ## Arguments
    ///
    /// * `bytes` - The raw value of the column.
    fn from_sql(bytes: <MultiBackend as Backend>::RawValue<'_>) -> deserialize::Result<Self> {
        bytes.from_sql::<UuidProxy, MultiBackendUuid>()
    }
}

impl ToSql<MultiBackendUuid, MultiBackend> for UuidProxy {
    /// Serialize a `uuid::Uuid` to a `MultiBackendUuid` column.
    ///
    /// ## Arguments
    ///
    /// * `out` - The output buffer to write the serialized value to.
    fn to_sql<'b>(
        &'b self,
        out: &mut serialize::Output<'b, '_, MultiBackend>
    ) -> serialize::Result {
        out.set_value((MultiBackendUuid, self));
        Ok(IsNull::No)
    }
}
