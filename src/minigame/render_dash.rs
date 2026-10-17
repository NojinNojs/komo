//! Colored rendering for the Starlight Hurdle Dash mode.

use super::types::*;
use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span},
};

impl MiniGame {
    pub(super) fn render_colored_dash(&self, accent: Color) -> Vec<Line<'static>> {
        let w = self.width as usize;
        let player_col = 6usize.min(w.saturating_sub(3));
        let in_air = self.dash_jump_ticks > 0;

        let mut sky_cells: Vec<(char, Color)> = vec![(' ', Color::DarkGray); w];
        for x in (0..w).step_by(8) {
            let pos = (x + w - (self.dash_scroll % 8)) % w;
            sky_cells[pos] = ('✦', Color::DarkGray);
        }
        for &cx in &self.dash_coins {
            if (cx as usize) < w {
                sky_cells[cx as usize] = ('★', Color::LightYellow);
            }
        }
        if in_air {
            sky_cells[player_col] = ('▀', accent);
            if player_col + 1 < w {
                sky_cells[player_col + 1] = ('█', Color::LightYellow);
            }
        }

        let mut ground_cells: Vec<(char, Color)> = vec![(' ', Color::DarkGray); w];
        for &ox in &self.dash_obstacles {
            if (ox as usize) < w {
                ground_cells[ox as usize] = ('▲', Color::LightRed);
            }
        }
        if !in_air {
            ground_cells[player_col] = ('▄', accent);
            if player_col + 1 < w {
                ground_cells[player_col + 1] = ('█', Color::LightYellow);
            }
        }

        let mut sky_spans = vec![Span::styled("║", Style::default().fg(accent))];
        for (ch, col) in sky_cells {
            sky_spans.push(Span::styled(
                ch.to_string(),
                Style::default().fg(col).add_modifier(Modifier::BOLD),
            ));
        }
        sky_spans.push(Span::styled("║", Style::default().fg(accent)));

        let mut ground_spans = vec![Span::styled("║", Style::default().fg(accent))];
        for (ch, col) in ground_cells {
            ground_spans.push(Span::styled(
                ch.to_string(),
                Style::default().fg(col).add_modifier(Modifier::BOLD),
            ));
        }
        ground_spans.push(Span::styled("║", Style::default().fg(accent)));

        let mut track_spans = vec![Span::styled("╚", Style::default().fg(accent))];
        for x in 0..w {
            let is_alt = (x + self.dash_scroll).is_multiple_of(2);
            track_spans.push(Span::styled(
                if is_alt { "▀" } else { "▄" },
                Style::default().fg(if is_alt { accent } else { Color::Cyan }),
            ));
        }
        track_spans.push(Span::styled("╝", Style::default().fg(accent)));

        vec![
            Line::from(vec![
                Span::styled(
                    format!(
                        "⚡ HURDLE STREAK x{}   •   HITS: {}   •   ",
                        self.combo.max(1),
                        self.hits_taken
                    ),
                    Style::default()
                        .fg(Color::LightYellow)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    "★ Air Coin (+25)   ",
                    Style::default().fg(Color::LightYellow),
                ),
                Span::styled(
                    "▲ Fire Hurdle (Jump!)",
                    Style::default().fg(Color::LightRed),
                ),
            ]),
            Line::from(Span::styled(
                format!("╔{}╗", "═".repeat(w)),
                Style::default().fg(accent),
            )),
            Line::from(Span::styled(
                format!("║{:^width$}║", "·  ✦  STARLIGHT STADIUM  ✦  ·", width = w),
                Style::default().fg(Color::DarkGray),
            )),
            Line::from(sky_spans),
            Line::from(ground_spans),
            Line::from(track_spans),
            Line::from(""),
            Line::from(Span::styled(
                "Press [SPACE] or [↑ / W] to LEAP over ▲ Fire Hurdles & grab ★ Air Coins!",
                Style::default()
                    .fg(Color::LightGreen)
                    .add_modifier(Modifier::BOLD),
            )),
        ]
    }
}
