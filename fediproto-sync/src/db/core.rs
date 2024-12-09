pub const POSTGRES_MIGRATIONS: diesel_migrations::EmbeddedMigrations =
    diesel_migrations::embed_migrations!("./migrations/postgres");
pub const SQLITE_MIGRATIONS: diesel_migrations::EmbeddedMigrations =
    diesel_migrations::embed_migrations!("./migrations/sqlite");

pub fn run_migrations(
    connection: &mut crate::db::AnyConnection
) -> Result<(), crate::error::Error> {
    match connection {
        crate::db::AnyConnection::Postgres(connection) => {
            run_migrations_backend(connection, POSTGRES_MIGRATIONS)
        }
        crate::db::AnyConnection::SQLite(connection) => {
            run_migrations_backend(connection, SQLITE_MIGRATIONS)
        }
    }
}

/// Run any pending database migrations for Postgres.
///
/// ## Arguments
///
/// * `connection` - The database connection to run the migrations on.
fn run_migrations_backend<T: diesel::backend::Backend + 'static>(
    connection: &mut impl diesel_migrations::MigrationHarness<T>,
    migrations: diesel_migrations::EmbeddedMigrations
) -> Result<(), crate::error::Error> {
    let pending_migrations = connection.pending_migrations(migrations)
        .map_err(|e| crate::error::Error::with_source(
            "Failed to get pending database migrations.",
            crate::error::ErrorKind::DatabaseMigrationError,
            e
        ))?;

    if pending_migrations.is_empty() {
        tracing::info!("No pending database migrations.");
        return Ok(());
    }

    tracing::info!(
        "Applying '{}' pending database migrations...",
        pending_migrations.len()
    );

    for migration_item in pending_migrations {
        connection.run_migration(&migration_item)
            .map_err(|e| crate::error::Error::with_source(
                "Failed to run database migration.",
                crate::error::ErrorKind::DatabaseMigrationError,
                e
            ))?;

        tracing::info!("Applied migration '{}'", migration_item.name());
    }

    tracing::info!("Applied all pending database migrations.");

    Ok(())
}
