//! Pixel-art promo share cards for **KOMO** — rendered at a low-res native
//! canvas, integer-upscaled 3x, and encoded to PNG with zero external image
//! crates (stored DEFLATE + hand-rolled CRC-32 in [`png_encoder`]).

mod canvas;
mod data;
mod font_5x7;
mod habitat;
mod layout;
mod png_encoder;
mod ratio;
mod stats;

pub use canvas::{NativeCanvas, Rect};
pub use data::{sanitize_card_text, CustomPalette, ShareCardData};
pub use font_5x7::{
    draw_text, draw_text_centered, draw_text_right, measure_text, TextStyle, FONT_5X7,
};
pub use layout::{compose, ShareCardCanvas};
pub use png_encoder::encode_rgba_png;
pub use ratio::ShareRatio;

use crate::pet::Pet;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

/// Render the promo card PNG (RGBA, `native * 3` pixels) for a live pet.
pub fn render_share_card_png(pet: &Pet, ratio: ShareRatio) -> Vec<u8> {
    render_share_card_data_png(&ShareCardData::from_pet(pet), ratio)
}

/// Render the promo card PNG from a decoupled [`ShareCardData`] payload.
pub fn render_share_card_data_png(data: &ShareCardData, ratio: ShareRatio) -> Vec<u8> {
    compose(data, ratio).into_png_bytes()
}

pub fn save_share_card_png(pet: &Pet, ratio: ShareRatio, out_path: &Path) -> io::Result<()> {
    fs::write(out_path, render_share_card_png(pet, ratio))
}

/// `komo_<sanitized-name>_<ratio-slug>.png` in the current directory.
pub fn default_share_output_path(pet: &Pet, ratio: ShareRatio) -> PathBuf {
    let mapped: String = pet
        .name
        .trim()
        .to_ascii_lowercase()
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' })
        .collect();
    let mut slug = mapped;
    while slug.contains("__") {
        slug = slug.replace("__", "_");
    }
    let slug = slug.trim_matches('_');
    let slug = if slug.is_empty() { "pet" } else { slug };
    PathBuf::from(format!("komo_{slug}_{}.png", ratio.slug()))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ihdr_dims(png: &[u8]) -> (u32, u32) {
        let w = u32::from_be_bytes([png[16], png[17], png[18], png[19]]);
        let h = u32::from_be_bytes([png[20], png[21], png[22], png[23]]);
        (w, h)
    }

    #[test]
    fn png_has_valid_signature_and_chunks() {
        let pet = Pet::new("Koma");
        let png = render_share_card_png(&pet, ShareRatio::Landscape16x9);
        assert_eq!(&png[..8], b"\x89PNG\r\n\x1a\n");
        assert_eq!(&png[12..16], b"IHDR");
        assert!(png.windows(4).any(|w| w == b"IDAT"));
        let iend = png.windows(4).rposition(|w| w == b"IEND").expect("IEND");
        assert_eq!(png.len(), iend + 8);
    }

    #[test]
    fn all_ratios_render_exact_upscaled_dimensions() {
        let pet = Pet::new("Koma");
        for ratio in ShareRatio::all() {
            let png = render_share_card_png(&pet, *ratio);
            let (nw, nh) = ratio.native();
            let s = ratio.scale() as u32;
            assert_eq!(ihdr_dims(&png), (nw as u32 * s, nh as u32 * s));
        }
    }

    #[test]
    fn ratio_parsing_labels_and_navigation() {
        assert_eq!(ShareRatio::parse("16:9"), Some(ShareRatio::Landscape16x9));
        assert_eq!(ShareRatio::parse("story"), Some(ShareRatio::Story9x16));
        assert_eq!(ShareRatio::parse("4:3"), Some(ShareRatio::Classic4x3));
        assert_eq!(ShareRatio::parse("Portrait"), Some(ShareRatio::Portrait3x4));
        assert_eq!(ShareRatio::parse("21:9"), None);
        assert_eq!(ShareRatio::Landscape16x9.label(), "16:9");
        assert_eq!(ShareRatio::Story9x16.slug(), "9x16");
        assert_eq!(ShareRatio::Landscape16x9.next(), ShareRatio::Story9x16);
        assert_eq!(ShareRatio::Landscape16x9.prev(), ShareRatio::Portrait3x4);
        assert_eq!(ShareRatio::Portrait3x4.next(), ShareRatio::Landscape16x9);
    }

    #[test]
    fn output_path_sanitizes_emoji_and_punctuation() {
        let pet = Pet::new("Ko \u{1F431} mo!!");
        let path = default_share_output_path(&pet, ShareRatio::Story9x16);
        assert_eq!(path.file_name().unwrap(), "komo_ko_mo_9x16.png");
    }

    #[test]
    fn sanitized_card_text_never_keeps_unknown_glyphs() {
        assert_eq!(sanitize_card_text("\u{1F431} Neko Mochi"), "NEKO MOCHI");
        assert_eq!(sanitize_card_text("a-b 100%"), "A-B 100%");
    }
}
