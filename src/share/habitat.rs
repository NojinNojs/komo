//! Habitat vignette: dithered day/night sky, woven pedestal floor, and the
//! 40x22 companion sprite stamped at an integer pixel scale.

use super::canvas::{rgb, shade, to_rgb, NativeCanvas, Rect, BG_MID, CREMA, INK};
use super::data::ShareCardData;
use crate::senses::PetGaze;
use crate::sprites::{build_grid_with_gaze, species_palette, CANVAS_H, CANVAS_W};

const PEDI_H: usize = 16;

#[derive(Debug, Clone, Copy)]
pub struct Palette {
    pub primary: [u8; 4],
    pub secondary: [u8; 4],
    pub accent: [u8; 4],
}

pub fn card_palette(data: &ShareCardData) -> Palette {
    match data.custom_palette {
        Some((p, s, a)) => Palette {
            primary: rgb(p.0, p.1, p.2),
            secondary: rgb(s.0, s.1, s.2),
            accent: rgb(a.0, a.1, a.2),
        },
        None => {
            let (p, s, a) = species_palette(data.species);
            Palette {
                primary: to_rgb(p),
                secondary: to_rgb(s),
                accent: to_rgb(a),
            }
        }
    }
}

/// Mirrors the terminal engine's char-code mapping so the card sprite matches
/// the LCD habitat exactly, in pure integer math.
fn char_color(code: char, pal: &Palette) -> Option<[u8; 4]> {
    match code {
        '.' => None,
        'O' | '0' => Some(rgb(27, 22, 44)),
        'D' => Some(shade(pal.primary, 68)),
        'B' | 'b' => Some(pal.primary),
        'L' => Some(shade(pal.primary, 122)),
        'S' | 's' => Some(pal.secondary),
        'A' | 'a' => Some(pal.accent),
        'R' | 'r' => Some(rgb(255, 55, 75)),
        'W' | 'E' => Some(rgb(250, 252, 255)),
        'K' => Some(rgb(18, 14, 30)),
        'P' => Some(rgb(255, 110, 165)),
        'G' | 'g' => Some(rgb(255, 220, 70)),
        _ => Some(pal.primary),
    }
}

fn draw_disk(c: &mut NativeCanvas, cx: usize, cy: usize, rad: usize, col: [u8; 4]) {
    let r2 = (rad * rad) as i32;
    for dy in 0..=rad * 2 {
        for dx in 0..=rad * 2 {
            let ox = dx as i32 - rad as i32;
            let oy = dy as i32 - rad as i32;
            if ox * ox + oy * oy <= r2 {
                c.set_pixel(cx + dx - rad, cy + dy - rad, col);
            }
        }
    }
}

/// Deterministic star field — same card bytes every render, no RNG state.
fn star_level(x: usize, y: usize) -> u8 {
    let h = (x.wrapping_mul(73) + y.wrapping_mul(151) + (x * y) % 37) % 101;
    match h {
        0 => 2,
        1..=3 => 1,
        _ => 0,
    }
}

fn draw_sky(c: &mut NativeCanvas, well: Rect, data: &ShareCardData) {
    let sky_h = well.h.saturating_sub(PEDI_H);
    let sky = Rect::new(well.x, well.y, well.w, sky_h);
    if data.light_on {
        let top = rgb(116, 186, 228);
        let haze = rgb(198, 230, 244);
        c.fill_rect(sky.x, sky.y, sky.w, sky.h, top);
        c.draw_dither_band(
            Rect::new(sky.x, sky.bottom() - sky.h / 3, sky.w, sky.h / 3),
            top,
            haze,
        );
        draw_disk(c, sky.x + sky.w - 22, sky.y + 16, 7, rgb(255, 236, 170));
        draw_disk(c, sky.x + sky.w - 22, sky.y + 16, 4, CREMA);
    } else {
        let night = rgb(22, 26, 50);
        let deep = rgb(13, 16, 34);
        c.fill_rect(sky.x, sky.y, sky.w, sky.h, night);
        c.draw_dither_band(Rect::new(sky.x, sky.y, sky.w, sky.h / 2), deep, night);
        for y in sky.y..sky.bottom() {
            for x in sky.x..sky.right() {
                match star_level(x, y) {
                    2 => c.set_pixel(x, y, CREMA),
                    1 => c.set_pixel(x, y, rgb(140, 146, 190)),
                    _ => {}
                }
            }
        }
        draw_disk(c, sky.x + sky.w - 22, sky.y + 16, 6, rgb(226, 228, 214));
    }
}

fn draw_pedestal(c: &mut NativeCanvas, well: Rect, pal: &Palette) {
    let p = Rect::new(well.x, well.bottom() - PEDI_H, well.w, PEDI_H);
    let base = shade(pal.secondary, 30);
    let weave = shade(pal.secondary, 44);
    let mut by = 0;
    while p.y + by < p.bottom() {
        let mut bx = 0;
        while p.x + bx < p.right() {
            let col = if ((bx / 4) + (by / 4)) % 2 == 0 {
                base
            } else {
                weave
            };
            let w = 4.min(p.right().saturating_sub(p.x + bx));
            let h = 4.min(p.bottom().saturating_sub(p.y + by));
            c.fill_rect(p.x + bx, p.y + by, w, h, col);
            bx += 4;
        }
        by += 4;
    }
    c.fill_rect(p.x, p.y, p.w, 1, shade(pal.secondary, 62));
    c.fill_rect(p.x, p.y + 1, p.w, 1, INK);
}

fn fit_sprite_scale(well: Rect) -> usize {
    (1..=3)
        .rev()
        .find(|s| CANVAS_W * s + 8 <= well.w && CANVAS_H * s + PEDI_H + 6 <= well.h)
        .unwrap_or(1)
}

fn draw_sprite(c: &mut NativeCanvas, well: Rect, data: &ShareCardData, pal: &Palette) {
    let scale = fit_sprite_scale(well);
    let mut grid = build_grid_with_gaze(data.species, data.stage, data.mood, 0, 0, PetGaze::Center);
    crate::sprites::apply_accessory_overlay(&mut grid, data.stage, data.mood, data.accessory);
    let (sw, sh) = (CANVAS_W * scale, CANVAS_H * scale);
    let x0 = well.x + (well.w.saturating_sub(sw)) / 2;
    let floor_y = well.bottom() - PEDI_H;
    let y0 = floor_y.saturating_sub(sh);
    for (gy, row) in grid.iter().enumerate() {
        for (gx, &code) in row.iter().enumerate() {
            if let Some(col) = char_color(code, pal) {
                c.fill_rect(x0 + gx * scale, y0 + gy * scale, scale, scale, col);
            }
        }
    }
}

pub fn draw_habitat(
    c: &mut NativeCanvas,
    panel: Rect,
    data: &ShareCardData,
    pal: &Palette,
    accent: [u8; 4],
) {
    c.draw_cut_corner_panel(panel, BG_MID, accent, INK);
    let inner = panel.inset(6);
    c.draw_recessed_box(inner, INK, INK, shade(accent, 45));
    let well = inner.inset(2);
    draw_sky(c, well, data);
    draw_pedestal(c, well, pal);
    draw_sprite(c, well, data, pal);
    c.fill_rect(well.x, well.y, well.w, 1, shade(accent, 30));
}
