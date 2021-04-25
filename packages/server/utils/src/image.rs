use super::font::get_default_font;
use chrono::Timelike;
use image::{
    codecs::{bmp::BmpEncoder, jpeg::JpegEncoder},
    imageops::FilterType,
    Bgr, ColorType, DynamicImage, GenericImage, GenericImageView, ImageBuffer, ImageError,
    ImageResult,
};
use imageproc::drawing::draw_text_mut;
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

fn copy_to_sub_image(
    image: &mut ImageBuffer<Bgr<u8>, Vec<u8>>,
    resized: &ImageBuffer<Bgr<u8>, Vec<u8>>,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
) {
    let mut sub_image_left = image.sub_image(x, y, width, height);

    for y in 0..height {
        for x in 0..width {
            let pixel = *resized.get_pixel(x, y);
            *sub_image_left.get_pixel_mut(x, y) = pixel;
        }
    }
}

fn timestamp_image(image: &mut ImageBuffer<Bgr<u8>, Vec<u8>>, x: u32, y: u32, height: f32) {
    let font = get_default_font();

    let scale = rusttype::Scale {
        x: height * 1.4,
        y: height,
    };

    let now = chrono::Local::now();
    let timestamp = now.format("%H:%M").to_string();

    let (image_width, _) = image.dimensions();

    let (x, y) = if now.second() > 30 {
        (x, y)
    } else {
        (image_width - x, y)
    };

    draw_text_mut(
        image,
        Bgr([83u8, 83u8, 83u8]),
        x,
        y,
        scale,
        &font,
        &timestamp,
    );
}

pub fn vr_transform(
    img: &DynamicImage,
    ndimensions: Dimensions,
    ipd: i16,
    scale: f32,
    alg: Option<FilterType>,
) -> ImageResult<DynamicImage> {
    let alg = alg.unwrap_or(FilterType::Triangle);
    let Dimensions { width, height } = ndimensions;

    let (scaled_width, scaled_height) = (
        ((width as f32 / 3 as f32) * scale) as u32,
        ((height as f32 / 3 as f32) * scale) as u32,
    );
    let (margin_top_bottom, margin_left_right) = (
        ((height - scaled_height) as f32 / 2 as f32) as u32,
        (((scaled_width as f32 / 3 as f32) as i32) - ipd as i32) as u32,
    );

    let mut image = ImageBuffer::<Bgr<u8>, Vec<u8>>::new(width, height);
    let resized = img
        .resize_exact(scaled_width, scaled_height, alg)
        .into_bgr8();

    copy_to_sub_image(
        &mut image,
        &resized,
        margin_left_right,
        margin_top_bottom,
        scaled_width,
        scaled_height,
    );
    copy_to_sub_image(
        &mut image,
        &resized,
        width - scaled_width - margin_left_right,
        margin_top_bottom,
        scaled_width,
        scaled_height,
    );

    let font_size = 50;
    let (timestamp_x, timestamp_y) = (margin_left_right * 3, margin_top_bottom - font_size * 2);
    timestamp_image(&mut image, timestamp_x, timestamp_y, font_size as f32);

    Ok(DynamicImage::ImageBgr8(image))
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
            "jpeg" | "jpg" => Ok(Self::Jpeg),
            "bmp" | "bitmap" => Ok(Self::Bmp),
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
        }
        ImageFormat::Bmp => {
            let mut encoder = BmpEncoder::new(&mut encoded);
            encoder.encode(rgb.as_raw(), width, height, ColorType::Rgb8)?;
            Ok(encoded)
        }
    }
}
