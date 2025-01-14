use std::{io::Cursor, str::FromStr};

use anyhow::Result;
use fediproto_sync_lib::error::FediProtoSyncError;
use image::{
    DynamicImage,
    GenericImageView,
    ImageReader,
    codecs::{
        bmp::BmpDecoder,
        jpeg::{JpegDecoder, JpegEncoder},
        png::PngDecoder,
        webp::WebPDecoder
    },
    imageops::FilterType
};
use once_cell::sync::Lazy;
use regex::Regex;

use crate::bsky::MAX_IMAGE_SIZE;

/// The max height/width, in pixels, for a resized image.
const MAX_IMAGE_PIXELS: u32 = 1080;

/// Regex for parsing the URL for an image.
static IMAGE_URL_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^(?:https|http)://(.+?)/(.+)/(?P<fileName>.+(?P<fileExtension>\..+))$").unwrap()
});

/// The format of the image.
#[derive(Debug)]
enum ImageFormatType {
    /// JPEG
    Jpeg,

    /// PNG
    Png,

    /// WebP
    WebP,

    /// BMP
    Bmp,

    /// An unknown/unsupported format.
    Unknown
}

impl std::str::FromStr for ImageFormatType {
    type Err = FediProtoSyncError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let format = s.to_lowercase();
        match format.as_str() {
            ".jpeg" => Ok(Self::Jpeg),
            ".jpg" => Ok(Self::Jpeg),
            ".png" => Ok(Self::Png),
            ".webp" => Ok(Self::WebP),
            ".bmp" => Ok(Self::Bmp),
            _ => Ok(Self::Unknown)
        }
    }
}

/// Data representing an image attachment to be used during the sync.
pub struct ImageAttachmentData {
    /// The image's data represented as bytes.
    pub image_bytes: bytes::Bytes,

    /// The calculated aspect ration width.
    pub aspect_ratio_width: u32,

    /// The calculated aspect ratio height.
    pub aspect_ratio_height: u32
}

impl ImageAttachmentData {
    /// Create a new image attachment data.
    ///
    /// ## Arguments
    ///
    /// * `data` - The image data.
    /// * `url` - The URL of the image.
    pub fn new(
        data: bytes::Bytes,
        url: &str
    ) -> Result<Self> {
        let url_captures = IMAGE_URL_REGEX.captures(url);

        let image_format = match url_captures {
            Some(capture) => {
                let file_extension = capture.name("fileExtension").unwrap().as_str();
                tracing::info!("File extension: {}", file_extension);
                ImageFormatType::from_str(file_extension)?
            }

            _ => ImageFormatType::Unknown
        };

        tracing::info!("Image format: {:?}", image_format);

        let data_reader = Cursor::new(&data);

        let image = decode_image(data_reader, &image_format)?;

        let (aspect_ratio_width, aspect_ratio_height) = get_aspect_ratio(&image)?;

        let image_bytes = match data.len() > MAX_IMAGE_SIZE as usize {
            true => {
                tracing::info!("Original size: {} bytes.", data.len());
                convert_to_jpeg(image)?
            }
            false => {
                tracing::info!("Image does not need to be compressed.");
                data
            }
        };

        Ok(Self {
            image_bytes,
            aspect_ratio_width,
            aspect_ratio_height
        })
    }
}

/// Decode an image.
///
/// ## Arguments
///
/// * `data_reader` - The data reader.
/// * `image_format` - The image format.
fn decode_image(
    mut data_reader: Cursor<&bytes::Bytes>,
    image_format: &ImageFormatType
) -> Result<DynamicImage> {
    let image = match image_format {
        ImageFormatType::Jpeg => {
            tracing::info!("Decoding JPEG image.");
            let decoded_image = JpegDecoder::new(&mut data_reader)?;

            DynamicImage::from_decoder(decoded_image)?
        }

        ImageFormatType::Png => {
            tracing::info!("Decoding PNG image.");
            let decoded_image = PngDecoder::new(&mut data_reader)?;

            DynamicImage::from_decoder(decoded_image)?
        }

        ImageFormatType::WebP => {
            tracing::info!("Decoding WebP image.");
            let decoded_image = WebPDecoder::new(&mut data_reader)?;

            DynamicImage::from_decoder(decoded_image)?
        }

        ImageFormatType::Bmp => {
            tracing::info!("Decoding BMP image.");
            let decoded_image = BmpDecoder::new(&mut data_reader)?;

            DynamicImage::from_decoder(decoded_image)?
        }

        ImageFormatType::Unknown => {
            tracing::info!("Decoding image with an unknown format.");

            ImageReader::new(&mut data_reader)
                .with_guessed_format()?
                .decode()?
        }
    };

    tracing::info!("Color type: {:?}", image.color());

    Ok(image)
}

/// Convert an image to a JPEG.
///
/// ## Arguments
///
/// * `image` - The byte slice of the image.
fn convert_to_jpeg(decoded_image: DynamicImage) -> Result<bytes::Bytes> {
    let decoded_image = decoded_image.resize_image();

    let mut image_buffer = vec![];
    let mut jpeg_encoder = JpegEncoder::new_with_quality(&mut image_buffer, 80);

    tracing::info!("Encoding image.");
    jpeg_encoder.encode_image(&decoded_image)?;

    Ok(bytes::Bytes::from(image_buffer))
}

/// Utilities for interacting with images.
trait ImageUtils {
    /// Resize an image to a maximum of 1080 pixels in either dimension.
    fn resize_image(self) -> Self;
}

impl<'a> ImageUtils for DynamicImage {
    /// Resize an image to a maximum of 1080 pixels in either dimension.
    fn resize_image(self) -> DynamicImage {
        let dimensions = self.dimensions();

        let is_height_greater_than_width = dimensions.1 > dimensions.0;

        let dimension_to_check = match is_height_greater_than_width {
            true => dimensions.1,
            false => dimensions.0
        };

        if dimension_to_check <= MAX_IMAGE_PIXELS {
            return self;
        }

        let new_height = match is_height_greater_than_width {
            true => MAX_IMAGE_PIXELS,
            false => (MAX_IMAGE_PIXELS as f32 * (dimensions.1 as f32 / dimensions.0 as f32)).round()
                as u32
        };

        let new_width = match is_height_greater_than_width {
            true => (MAX_IMAGE_PIXELS as f32 * (dimensions.0 as f32 / dimensions.1 as f32)).round()
                as u32,
            false => MAX_IMAGE_PIXELS
        };

        tracing::info!(
            "Resizing image from {}x{} to {}x{}.",
            dimensions.0,
            dimensions.1,
            new_width,
            new_height
        );

        return self.resize(new_width, new_height, FilterType::Lanczos3);
    }
}

/// Get the aspect ratio of an image.
fn get_aspect_ratio(image: &DynamicImage) -> Result<(u32, u32)> {
    let dimensions = image.dimensions();
    let gcd = greatest_common_divisor(dimensions.0, dimensions.1);

    Ok((dimensions.0 / gcd, dimensions.1 / gcd))
}

/// Get the greatest common divisor of two numbers.
///
/// ## Arguments
///
/// * `a` - The first number.
/// * `b` - The second number.
fn greatest_common_divisor(
    a: u32,
    b: u32
) -> u32 {
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
