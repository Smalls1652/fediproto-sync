use crate::error::{FediProtoSyncError, FediProtoSyncErrorKind};

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
        .map_err(|e| {
            FediProtoSyncError::with_source(
                "Failed to encrypt string",
                FediProtoSyncErrorKind::EncryptionError,
                Box::new(e)
            )
        })?;

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

    let encrypted_string = openssl::base64::decode_block(input_string).map_err(|e| {
        FediProtoSyncError::with_source(
            "Failed to decode base64 string",
            FediProtoSyncErrorKind::DecryptionError,
            Box::new(e)
        )
    })?;

    let decrypted_length = private_key
        .private_decrypt(
            &encrypted_string,
            &mut decrypted_string,
            openssl::rsa::Padding::PKCS1
        )
        .map_err(|e| {
            FediProtoSyncError::with_source(
                "Failed to decrypt string",
                FediProtoSyncErrorKind::DecryptionError,
                Box::new(e)
            )
        })?;

    let decrypted_string = String::from_utf8(decrypted_string[..decrypted_length].to_vec())
        .map_err(|e| {
            FediProtoSyncError::with_source(
                "Failed to convert decrypted bytes to string",
                FediProtoSyncErrorKind::DecryptionError,
                Box::new(e)
            )
        })?;

    Ok(decrypted_string)
}
