/// Core database functionality for the FediProto Sync project.
pub mod core;
/// Error types for FediProto Sync database operations.
pub mod error;
/// Database models for FediProto Sync.
pub mod models;
/// Database operations for FediProto Sync.
pub mod operations;
/// Database schema for FediProto Sync.
pub mod schema;
/// Type implementations for FediProto Sync.
pub mod type_impls;

/// PostgreSQL database schema for FediProto Sync.
#[cfg(feature = "local_dev")]
mod schema_postgres;
/// SQLite database schema for FediProto Sync.
#[cfg(feature = "local_dev")]
mod schema_sqlite;

use std::time::Duration;

use anyhow::{Context, Result};
use diesel::{
    backend::Backend,
    connection::Connection,
    deserialize::{self, FromSql},
    r2d2::{ConnectionManager, Pool},
    serialize::{self, IsNull, ToSql},
    sql_types::HasSqlType,
};
use type_impls::{MultiBackendUuid, UuidProxy};

/// A multi-backend enum to use with Diesel. Supports PostgreSQL and SQLite.
#[derive(diesel::MultiConnection)]
pub enum AnyConnection {
    /// A PostgreSQL connection.
    Postgres(diesel::PgConnection),

    /// A SQLite connection.
    SQLite(diesel::SqliteConnection),
}

pub fn create_database_connection(
    database_url: &str
) -> Result<Pool<ConnectionManager<AnyConnection>>> {
    tracing::debug!("Creating database connection pool.");
    let connection_manager = ConnectionManager::<AnyConnection>::new(database_url);

    tracing::debug!("Building database connection pool.");
    let pool = Pool::builder()
        .test_on_check_out(true)
        .connection_timeout(Duration::from_secs(15))
        .min_idle(Some(1))
        .max_size(10)
        .build(connection_manager)
        .context("Failed to create database connection pool.")?;

    Ok(pool)
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
        out: &mut serialize::Output<'b, '_, MultiBackend>,
    ) -> serialize::Result {
        out.set_value((MultiBackendUuid, self));
        Ok(IsNull::No)
    }
}
