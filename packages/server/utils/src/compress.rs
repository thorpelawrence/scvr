use flate2::{Compression, write::{DeflateEncoder, GzEncoder, ZlibEncoder}};
use std::{
    io::{prelude::*, Error},
    num::ParseIntError,
    str::FromStr,
};
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CompressionLevel {
    Fast,
    Default,
    Best,
    Custom(u8),
}

impl FromStr for CompressionLevel {
    type Err = ParseIntError;
    fn from_str(level: &str) -> Result<Self, Self::Err> {
        let level = level.parse::<u8>()?;
        Ok(CompressionLevel::Custom(level))
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CompressionFormat {
    Gzip,
    Deflate,
    Zlib,
    None,
}

impl FromStr for CompressionFormat {
    type Err = String;
    fn from_str(format: &str) -> Result<Self, Self::Err> {
        let format = format.to_lowercase();
        let format = format.trim();
        match format {
            "gzip" | "gz" => Ok(Self::Gzip),
            "deflate" => Ok(Self::Deflate),
            "zlib" => Ok(Self::Zlib),
            "none" => Ok(Self::None),
            _ => Err(format!(
                "'{}' isn't a valid value for CompressionFormat",
                format
            )),
        }
    }
}

pub fn compress(
    bytes: &[u8],
    compression_level: Option<CompressionLevel>,
    compression_format: CompressionFormat,
) -> Result<Vec<u8>, Error> {
    let compression_level = match compression_level {
        Some(CompressionLevel::Best) => Compression::best(),
        Some(CompressionLevel::Default) => Compression::default(),
        Some(CompressionLevel::Fast) | None => Compression::fast(),
        Some(CompressionLevel::Custom(level)) => Compression::new(level.into()),
    };

    Ok(match compression_format {
        CompressionFormat::None => bytes.into(),
        CompressionFormat::Deflate => {
            let mut encoder = DeflateEncoder::new(Vec::new(), compression_level);
            encoder.write_all(&bytes)?;
            let compressed_bytes = encoder.finish()?;
            compressed_bytes
        }
        CompressionFormat::Zlib => {
            let mut encoder = ZlibEncoder::new(Vec::new(), compression_level);
            encoder.write_all(&bytes)?;
            let compressed_bytes = encoder.finish()?;
            compressed_bytes
        }
        CompressionFormat::Gzip => {
            let mut encoder = GzEncoder::new(Vec::new(), compression_level);
            encoder.write_all(&bytes)?;
            let compressed_bytes = encoder.finish()?;
            compressed_bytes
        }
    })
}
