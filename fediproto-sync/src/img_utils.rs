use std::io::Cursor;

use fediproto_sync_lib::error::{FediProtoSyncError, FediProtoSyncErrorKind};
use image::{codecs::jpeg::JpegEncoder, ImageReader};

use crate::bsky::MAX_IMAGE_SIZE;

/// Compress an image using the JPEG format.
/// 
/// ## Arguments
/// 
/// * `image` - The image to compress.
pub fn compress_image_from_bytes(image: &[u8]) -> Result<bytes::Bytes, FediProtoSyncError> {
    tracing::info!("Decoding image for compression.");

    let image_reader = ImageReader::new(Cursor::new(image))
        .with_guessed_format()
        .map_err(|e| {
            FediProtoSyncError::with_source(
                "Failed to guess image format.",
                FediProtoSyncErrorKind::ImageCompressionError,
                Box::new(e)
            )
        })?
        .decode()
        .map_err(|e| {
            FediProtoSyncError::with_source(
                "Failed to decode image.",
                FediProtoSyncErrorKind::ImageCompressionError,
                Box::new(e)
            )
        })?;

    let image_reader = image_reader.into_rgb8();

    let mut image_buffer = vec![];
    let mut jpeg_encoder = JpegEncoder::new_with_quality(&mut image_buffer, 75);

    jpeg_encoder.encode_image(&image_reader).map_err(|e| {
        FediProtoSyncError::with_source(
            "Failed to encode image.",
            FediProtoSyncErrorKind::ImageCompressionError,
            Box::new(e)
        )
    })?;

    tracing::info!("Compressing image.");
    image_reader.write_with_encoder(jpeg_encoder).map_err(|e| {
        FediProtoSyncError::with_source(
            "Failed to compress image.",
            FediProtoSyncErrorKind::ImageCompressionError,
            Box::new(e)
        )
    })?;

    Ok(bytes::Bytes::from(image_buffer))
}

pub trait ImageCompressionUtils {
    fn compress_image(self) -> Result<Vec<u8>, FediProtoSyncError>;
}

impl<'a> ImageCompressionUtils for Vec<u8> {
    fn compress_image(self) -> Result<Self, FediProtoSyncError> {
        if self.len() > MAX_IMAGE_SIZE as usize {
            Ok(compress_image_from_bytes(&self)?.to_vec())
        }
        else {
            Ok(self)
        }
    }
}
