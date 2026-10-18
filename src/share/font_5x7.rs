//! Single 5x7 bitmap font engine for every string on a share card. `X` = lit
//! pixel, tracking 1px, stamped with a 1px (scaled) hard drop shadow.
//! Lowercase maps to uppercase on lookup so the `?` fallback never appears.

use super::canvas::{NativeCanvas, INK};

pub const FONT_W: usize = 5;
pub const FONT_H: usize = 7;
pub const FONT_TRACKING: usize = 1;

#[rustfmt::skip]
pub const FONT_5X7: &[(char, [&str; FONT_H])] = &[
    ('A', [".XXX.", "X...X", "X...X", "XXXXX", "X...X", "X...X", "X...X"]),
    ('B', ["XXXX.", "X...X", "X...X", "XXXX.", "X...X", "X...X", "XXXX."]),
    ('C', [".XXXX", "X....", "X....", "X....", "X....", "X....", ".XXXX"]),
    ('D', ["XXXX.", "X...X", "X...X", "X...X", "X...X", "X...X", "XXXX."]),
    ('E', ["XXXXX", "X....", "X....", "XXXX.", "X....", "X....", "XXXXX"]),
    ('F', ["XXXXX", "X....", "X....", "XXXX.", "X....", "X....", "X...."]),
    ('G', [".XXXX", "X....", "X....", "X..XX", "X...X", "X...X", ".XXXX"]),
    ('H', ["X...X", "X...X", "X...X", "XXXXX", "X...X", "X...X", "X...X"]),
    ('I', ["XXXXX", "..X..", "..X..", "..X..", "..X..", "..X..", "XXXXX"]),
    ('J', ["..XXX", "...X.", "...X.", "...X.", "...X.", "X..X.", ".XX.."]),
    ('K', ["X...X", "X..X.", "X.X..", "XX...", "X.X..", "X..X.", "X...X"]),
    ('L', ["X....", "X....", "X....", "X....", "X....", "X....", "XXXXX"]),
    ('M', ["X...X", "XX.XX", "X.X.X", "X.X.X", "X...X", "X...X", "X...X"]),
    ('N', ["X...X", "XX..X", "X.X.X", "X..XX", "X...X", "X...X", "X...X"]),
    ('O', [".XXX.", "X...X", "X...X", "X...X", "X...X", "X...X", ".XXX."]),
    ('P', ["XXXX.", "X...X", "X...X", "XXXX.", "X....", "X....", "X...."]),
    ('Q', [".XXX.", "X...X", "X...X", "X...X", "X.X.X", "X..X.", ".XX.X"]),
    ('R', ["XXXX.", "X...X", "X...X", "XXXX.", "X.X..", "X..X.", "X...X"]),
    ('S', [".XXXX", "X....", "X....", ".XXX.", "....X", "....X", "XXXX."]),
    ('T', ["XXXXX", "..X..", "..X..", "..X..", "..X..", "..X..", "..X.."]),
    ('U', ["X...X", "X...X", "X...X", "X...X", "X...X", "X...X", ".XXX."]),
    ('V', ["X...X", "X...X", "X...X", "X...X", "X...X", ".X.X.", "..X.."]),
    ('W', ["X...X", "X...X", "X...X", "X.X.X", "X.X.X", "X.X.X", ".X.X."]),
    ('X', ["X...X", "X...X", ".X.X.", "..X..", ".X.X.", "X...X", "X...X"]),
    ('Y', ["X...X", "X...X", ".X.X.", "..X..", "..X..", "..X..", "..X.."]),
    ('Z', ["XXXXX", "....X", "...X.", "..X..", ".X...", "X....", "XXXXX"]),
    ('0', [".XXX.", "X...X", "X..XX", "X.X.X", "XX..X", "X...X", ".XXX."]),
    ('1', ["..X..", ".XX..", "..X..", "..X..", "..X..", "..X..", "XXXXX"]),
    ('2', [".XXX.", "X...X", "....X", "...X.", "..X..", ".X...", "XXXXX"]),
    ('3', ["XXXX.", "....X", "....X", ".XXX.", "....X", "....X", "XXXX."]),
    ('4', ["...X.", "..XX.", ".X.X.", "X..X.", "XXXXX", "...X.", "...X."]),
    ('5', ["XXXXX", "X....", "XXXX.", "....X", "....X", "X...X", ".XXX."]),
    ('6', ["..XX.", ".X...", "X....", "XXXX.", "X...X", "X...X", ".XXX."]),
    ('7', ["XXXXX", "....X", "...X.", "..X..", "..X..", "..X..", "..X.."]),
    ('8', [".XXX.", "X...X", "X...X", ".XXX.", "X...X", "X...X", ".XXX."]),
    ('9', [".XXX.", "X...X", "X...X", ".XXXX", "....X", "...X.", ".XX.."]),
    (' ', [".....", ".....", ".....", ".....", ".....", ".....", "....."]),
    ('.', [".....", ".....", ".....", ".....", ".....", ".XX..", ".XX.."]),
    (':', [".....", "..X..", "..X..", ".....", "..X..", "..X..", "....."]),
    ('/', ["....X", "....X", "...X.", "..X..", ".X...", "X....", "X...."]),
    ('-', [".....", ".....", ".....", ".XXX.", ".....", ".....", "....."]),
    ('+', [".....", "..X..", "..X..", "XXXXX", "..X..", "..X..", "....."]),
    ('%', [".XX.X", ".XX.X", "...X.", "..X..", ".X...", "X.XX.", "X.XX."]),
    ('!', ["..X..", "..X..", "..X..", "..X..", "..X..", ".....", "..X.."]),
    ('?', [".XXX.", "X...X", "....X", "...X.", "..X..", ".....", "..X.."]),
    ('\'', ["..X..", "..X..", ".....", ".....", ".....", ".....", "....."]),
    ('"', [".X.X.", ".X.X.", ".....", ".....", ".....", ".....", "....."]),
    ('(', ["...X.", "..X..", ".X...", ".X...", ".X...", "..X..", "...X."]),
    (')', [".X...", "..X..", "...X.", "...X.", "...X.", "..X..", ".X..."]),
    ('[', [".XXX.", ".X...", ".X...", ".X...", ".X...", ".X...", ".XXX."]),
    (']', [".XXX.", "...X.", "...X.", "...X.", "...X.", "...X.", ".XXX."]),
    ('*', [".....", "X.X.X", ".XXX.", "XXXXX", ".XXX.", "X.X.X", "....."]),
    ('\u{2022}', [".....", ".....", ".XX..", ".XX..", ".....", ".....", "....."]),
];

#[derive(Debug, Clone, Copy)]
pub struct TextStyle {
    pub color: [u8; 4],
    pub shadow: Option<[u8; 4]>,
    pub scale: usize,
}

impl TextStyle {
    pub const fn new(color: [u8; 4], scale: usize) -> Self {
        Self {
            color,
            shadow: Some(INK),
            scale,
        }
    }

    pub const fn plain(color: [u8; 4], scale: usize) -> Self {
        Self {
            color,
            shadow: None,
            scale,
        }
    }
}

pub fn has_glyph(c: char) -> bool {
    let key = c.to_ascii_uppercase();
    FONT_5X7.iter().any(|(ch, _)| *ch == key)
}

pub fn glyph(c: char) -> [&'static str; FONT_H] {
    let key = c.to_ascii_uppercase();
    FONT_5X7
        .iter()
        .find(|(ch, _)| *ch == key)
        .map_or(["....."; FONT_H], |(_, rows)| *rows)
}

pub fn measure_text(text: &str, scale: usize) -> (usize, usize) {
    let n = text.chars().count();
    let w = if n == 0 {
        0
    } else {
        (n * (FONT_W + FONT_TRACKING) - FONT_TRACKING) * scale
    };
    (w, FONT_H * scale)
}

/// Ink bounding box including the 1px (scaled) drop shadow, for exact centering.
pub fn visual_size(text: &str, st: TextStyle) -> (usize, usize) {
    let (mut w, mut h) = measure_text(text, st.scale);
    if st.shadow.is_some() && w > 0 {
        w += st.scale;
        h += st.scale;
    }
    (w, h)
}

pub fn draw_text(c: &mut NativeCanvas, x: usize, y: usize, text: &str, st: TextStyle) {
    let advance = (FONT_W + FONT_TRACKING) * st.scale;
    for (i, ch) in text.chars().enumerate() {
        let gx = x + i * advance;
        for (ry, row) in glyph(ch).iter().enumerate() {
            for (rx, cell) in row.chars().enumerate() {
                if cell != 'X' {
                    continue;
                }
                let (px, py) = (gx + rx * st.scale, y + ry * st.scale);
                if let Some(s) = st.shadow {
                    c.fill_rect(px + st.scale, py + st.scale, st.scale, st.scale, s);
                }
                c.fill_rect(px, py, st.scale, st.scale, st.color);
            }
        }
    }
}

pub fn draw_text_centered(
    c: &mut NativeCanvas,
    area_x: usize,
    area_w: usize,
    y: usize,
    text: &str,
    st: TextStyle,
) {
    let (vw, _) = visual_size(text, st);
    draw_text(c, area_x + (area_w.saturating_sub(vw)) / 2, y, text, st);
}

pub fn draw_text_right(c: &mut NativeCanvas, right_x: usize, y: usize, text: &str, st: TextStyle) {
    let (vw, _) = visual_size(text, st);
    draw_text(c, right_x.saturating_sub(vw), y, text, st);
}
