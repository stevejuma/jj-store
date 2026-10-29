//! Profile/emoji source normalization is shared with native consumers.
use super::*;
#[cfg(test)]
use image::{DynamicImage, ImageFormat};
#[cfg(test)]
use jj_collab::ImageAssetV1;
#[cfg(test)]
use std::io::Cursor;

pub(super) fn normalize(
    bytes: &[u8],
) -> Result<(jj_collab::ImageAssetV1, jj_collab::ImageAssetV1)> {
    jj_collab::normalize_settings_image(bytes).map_err(|error| match error {
        jj_collab::Error::Limit => Error::Limit,
        _ => Error::Invalid,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    fn raster(width: u32, height: u32, format: ImageFormat) -> Vec<u8> {
        let image = DynamicImage::ImageRgb8(image::RgbImage::from_pixel(
            width,
            height,
            image::Rgb([30, 90, 150]),
        ));
        let mut bytes = Cursor::new(Vec::new());
        image.write_to(&mut bytes, format).unwrap();
        bytes.into_inner()
    }
    #[test]
    fn large_jpeg_avatars_resize_and_keep_aspect_ratio() {
        let bytes = raster(2500, 1800, ImageFormat::Jpeg);
        assert!(ImageAssetV1::ingest(&bytes).is_err());
        let (asset, preview) = normalize(&bytes).unwrap();
        assert_eq!((asset.width, asset.height), (512, 369));
        assert_eq!(asset.media_type, jj_collab::ImageMediaTypeV1::Png);
        asset.validate().unwrap();
        preview.validate().unwrap();
    }
    #[test]
    fn resized_jpeg_avatars_apply_exif_orientation() {
        let bytes = raster(2500, 1800, ImageFormat::Jpeg);
        let exif = b"Exif\0\0II*\0\x08\0\0\0\x01\0\x12\x01\x03\0\x01\0\0\0\x06\0\0\0\0\0\0\0";
        let mut oriented = bytes[..2].to_vec();
        oriented.extend_from_slice(&[0xff, 0xe1]);
        oriented.extend_from_slice(&((exif.len() + 2) as u16).to_be_bytes());
        oriented.extend_from_slice(exif);
        oriented.extend_from_slice(&bytes[2..]);
        let (asset, _) = normalize(&oriented).unwrap();
        assert_eq!((asset.width, asset.height), (369, 512));
    }
    #[test]
    fn oversized_emoji_rasters_normalize_and_keep_transparency() {
        for format in [ImageFormat::Png, ImageFormat::Gif, ImageFormat::WebP] {
            let photo = DynamicImage::ImageRgba8(image::RgbaImage::from_pixel(
                800,
                400,
                image::Rgba([60, 90, 150, 0]),
            ));
            let mut bytes = Cursor::new(Vec::new());
            photo.write_to(&mut bytes, format).unwrap();
            let (asset, preview) = normalize(&bytes.into_inner()).unwrap();
            assert_eq!((asset.width, asset.height), (512, 256));
            assert_eq!(asset.media_type, jj_collab::ImageMediaTypeV1::Png);
            assert_eq!(
                image::load_from_memory(&asset.bytes)
                    .unwrap()
                    .to_rgba8()
                    .get_pixel(0, 0)[3],
                0
            );
            asset.validate().unwrap();
            preview.validate().unwrap();
        }
    }
    #[test]
    fn source_file_over_two_mib_is_normalized_and_small_sources_keep_bytes() {
        let bytes = raster(32, 24, ImageFormat::Png);
        assert_eq!(normalize(&bytes).unwrap().0.bytes, bytes);
        let mut large = bytes.clone();
        large.resize(jj_collab::MAX_IMAGE_BYTES + 1, 0);
        let (asset, _) = normalize(&large).unwrap();
        assert_eq!((asset.width, asset.height), (32, 24));
        assert!(asset.bytes.len() < jj_collab::MAX_IMAGE_BYTES);
    }
    #[test]
    fn avatar_decode_and_source_limits_reject_before_publication() {
        assert!(matches!(normalize(b"not an image"), Err(Error::Invalid)));
        assert!(matches!(
            normalize(&vec![0; MAX_IMAGE_UPLOAD_BYTES + 1]),
            Err(Error::Limit)
        ));
        assert!(matches!(
            normalize(&raster(8193, 1, ImageFormat::Png)),
            Err(Error::Limit)
        ));
        assert!(matches!(
            normalize(&raster(4097, 4096, ImageFormat::Png)),
            Err(Error::Limit)
        ));
    }
}
