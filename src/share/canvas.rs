//! Low-resolution pixel canvas with retro bevel primitives for share cards.

use ratatui::style::Color;

pub const fn rgb(r: u8, g: u8, b: u8) -> [u8; 4] {
    [r, g, b, 255]
}

pub const INK: [u8; 4] = rgb(11, 8, 14);
pub const BG_DEEP: [u8; 4] = rgb(20, 16, 25);
pub const BG_MID: [u8; 4] = rgb(29, 24, 40);
pub const PANEL_HI: [u8; 4] = rgb(58, 48, 80);
pub const PANEL_LO: [u8; 4] = rgb(13, 10, 20);
pub const CREMA: [u8; 4] = rgb(255, 243, 214);
pub const AMBER: [u8; 4] = rgb(244, 180, 70);
pub const TEXT_LABEL: [u8; 4] = rgb(169, 155, 196);
pub const TEXT_DIM: [u8; 4] = rgb(110, 98, 134);
pub const BAR_UNLIT: [u8; 4] = rgb(42, 35, 56);
pub const STAT_GOOD: [u8; 4] = rgb(143, 214, 148);
pub const STAT_WARN: [u8; 4] = rgb(244, 180, 70);
pub const STAT_BAD: [u8; 4] = rgb(242, 118, 107);
pub const BOND_PINK: [u8; 4] = rgb(255, 143, 181);

/// Integer brightness scale (`percent = 100` is identity), no float rounding drift.
pub fn shade(c: [u8; 4], percent: u16) -> [u8; 4] {
    let f = |v: u8| ((u16::from(v) * percent) / 100).min(255) as u8;
    [f(c[0]), f(c[1]), f(c[2]), 255]
}

/// Extract RGBA from a ratatui color; every species/theme color in this crate is `Rgb`.
pub fn to_rgb(c: Color) -> [u8; 4] {
    match c {
        Color::Rgb(r, g, b) => rgb(r, g, b),
        _ => CREMA,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rect {
    pub x: usize,
    pub y: usize,
    pub w: usize,
    pub h: usize,
}

impl Rect {
    pub fn new(x: usize, y: usize, w: usize, h: usize) -> Self {
        Self { x, y, w, h }
    }

    pub fn inset(self, m: usize) -> Self {
        Self {
            x: self.x + m,
            y: self.y + m,
            w: self.w.saturating_sub(m * 2),
            h: self.h.saturating_sub(m * 2),
        }
    }

    pub fn right(self) -> usize {
        self.x + self.w
    }

    pub fn bottom(self) -> usize {
        self.y + self.h
    }
}

pub struct NativeCanvas {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<[u8; 4]>,
}

impl NativeCanvas {
    pub fn new(width: usize, height: usize, bg: [u8; 4]) -> Self {
        Self {
            width,
            height,
            pixels: vec![bg; width * height],
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, c: [u8; 4]) {
        if x < self.width && y < self.height {
            self.pixels[y * self.width + x] = c;
        }
    }

    pub fn fill_rect(&mut self, x: usize, y: usize, w: usize, h: usize, c: [u8; 4]) {
        let x_end = x.saturating_add(w).min(self.width);
        let y_end = y.saturating_add(h).min(self.height);
        for row in y.min(self.height)..y_end {
            for col in x.min(self.width)..x_end {
                self.pixels[row * self.width + col] = c;
            }
        }
    }

    /// 2px cut-corner panel: 1px top/left highlight bevel, 1px bottom/right shadow bevel.
    pub fn draw_cut_corner_panel(&mut self, r: Rect, face: [u8; 4], hi: [u8; 4], lo: [u8; 4]) {
        if r.w < 5 || r.h < 5 {
            return;
        }
        self.fill_rect(r.x + 2, r.y, r.w - 4, r.h, face);
        self.fill_rect(r.x, r.y + 2, r.w, r.h - 4, face);
        self.fill_rect(r.x + 2, r.y, r.w - 4, 1, hi);
        self.fill_rect(r.x, r.y + 2, 1, r.h - 4, hi);
        self.fill_rect(r.x + 2, r.y + r.h - 1, r.w - 4, 1, lo);
        self.fill_rect(r.x + r.w - 1, r.y + 2, 1, r.h - 4, lo);
    }

    /// Recessed slot with inverted bevels: dark top/left edge, light bottom/right edge.
    pub fn draw_recessed_box(&mut self, r: Rect, face: [u8; 4], dark: [u8; 4], light: [u8; 4]) {
        if r.w < 2 || r.h < 2 {
            return;
        }
        self.fill_rect(r.x, r.y, r.w, r.h, face);
        self.fill_rect(r.x, r.y, r.w, 1, dark);
        self.fill_rect(r.x, r.y, 1, r.h, dark);
        self.fill_rect(r.x, r.y + r.h - 1, r.w, 1, light);
        self.fill_rect(r.x + r.w - 1, r.y, 1, r.h, light);
    }

    /// 1x1 checkerboard dither of `b` over `a` for gradients and ambient glow.
    pub fn draw_dither_band(&mut self, r: Rect, a: [u8; 4], b: [u8; 4]) {
        let x_end = r.right().min(self.width);
        let y_end = r.bottom().min(self.height);
        for y in r.y.min(self.height)..y_end {
            for x in r.x.min(self.width)..x_end {
                self.pixels[y * self.width + x] = if (x + y) % 2 == 0 { a } else { b };
            }
        }
    }

    /// Segmented bar with exact integer segment widths; the remainder widens the
    /// leading segments by 1px each so the track is always filled edge-to-edge.
    pub fn draw_segmented_bar(
        &mut self,
        r: Rect,
        segs: usize,
        gap: usize,
        filled: usize,
        lit: [u8; 4],
        unlit: [u8; 4],
    ) {
        if segs == 0 || r.w == 0 {
            return;
        }
        let usable = r.w.saturating_sub(gap * (segs - 1));
        let base = usable / segs;
        let extra = usable % segs;
        let mut x = r.x;
        for i in 0..segs {
            let seg_w = base + usize::from(i < extra);
            self.fill_rect(x, r.y, seg_w, r.h, if i < filled { lit } else { unlit });
            x += seg_w + gap;
        }
    }

    /// Integer nearest-neighbor upscale to packed RGBA bytes (never fractional scales).
    pub fn upscale_nearest(&self, scale: usize) -> (u32, u32, Vec<u8>) {
        let scale = scale.max(1);
        let (ow, oh) = (self.width * scale, self.height * scale);
        let mut out = vec![0u8; ow * oh * 4];
        let row_stride = ow * 4;
        for sy in 0..self.height {
            for sx in 0..self.width {
                let p = self.pixels[sy * self.width + sx];
                for dy in 0..scale {
                    let start = (sy * scale + dy) * row_stride + sx * scale * 4;
                    for dx in 0..scale {
                        out[start + dx * 4..start + dx * 4 + 4].copy_from_slice(&p);
                    }
                }
            }
        }
        (ow as u32, oh as u32, out)
    }
}
