use fediproto_sync_lib::error::{FediProtoSyncError, FediProtoSyncErrorKind};

/// Embedded database migrations for PostgreSQL.
pub const POSTGRES_MIGRATIONS: diesel_migrations::EmbeddedMigrations =
    diesel_migrations::embed_migrations!("./migrations/postgres");

/// Embedded database migrations for SQLite.
pub const SQLITE_MIGRATIONS: diesel_migrations::EmbeddedMigrations =
    diesel_migrations::embed_migrations!("./migrations/sqlite");

/// Run any pending database migrations.
///
/// ## Arguments
///
/// * `connection` - The database connection to run the migrations on.
///
/// ## Note
///
/// This is the main entry point for running database migrations. It will
/// automatically determine the database backend and run the appropriate
/// migrations.
pub fn run_migrations(connection: &mut crate::AnyConnection) -> Result<(), FediProtoSyncError> {
    match connection {
        crate::AnyConnection::Postgres(connection) => {
            apply_migrations(connection, POSTGRES_MIGRATIONS)
        }
        crate::AnyConnection::SQLite(connection) => apply_migrations(connection, SQLITE_MIGRATIONS)
    }
}

/// Apply any migrations to the database.
///
/// ## Arguments
///
/// * `connection` - The database connection to run the migrations on.
/// * `migrations` - The embedded migrations, specific to the database backend,
///   to run.
///
/// ## Note
///
/// This function is a helper for `run_migrations` and should not be called
/// directly.
fn apply_migrations<T: diesel::backend::Backend + 'static>(
    connection: &mut impl diesel_migrations::MigrationHarness<T>,
    migrations: diesel_migrations::EmbeddedMigrations
) -> Result<(), FediProtoSyncError> {
    let pending_migrations = connection.pending_migrations(migrations).map_err(|e| {
        FediProtoSyncError::with_source(
            "Failed to get pending database migrations.",
            FediProtoSyncErrorKind::DatabaseMigrationError,
            e
        )
    })?;

    if pending_migrations.is_empty() {
        tracing::info!("No pending database migrations.");
        return Ok(());
    }

    tracing::info!(
        "Applying '{}' pending database migrations...",
        pending_migrations.len()
    );

    for migration_item in pending_migrations {
        connection.run_migration(&migration_item).map_err(|e| {
            FediProtoSyncError::with_source(
                "Failed to run database migration.",
                FediProtoSyncErrorKind::DatabaseMigrationError,
                e
            )
        })?;

        tracing::info!("Applied migration '{}'", migration_item.name());
    }

    tracing::info!("Applied all pending database migrations.");

    Ok(())
}
