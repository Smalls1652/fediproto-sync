use anyhow::Result;

/// CLI options for FediProtoSync.
enum CliOption {
    /// Generate a token encryption key for the application.
    GenerateTokenEncryptionKey,
}

impl std::str::FromStr for CliOption {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "generate-token-encryption-key" => Ok(CliOption::GenerateTokenEncryptionKey),
            _ => Err(anyhow::anyhow!("Invalid CLI option"))
        }
    }
}

/// Handle CLI options for the application.
pub fn handle_cli_options(args: Vec<String>) -> Result<()> {
    let cli_option = args[1].as_str().parse::<CliOption>()?;

    match cli_option {
        CliOption::GenerateTokenEncryptionKey => {
            let encryption_keys = fediproto_sync_lib::crypto::generate_token_encryption_key()?;

            println!("{}", encryption_keys);

            std::process::exit(0);
        }
    };
}
