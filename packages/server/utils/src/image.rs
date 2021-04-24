use image::{
    codecs::{bmp::BmpEncoder, jpeg::JpegEncoder},
    imageops::FilterType,
    Bgr, ColorType, DynamicImage, GenericImageView, ImageBuffer, ImageError, ImageResult,
};
use std::str::FromStr;

pub struct Dimensions<T = u32> {
    pub width: T,
    pub height: T,
}

pub fn bgra_to_image(bytes: &[u8], dimensions: Dimensions<usize>) -> DynamicImage {
    let (w, h) = (dimensions.width, dimensions.height);
    let stride = bytes.len() / h as usize;
    let mut img = ImageBuffer::<Bgr<u8>, Vec<u8>>::new(w as u32, h as u32);
    for y in 0..h {
        for x in 0..w {
            let i = stride * y + 4 * x;
            *img.get_pixel_mut(x as u32, y as u32) =
                image::Bgr([bytes[i], bytes[i + 1], bytes[i + 2]])
        }
    }
    DynamicImage::ImageBgr8(img)
}

pub fn vr_transform(
    img: &DynamicImage,
    ndimensions: Option<Dimensions>,
    alg: Option<FilterType>,
) -> ImageResult<DynamicImage> {
    let alg = alg.unwrap_or(FilterType::Lanczos3);
    let Dimensions { width, height } = ndimensions.unwrap_or(Dimensions {
        width: 1920,
        height: 1080,
    });
    let resized = img.resize(width, height, alg);
    Ok(resized)
}

#[derive(Debug, Clone, Copy)]
pub enum ImageFormat {
    Jpeg,
    Bmp,
}

impl FromStr for ImageFormat {
    type Err = String;
    fn from_str(format: &str) -> Result<Self, Self::Err> {
        let format = format.to_lowercase();
        let format = format.trim();
        match format {
            "jpeg" | "jpg" => Ok(ImageFormat::Jpeg),
            "bmp" | "bitmap" => Ok(ImageFormat::Bmp),
            _ => Err(format!("'{}' isn't a valid value for ImageFormat", format)),
        }
    }
}

pub fn encode_image(
    img: DynamicImage,
    format: ImageFormat,
    quality: u8,
) -> Result<Vec<u8>, ImageError> {
    let mut encoded = Vec::new();
    let (width, height) = (img.width(), img.height());
    let rgb = img.into_rgb8();

    match format {
        ImageFormat::Jpeg => {
            let mut encoder = JpegEncoder::new_with_quality(&mut encoded, quality);
            encoder.encode(rgb.as_raw(), width, height, ColorType::Rgb8)?;
            Ok(encoded)
        },
        ImageFormat::Bmp => {
            let mut encoder = BmpEncoder::new(&mut encoded);
            encoder.encode(rgb.as_raw(), width, height, ColorType::Rgb8)?;
            Ok(encoded)
        }
    }
}
