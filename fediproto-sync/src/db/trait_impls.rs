use diesel::{
    deserialize::{self, FromSql, FromSqlRow}, query_builder::QueryId, serialize::{self, IsNull, ToSql}, sql_types::*, AsExpression
};

#[derive(Copy, Clone, Debug, SqlType, QueryId)]
#[diesel(postgres_type(name = "uuid"))]
#[diesel(sqlite_type(name = "Text"))]
pub struct MultiBackendUuid;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, FromSqlRow, AsExpression)]
#[diesel(sql_type = MultiBackendUuid)]
pub struct UuidProxy(pub uuid::Uuid);

impl FromSql<MultiBackendUuid, diesel::sqlite::Sqlite> for UuidProxy {
    fn from_sql(
        bytes: <diesel::sqlite::Sqlite as diesel::backend::Backend>::RawValue<'_>
    ) -> deserialize::Result<Self> {
        let from_sql_value = <String as FromSql<Text, diesel::sqlite::Sqlite>>::from_sql(bytes)?;
        let uuid = uuid::Uuid::parse_str(&from_sql_value)?;

        Ok(UuidProxy(uuid))
    }
}

impl ToSql<MultiBackendUuid, diesel::sqlite::Sqlite> for UuidProxy {
    fn to_sql<'b>(
        &'b self,
        out: &mut serialize::Output<'b, '_, diesel::sqlite::Sqlite>
    ) -> serialize::Result {
        out.set_value(self.0.to_string());
        Ok(IsNull::No)
    }
}

impl FromSql<MultiBackendUuid, diesel::pg::Pg> for UuidProxy {
    fn from_sql(value: diesel::pg::PgValue<'_>) -> deserialize::Result<Self> {
        let uuid = <uuid::Uuid as FromSql<Uuid, diesel::pg::Pg>>::from_sql(value)?;

        Ok(UuidProxy(uuid))
    }
}

impl ToSql<MultiBackendUuid, diesel::pg::Pg> for UuidProxy {
    fn to_sql<'b>(&'b self, out: &mut diesel::serialize::Output<'b, '_, diesel::pg::Pg>) -> serialize::Result {
        <uuid::Uuid as ToSql<Uuid, diesel::pg::Pg>>::to_sql(&self.0, out)?;

        Ok(IsNull::No)
    }
}
