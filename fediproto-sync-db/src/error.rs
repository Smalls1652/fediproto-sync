use thiserror::Error;

#[derive(Error, Debug)]
pub enum FediProtoSyncDbError {
    /// An error occurred while trying to connect to the database.
    #[error("An error occurred while trying to connect to the database.")]
    DatabaseConnectionError(#[from] diesel::r2d2::Error),

    /// An error occurred while trying to run a database migration.
    #[error("An error occurred while trying to run a database migration.")]
    DatabaseMigrationError,

    /// An error occurred while running a database operation.
    #[error("An error occurred while running a database operation.")]
    DatabaseOperationError(#[from] diesel::result::Error)
}
