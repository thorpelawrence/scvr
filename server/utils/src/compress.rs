use flate2::{
    write::{DeflateEncoder, GzEncoder},
    Compression,
};
use std::io::prelude::*;
use std::io::Error;
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CompressionLevel {
    Fast,
    Default,
    Best,
    Custom(u8),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CompressionFormat {
    Gzip,
    Deflate,
    None,
}

pub fn compress(
    bytes: &[u8],
    compression_level: Option<CompressionLevel>,
    compression_format: Option<CompressionFormat>,
) -> Result<Vec<u8>, Error> {
    let compression_level = match compression_level {
        Some(CompressionLevel::Best) => Compression::best(),
        Some(CompressionLevel::Default) => Compression::default(),
        Some(CompressionLevel::Fast) | None => Compression::fast(),
        Some(CompressionLevel::Custom(level)) => Compression::new(level.into()),
    };

    if compression_format == Some(CompressionFormat::None) {
        return Ok(bytes.into());
    }

    if compression_format == Some(CompressionFormat::Deflate) {
        let mut encoder = DeflateEncoder::new(Vec::new(), compression_level);
        encoder.write_all(&bytes)?;
        let compressed_bytes = encoder.finish()?;
        return Ok(compressed_bytes);
    }

    let mut encoder = GzEncoder::new(Vec::new(), compression_level);
    encoder.write_all(&bytes)?;
    let compressed_bytes = encoder.finish()?;
    Ok(compressed_bytes)
}
