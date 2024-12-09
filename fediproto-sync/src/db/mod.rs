pub mod core;
pub mod models;
pub mod trait_impls;

use diesel::{
    backend::Backend,
    deserialize::{self, FromSql},
    serialize::{self, IsNull, ToSql},
    sql_types::HasSqlType,
    QueryResult
};
use trait_impls::{MultiBackendUuid, UuidProxy};

#[derive(diesel::MultiConnection)]
pub enum AnyConnection {
    Postgres(diesel::PgConnection),
    SQLite(diesel::SqliteConnection)
}

impl HasSqlType<MultiBackendUuid> for MultiBackend {
    fn metadata(lookup: &mut Self::MetadataLookup) -> Self::TypeMetadata {
        MultiBackend::lookup_sql_type::<MultiBackendUuid>(lookup)
    }
}

impl FromSql<MultiBackendUuid, MultiBackend> for UuidProxy {
    fn from_sql(bytes: <MultiBackend as Backend>::RawValue<'_>) -> deserialize::Result<Self> {
        bytes.from_sql::<UuidProxy, MultiBackendUuid>()
    }
}

impl ToSql<MultiBackendUuid, MultiBackend> for UuidProxy {
    fn to_sql<'b>(
        &'b self,
        out: &mut serialize::Output<'b, '_, MultiBackend>
    ) -> serialize::Result {
        out.set_value((MultiBackendUuid, self));
        Ok(IsNull::No)
    }
}
