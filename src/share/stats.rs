//! Stats & vitals panel: stage/level/age/coat lines plus segmented pixel bars.

use super::canvas::{
    NativeCanvas, Rect, BAR_UNLIT, BG_MID, BOND_PINK, CREMA, INK, PANEL_HI, PANEL_LO, STAT_BAD,
    STAT_GOOD, STAT_WARN, TEXT_LABEL,
};
use super::data::ShareCardData;
use super::font_5x7::{draw_text, draw_text_right, measure_text, TextStyle, FONT_TRACKING, FONT_W};

const LABEL_COL: usize = 40;
const VALUE_COL: usize = 26;
const BAR_GAP: usize = 2;
const BAR_SEGS: usize = 10;

fn stat_color(v: u8) -> [u8; 4] {
    match v {
        60..=255 => STAT_GOOD,
        30..=59 => STAT_WARN,
        _ => STAT_BAD,
    }
}

/// Pixel-exact truncation for runtime strings (names, titles) that must fit a box.
pub(super) fn clip_to_px(text: &str, max_w: usize, scale: usize) -> String {
    let (w, _) = measure_text(text, scale);
    if w <= max_w {
        return text.to_string();
    }
    let advance = (FONT_W + FONT_TRACKING) * scale;
    let max_chars = (max_w + FONT_TRACKING * scale) / advance;
    text.chars().take(max_chars).collect()
}

pub fn draw_stats_panel(c: &mut NativeCanvas, panel: Rect, data: &ShareCardData) {
    c.draw_cut_corner_panel(panel, BG_MID, PANEL_HI, PANEL_LO);
    let inner = panel.inset(7);
    let label_st = TextStyle::new(TEXT_LABEL, 1);
    let value_st = TextStyle::new(CREMA, 1);

    let text_pitch = if inner.h >= 100 { 11 } else { 10 };
    draw_text(
        c,
        inner.x,
        inner.y,
        &format!("STAGE {}", data.stage_label),
        label_st,
    );
    draw_text_right(
        c,
        inner.right(),
        inner.y,
        &format!("LV.{}", data.level),
        value_st,
    );
    draw_text(
        c,
        inner.x,
        inner.y + text_pitch,
        &data.age_label(),
        label_st,
    );
    draw_text_right(
        c,
        inner.right(),
        inner.y + text_pitch,
        &format!("COINS {}", data.coins),
        value_st,
    );
    let coat_full = data.coat_line();
    let coat = if measure_text(&coat_full, 1).0 <= inner.w {
        coat_full
    } else {
        clip_to_px(&data.coat_line_compact(), inner.w, 1)
    };
    draw_text(c, inner.x, inner.y + text_pitch * 2, &coat, label_st);

    let bars_top = inner.y + text_pitch * 3 + 1;
    let pitch = ((inner.bottom() - bars_top - 9) / 4).clamp(9, 15);
    let rows: [(&str, u8, [u8; 4]); 5] = [
        ("HUNGRY", data.hunger, stat_color(data.hunger)),
        ("HAPPY", data.happiness, stat_color(data.happiness)),
        ("HEALTH", data.health, stat_color(data.health)),
        ("ENERGY", data.energy, stat_color(data.energy)),
        ("BOND", data.bond, BOND_PINK),
    ];
    let max_bar_w = inner.w.saturating_sub(LABEL_COL + VALUE_COL + 8);
    let seg_w = max_bar_w.saturating_sub(4 + BAR_GAP * (BAR_SEGS - 1)) / BAR_SEGS;
    let bar_w = seg_w * BAR_SEGS + BAR_GAP * (BAR_SEGS - 1) + 4;
    let bar_x = inner.x + LABEL_COL + 4;

    for (i, (label, value, color)) in rows.iter().enumerate() {
        let y = bars_top + i * pitch;
        draw_text(c, inner.x, y + 1, label, label_st);
        let track = Rect::new(bar_x, y, bar_w, 9);
        c.draw_recessed_box(track, INK, INK, PANEL_HI);
        let filled = (usize::from(*value) * BAR_SEGS + 50) / 100;
        c.draw_segmented_bar(track.inset(2), BAR_SEGS, BAR_GAP, filled, *color, BAR_UNLIT);
        draw_text_right(c, inner.right(), y + 1, &format!("{value}%"), value_st);
    }
}
