use std::io::Cursor;

use fediproto_sync_lib::error::FediProtoSyncError;
use image::{codecs::jpeg::JpegEncoder, DynamicImage, GenericImageView, ImageReader};

use crate::bsky::MAX_IMAGE_SIZE;

const MAX_IMAGE_WIDTH: u32 = 1080;

/// Compress an image using the JPEG format.
///
/// ## Arguments
///
/// * `image` - The image to compress.
pub fn compress_image_from_bytes(image: &[u8]) -> Result<bytes::Bytes, FediProtoSyncError> {
    tracing::info!("Decoding image for compression.");

    let image_reader = ImageReader::new(Cursor::new(image))
        .with_guessed_format()
        .map_err(|_| FediProtoSyncError::ImageCompressionError)?
        .decode()
        .map_err(|_| FediProtoSyncError::ImageCompressionError)?;

    let image_dimensions = image_reader.dimensions();

    // Resize the image if it's height is greater than `1080`.
    let image_reader = match image_dimensions.1 > MAX_IMAGE_WIDTH {
        true => {
            let new_height = MAX_IMAGE_WIDTH;
            let new_width = image_dimensions.0 * (new_height / image_dimensions.1);
            
            image::imageops::resize::<DynamicImage>(
                &image_reader,
                new_width,
                new_height,
                image::imageops::FilterType::Lanczos3
            )
        },

        false => image_reader.into_rgba8()
    };

    let mut image_buffer = vec![];
    let mut jpeg_encoder = JpegEncoder::new_with_quality(&mut image_buffer, 75);

    jpeg_encoder
        .encode_image(&image_reader)
        .map_err(|_| FediProtoSyncError::ImageCompressionError)?;

    tracing::info!("Compressing image.");
    image_reader
        .write_with_encoder(jpeg_encoder)
        .map_err(|_| FediProtoSyncError::ImageCompressionError)?;

    Ok(bytes::Bytes::from(image_buffer))
}

pub trait ImageCompressionUtils {
    fn compress_image(self) -> Result<Vec<u8>, FediProtoSyncError>;
}

impl<'a> ImageCompressionUtils for Vec<u8> {
    fn compress_image(self) -> Result<Self, FediProtoSyncError> {
        if self.len() > MAX_IMAGE_SIZE as usize {
            Ok(compress_image_from_bytes(&self)?.to_vec())
        } else {
            Ok(self)
        }
    }
}
