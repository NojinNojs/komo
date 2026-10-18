//! Card composer: handheld shell frame, header banner, footer bezel, and
//! per-ratio region math (wide ratios sit side-by-side, tall ratios stack).

use super::canvas::{
    shade, to_rgb, NativeCanvas, Rect, AMBER, BG_DEEP, BG_MID, CREMA, INK, PANEL_HI, STAT_BAD,
    STAT_GOOD, TEXT_DIM, TEXT_LABEL,
};
use super::data::ShareCardData;
use super::font_5x7::{draw_text, draw_text_centered, measure_text, visual_size, TextStyle};
use super::habitat::{card_palette, draw_habitat};
use super::png_encoder::encode_rgba_png;
use super::ratio::ShareRatio;
use super::stats::draw_stats_panel;
use crate::sprites::theme_color;

const FOOTER_TEXT: &str = "KOMO \u{2022} MOCHIGOTCHI POCKET SOUL";
const MARGIN: usize = 8;
const GAP: usize = 6;

pub struct ShareCardCanvas {
    pub ratio: ShareRatio,
    pub canvas: NativeCanvas,
}

impl ShareCardCanvas {
    pub fn width(&self) -> u32 {
        (self.canvas.width * self.ratio.scale()) as u32
    }

    pub fn height(&self) -> u32 {
        (self.canvas.height * self.ratio.scale()) as u32
    }

    pub fn into_png_bytes(&self) -> Vec<u8> {
        let (w, h, px) = self.canvas.upscale_nearest(self.ratio.scale());
        encode_rgba_png(w, h, &px)
    }
}

fn rank_color(tier: u8, accent: [u8; 4]) -> [u8; 4] {
    match tier {
        0 => AMBER,
        1 => accent,
        2 => STAT_GOOD,
        _ => STAT_BAD,
    }
}

fn draw_shell(c: &mut NativeCanvas, w: usize, h: usize, accent: [u8; 4]) {
    c.draw_cut_corner_panel(
        Rect::new(1, 1, w - 2, h - 2),
        shade(accent, 26),
        shade(accent, 55),
        INK,
    );
    c.draw_cut_corner_panel(
        Rect::new(5, 5, w - 10, h - 10),
        BG_DEEP,
        shade(accent, 40),
        INK,
    );
}

fn draw_header(c: &mut NativeCanvas, r: Rect, data: &ShareCardData, accent: [u8; 4]) {
    c.draw_cut_corner_panel(r, BG_MID, accent, INK);

    let badge_st = TextStyle::plain(INK, 1);
    let (bw, _) = visual_size(&data.rank_label, badge_st);
    let badge_w = bw + 12;
    let badge_h = 14;
    let bx = r.right().saturating_sub(8 + badge_w);
    let by = r.y + (r.h - badge_h) / 2;
    let face = rank_color(data.rank_tier, accent);
    c.draw_cut_corner_panel(
        Rect::new(bx, by, badge_w, badge_h),
        face,
        shade(face, 135),
        INK,
    );
    draw_text_centered(
        c,
        bx,
        badge_w,
        by + (badge_h - 7) / 2,
        &data.rank_label,
        badge_st,
    );

    let name_x = r.x + 8;
    let name_room = bx.saturating_sub(name_x + GAP);
    let name_scale = if measure_text(&data.name, 2).0 <= name_room {
        2
    } else {
        1
    };
    let name_y = r.y + if name_scale == 2 { 3 } else { 7 };
    draw_text(
        c,
        name_x,
        name_y,
        &data.name,
        TextStyle::new(CREMA, name_scale),
    );

    let subtitle = data.subtitle();
    let sub = super::stats::clip_to_px(&subtitle, r.w - 16, 1);
    draw_text(
        c,
        name_x,
        r.bottom() - 12,
        &sub,
        TextStyle::new(TEXT_LABEL, 1),
    );
}

fn draw_footer(c: &mut NativeCanvas, r: Rect) {
    c.draw_recessed_box(r, BG_MID, INK, PANEL_HI);
    let st = TextStyle::plain(TEXT_DIM, 1);
    let ty = r.y + (r.h - 7) / 2;
    draw_text_centered(c, r.x, r.w, ty, FOOTER_TEXT, st);
    let (tw, _) = measure_text(FOOTER_TEXT, 1);
    let side = (r.w.saturating_sub(tw)) / 2;
    if side >= 12 {
        let gw = side - 8;
        c.draw_dither_band(Rect::new(r.x + 5, ty, gw, 7), BG_MID, PANEL_HI);
        c.draw_dither_band(Rect::new(r.right() - 5 - gw, ty, gw, 7), BG_MID, PANEL_HI);
    }
}

pub fn compose(data: &ShareCardData, ratio: ShareRatio) -> ShareCardCanvas {
    let (w, h) = ratio.native();
    let mut c = NativeCanvas::new(w, h, INK);
    let accent = to_rgb(theme_color(data.theme));
    let pal = card_palette(data);
    draw_shell(&mut c, w, h, accent);

    let wide = w >= h;
    let header_h = if wide { 34 } else { 32 };
    let header = Rect::new(MARGIN, MARGIN, w - MARGIN * 2, header_h);
    let footer = Rect::new(MARGIN, h - MARGIN - 12, w - MARGIN * 2, 12);
    let mid_y = header.bottom() + GAP;
    let mid_h = footer.y - GAP - mid_y;
    let (habitat, stats) = if wide {
        let total = w - MARGIN * 2 - GAP;
        let share = if w >= 384 { 48 } else { 45 };
        let hab_w = total * share / 100;
        (
            Rect::new(MARGIN, mid_y, hab_w, mid_h),
            Rect::new(MARGIN + hab_w + GAP, mid_y, total - hab_w, mid_h),
        )
    } else {
        let hab_h = mid_h * 52 / 100;
        (
            Rect::new(MARGIN, mid_y, w - MARGIN * 2, hab_h),
            Rect::new(
                MARGIN,
                mid_y + hab_h + GAP,
                w - MARGIN * 2,
                mid_h - hab_h - GAP,
            ),
        )
    };

    draw_header(&mut c, header, data, accent);
    draw_habitat(&mut c, habitat, data, &pal, accent);
    draw_stats_panel(&mut c, stats, data);
    draw_footer(&mut c, footer);
    ShareCardCanvas { ratio, canvas: c }
}
