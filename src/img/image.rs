use actix_web::web::Bytes;
use magick_rust::MagickWand;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};

pub struct Image {
    wand: MagickWand,
}

impl Image {
    pub fn from_bytes(bytes: &Bytes) -> Result<Self, ImageError> {
        let wand = MagickWand::new();
        match wand.read_image_blob(bytes) {
            Ok(_) => Ok(Self { wand }),
            Err(_) => Err(ImageError::InvalidImage),
        }
    }

    pub fn resize(&self, width: usize, height: usize) {
        // this could be improved with access to imagemagick's thumbnail re-sizing
        self.wand.fit(width, height);
    }

    pub fn to_buffer(&mut self, quality: u8) -> Result<Vec<u8>, ImageError> {
        let format = self.format()?;

        self.wand
            .set_image_compression_quality(quality as usize)
            .map_err(|_| ImageError::FailedWrite)?;

        self.wand
            .write_image_blob(&format)
            .map_err(|_| ImageError::FailedWrite)
    }

    pub fn format(&self) -> Result<String, ImageError> {
        self.wand
            .get_image_format()
            .map_err(|_| ImageError::InvalidFormat)
    }

    pub fn mime_type(self) -> Result<&'static str, ImageError> {
        let format = self.format()?;

        match &format[..] {
            "APNG" => Ok("image/apng"),
            "AVIF" => Ok("image/avif"),
            "GIF" => Ok("image/gif"),
            "JPEG" => Ok("image/jpeg"),
            "PNG" => Ok("image/png"),
            "SVG" => Ok("image/svg+xml"),
            "WEBP" => Ok("image/webp"),
            _ => Ok("application/octet-stream"),
        }
    }
}

pub enum ImageError {
    InvalidImage,
    InvalidFormat,
    FailedWrite,
}

impl ImageError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidImage => "Invalid Image",
            Self::InvalidFormat => "Invalid Format For Image",
            Self::FailedWrite => "Failed To Write Image",
        }
    }
}

impl Display for ImageError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Debug for ImageError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())
    }
}