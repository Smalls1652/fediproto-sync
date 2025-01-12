use crate::error::FediProtoSyncError;

/// Encrypts a string using the provided public key.
///
/// ## Arguments
///
/// * `public_key` - The public key to use for encryption.
pub fn encrypt_string(
    public_key: &openssl::rsa::Rsa<openssl::pkey::Public>,
    input_string: &str
) -> Result<String, FediProtoSyncError> {
    let mut encrypted_string = vec![0; public_key.size() as usize];

    public_key
        .public_encrypt(
            input_string.as_bytes(),
            &mut encrypted_string,
            openssl::rsa::Padding::PKCS1
        )
        .map_err(|_| FediProtoSyncError::EncryptionError)?;

    let encoded_string = openssl::base64::encode_block(&encrypted_string);

    Ok(encoded_string)
}

/// Decrypts a string using the provided private key.
///
/// ## Arguments
///
/// * `private_key` - The private key to use for decryption.
pub fn decrypt_string(
    private_key: &openssl::rsa::Rsa<openssl::pkey::Private>,
    input_string: &str
) -> Result<String, FediProtoSyncError> {
    let mut decrypted_string = vec![0; private_key.size() as usize];

    let encrypted_string = openssl::base64::decode_block(input_string)
        .map_err(|_| FediProtoSyncError::DecryptionError)?;

    let decrypted_length = private_key
        .private_decrypt(
            &encrypted_string,
            &mut decrypted_string,
            openssl::rsa::Padding::PKCS1
        )
        .map_err(|_| FediProtoSyncError::DecryptionError)?;

    let decrypted_string = String::from_utf8(decrypted_string[..decrypted_length].to_vec())
        .map_err(|_| FediProtoSyncError::DecryptionError)?;

    Ok(decrypted_string)
}

/// Generate a new token encryption key pair.
pub fn generate_token_encryption_key() -> Result<String, FediProtoSyncError> {
    let private_key =
        openssl::rsa::Rsa::generate(2048).map_err(|_| FediProtoSyncError::KeyGenerationError)?;

    let private_key_bytes = private_key
        .private_key_to_pem()
        .map_err(|_| FediProtoSyncError::KeyGenerationError)?;
    let private_key_base64 = openssl::base64::encode_block(&private_key_bytes);

    let public_key_bytes = private_key
        .public_key_to_pem()
        .map_err(|_| FediProtoSyncError::KeyGenerationError)?;
    let public_key_base64 = openssl::base64::encode_block(&public_key_bytes);

    let output_string = format!(
        "TOKEN_ENCRYPTION_PRIVATE_KEY=\"{}\"\nTOKEN_ENCRYPTION_PUBLIC_KEY=\"{}\"",
        private_key_base64, public_key_base64
    );

    Ok(output_string)
}
