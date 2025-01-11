use std::io::Cursor;

use anyhow::Result;
use image::{codecs::jpeg::JpegEncoder, DynamicImage, GenericImageView, ImageReader};

use crate::bsky::MAX_IMAGE_SIZE;

const MAX_IMAGE_PIXELS: u32 = 1080;

/// Compress an image using the JPEG format.
///
/// ## Arguments
///
/// * `image` - The image to compress.
pub fn compress_image_from_bytes(image: &[u8]) -> Result<bytes::Bytes> {
    tracing::info!("Decoding image for compression.");

    let image_reader = ImageReader::new(Cursor::new(image))
        .with_guessed_format()?
        .decode()?;

    let image_reader = resize_image(image_reader).into_rgb8();

    let mut image_buffer = vec![];
    let mut jpeg_encoder = JpegEncoder::new_with_quality(&mut image_buffer, 90);

    jpeg_encoder
        .encode_image(&image_reader)?;

    tracing::info!("Compressing image.");
    image_reader
        .write_with_encoder(jpeg_encoder)?;

    Ok(bytes::Bytes::from(image_buffer))
}

/// Get the aspect ratio of an image.
/// 
/// ## Arguments
/// 
/// * `image` - The image to get the aspect ratio of.
pub fn get_image_aspect_ratio(image: &[u8]) -> Result<(u32, u32)> {
    let image_reader = ImageReader::new(Cursor::new(image))
        .with_guessed_format()?
        .decode()?;

    let dimensions = image_reader.dimensions();
    let gcd = greatest_common_divisor(dimensions.0, dimensions.1);

    Ok((dimensions.0 / gcd, dimensions.1 / gcd))
}

pub trait ImageCompressionUtils {
    fn compress_image(self) -> Result<Vec<u8>>;
}

impl<'a> ImageCompressionUtils for Vec<u8> {
    fn compress_image(self) -> Result<Self> {
        if self.len() > MAX_IMAGE_SIZE as usize {
            Ok(compress_image_from_bytes(&self)?.to_vec())
        } else {
            Ok(self)
        }
    }
}

/// Resize an image to a maximum of 1080 pixels in either dimension.
/// 
/// ## Arguments
/// 
/// * `image` - The image to resize.
fn resize_image(image: DynamicImage) -> DynamicImage {
    let dimensions = image.dimensions();

    let is_height_greater_than_width = dimensions.1 > dimensions.0;

    let dimension_to_check = match is_height_greater_than_width {
        true => dimensions.1,
        false => dimensions.0
    };

    if dimension_to_check <= MAX_IMAGE_PIXELS {
        return image;
    }

    let new_height = match is_height_greater_than_width {
        true => MAX_IMAGE_PIXELS,
        false => {
            (MAX_IMAGE_PIXELS as f32 * (dimensions.1 as f32 / dimensions.0 as f32)).round() as u32
        }
    };

    let new_width = match is_height_greater_than_width {
        true => {
            (MAX_IMAGE_PIXELS as f32 * (dimensions.0 as f32 / dimensions.1 as f32)).round() as u32
        }
        false => MAX_IMAGE_PIXELS
    };

    tracing::info!(
        "Resizing image from {}x{} to {}x{}.",
        dimensions.0,
        dimensions.1,
        new_width,
        new_height
    );

    DynamicImage::ImageRgba8(image::imageops::resize::<DynamicImage>(
        &image,
        new_width,
        new_height,
        image::imageops::FilterType::Lanczos3
    ))
}

/// Get the greatest common divisor of two numbers.
/// 
/// ## Arguments
/// 
/// * `a` - The first number.
/// * `b` - The second number.
fn greatest_common_divisor(a: u32, b: u32) -> u32 {
    let (mut a, mut b) = match a > b {
        true => (a, b),
        false => (b, a)
    };

    while b != 0 {
        let temp = a;
        a = b;
        b = temp;

        b %= a;
    }

    a
}
