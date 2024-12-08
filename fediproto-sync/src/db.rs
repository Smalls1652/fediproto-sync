pub const POSTGRES_MIGRATIONS: diesel_migrations::EmbeddedMigrations =
    diesel_migrations::embed_migrations!("./migrations/postgres");

/// Run any pending database migrations.
///
/// ## Arguments
///
/// * `connection` - The database connection to run the migrations on.
pub fn run_postgres_migrations(
    connection: &mut impl diesel_migrations::MigrationHarness<diesel::pg::Pg>
) -> Result<(), crate::error::Error> {
    let pending_migrations = connection.pending_migrations(POSTGRES_MIGRATIONS)
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
