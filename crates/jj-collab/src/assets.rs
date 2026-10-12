use crate::*;
use image::{AnimationDecoder, ImageDecoder, ImageFormat, ImageReader, Limits, RgbaImage};
use jj_store_model::PrincipalId;
use serde::{Deserialize, Serialize};
use std::io::Cursor;

pub const MAX_IMAGE_BYTES: usize = 2 * 1024 * 1024;
pub const MAX_IMAGE_DIMENSION: u32 = 512;
pub const MAX_IMAGE_FRAMES: u32 = 100;
pub const MAX_DECODED_BYTES: u64 = 64 * 1024 * 1024;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ImageMediaTypeV1 {
    Png,
    Jpeg,
    Gif,
    Webp,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ImageAssetV1 {
    pub schema_version: u32,
    pub media_type: ImageMediaTypeV1,
    pub width: u32,
    pub height: u32,
    pub frames: u32,
    pub bytes: Vec<u8>,
}
impl ImageAssetV1 {
    pub fn validate(&self) -> Result<()> {
        schema(self.schema_version)?;
        let decoded = inspect(&self.bytes)?;
        if (
            decoded.media_type,
            decoded.width,
            decoded.height,
            decoded.frames,
        ) != (self.media_type, self.width, self.height, self.frames)
        {
            return Err(Error::Invalid("image metadata differs from retained bytes"));
        }
        Ok(())
    }
    /// Returns retained source bytes and a deterministic static PNG preview.
    pub fn ingest(bytes: &[u8]) -> Result<(Self, Self)> {
        let decoded = inspect(bytes)?;
        let mut preview = Cursor::new(Vec::new());
        image::DynamicImage::ImageRgba8(decoded.first).write_to(&mut preview, ImageFormat::Png)?;
        let preview_bytes = preview.into_inner();
        if preview_bytes.len() > MAX_IMAGE_BYTES {
            return Err(Error::Limit);
        }
        Ok((
            Self {
                schema_version: 1,
                media_type: decoded.media_type,
                width: decoded.width,
                height: decoded.height,
                frames: decoded.frames,
                bytes: bytes.to_vec(),
            },
            Self {
                schema_version: 1,
                media_type: ImageMediaTypeV1::Png,
                width: decoded.width,
                height: decoded.height,
                frames: 1,
                bytes: preview_bytes,
            },
        ))
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct CustomEmojiVersionV1 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    pub schema_version: u32,
    pub emoji_id: CustomEmojiId,
    pub shortcode: String,
    pub label: String,
    pub actor: PrincipalId,
    pub accepted_at_millis: i64,
    pub previous: Option<CollabObjectRef>,
    pub asset: CollabObjectRef,
    pub preview: CollabObjectRef,
    pub retired: bool,
}
impl CustomEmojiVersionV1 {
    pub fn validate(&self) -> Result<()> {
        schema(self.schema_version)?;
        if self.shortcode.is_empty()
            || self.shortcode.len() > 64
            || !self
                .shortcode
                .bytes()
                .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b"_-".contains(&b))
            || !label(&self.label, 256)
            || self.accepted_at_millis < 0
        {
            return Err(Error::Invalid("custom emoji definition"));
        }
        if let Some(reference) = self.previous {
            expect(reference, CollabObjectKind::CustomEmojiVersion)?;
        } else if self.retired {
            return Err(Error::Invalid("new retired emoji"));
        }
        expect(self.asset, CollabObjectKind::ImageAsset)?;
        expect(self.preview, CollabObjectKind::ImageAsset)?;
        Ok(())
    }
}
struct Inspected {
    media_type: ImageMediaTypeV1,
    width: u32,
    height: u32,
    frames: u32,
    first: RgbaImage,
}
fn limits() -> Limits {
    let mut value = Limits::default();
    value.max_image_width = Some(MAX_IMAGE_DIMENSION);
    value.max_image_height = Some(MAX_IMAGE_DIMENSION);
    value.max_alloc = Some(MAX_DECODED_BYTES);
    value
}
fn inspect(bytes: &[u8]) -> Result<Inspected> {
    if bytes.is_empty() || bytes.len() > MAX_IMAGE_BYTES {
        return Err(Error::Limit);
    }
    let format = image::guess_format(bytes)?;
    let media_type = match format {
        ImageFormat::Png => ImageMediaTypeV1::Png,
        ImageFormat::Jpeg => ImageMediaTypeV1::Jpeg,
        ImageFormat::Gif => ImageMediaTypeV1::Gif,
        ImageFormat::WebP => ImageMediaTypeV1::Webp,
        _ => return Err(Error::Invalid("unsupported reaction image format")),
    };
    match format {
        ImageFormat::Gif => {
            let mut decoder = image::codecs::gif::GifDecoder::new(Cursor::new(bytes))?;
            decoder.set_limits(limits())?;
            animation(media_type, decoder.into_frames())
        }
        ImageFormat::Png => {
            let mut decoder = image::codecs::png::PngDecoder::new(Cursor::new(bytes))?;
            decoder.set_limits(limits())?;
            if decoder.is_apng()? {
                animation(media_type, decoder.apng()?.into_frames())
            } else {
                static_image(bytes, format, media_type)
            }
        }
        ImageFormat::WebP => {
            let mut decoder = image::codecs::webp::WebPDecoder::new(Cursor::new(bytes))?;
            decoder.set_limits(limits())?;
            if decoder.has_animation() {
                animation(media_type, decoder.into_frames())
            } else {
                static_image(bytes, format, media_type)
            }
        }
        _ => static_image(bytes, format, media_type),
    }
}
fn static_image(
    bytes: &[u8],
    format: ImageFormat,
    media_type: ImageMediaTypeV1,
) -> Result<Inspected> {
    let mut reader = ImageReader::with_format(Cursor::new(bytes), format);
    reader.limits(limits());
    let first = reader.decode()?.to_rgba8();
    let (width, height) = first.dimensions();
    Ok(Inspected {
        media_type,
        width,
        height,
        frames: 1,
        first,
    })
}
fn animation(media_type: ImageMediaTypeV1, frames: image::Frames<'_>) -> Result<Inspected> {
    let mut first = None;
    let mut count = 0;
    let mut decoded_bytes = 0u64;
    for frame in frames {
        count += 1;
        if count > MAX_IMAGE_FRAMES {
            return Err(Error::Limit);
        }
        let frame = frame?;
        let buffer = frame.into_buffer();
        let (width, height) = buffer.dimensions();
        if width == 0 || height == 0 || width > MAX_IMAGE_DIMENSION || height > MAX_IMAGE_DIMENSION
        {
            return Err(Error::Limit);
        }
        decoded_bytes = decoded_bytes
            .checked_add(u64::from(width) * u64::from(height) * 4)
            .ok_or(Error::Limit)?;
        if decoded_bytes > MAX_DECODED_BYTES {
            return Err(Error::Limit);
        }
        if first.is_none() {
            first = Some(buffer);
        }
    }
    let first = first.ok_or(Error::Invalid("empty reaction animation"))?;
    let (width, height) = first.dimensions();
    Ok(Inspected {
        media_type,
        width,
        height,
        frames: count,
        first,
    })
}
