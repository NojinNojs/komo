//! Colored rendering for the Bandai Left/Right Mind-Reader mode.

use super::types::*;
use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span},
};

impl MiniGame {
    pub(super) fn render_colored_mind_reader(&self, accent: Color) -> Vec<Line<'static>> {
        let mut round_badges = Vec::with_capacity(14);
        round_badges.push(Span::styled(
            format!("ROUND {}/{}  •  ", self.mr_round, self.mr_total_rounds),
            Style::default()
                .fg(Color::LightYellow)
                .add_modifier(Modifier::BOLD),
        ));
        for r in 0..self.mr_total_rounds as usize {
            let badge = match self.mr_history.get(r) {
                Some(true) => Span::styled(
                    "[ ★ WIN ] ",
                    Style::default()
                        .fg(Color::LightGreen)
                        .add_modifier(Modifier::BOLD),
                ),
                Some(false) => Span::styled("[ ✗ MISS] ", Style::default().fg(Color::LightRed)),
                None if r + 1 == self.mr_round as usize => Span::styled(
                    "[ ? NOW ] ",
                    Style::default()
                        .fg(Color::Black)
                        .bg(Color::LightYellow)
                        .add_modifier(Modifier::BOLD),
                ),
                None => Span::styled("[ · · · ] ", Style::default().fg(Color::DarkGray)),
            };
            round_badges.push(badge);
        }

        let left_active = self.mr_last_pet_dir == Some(false);
        let right_active = self.mr_last_pet_dir == Some(true);
        let left_style = if left_active {
            Style::default()
                .fg(Color::Black)
                .bg(Color::LightCyan)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::LightCyan)
        };
        let right_style = if right_active {
            Style::default()
                .fg(Color::Black)
                .bg(Color::LightMagenta)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::LightMagenta)
        };

        let pet_face = match self.mr_last_pet_dir {
            Some(false) => "◀◀  ( ◕ ω ◕ )    ",
            Some(true) => "    ( ◕ ω ◕ )  ▶▶",
            None => " ◀? ( ◕ . ◕ ) ?▶ ",
        };

        vec![
            Line::from(round_badges),
            Line::from(""),
            Line::from(vec![
                Span::styled("╔═══════════════╗   ", left_style),
                Span::styled("▄███████████████▄", Style::default().fg(accent)),
                Span::styled("   ╔═══════════════╗", right_style),
            ]),
            Line::from(vec![
                Span::styled("║  ◀◀ LEFT [←]  ║   ", left_style),
                Span::styled(
                    pet_face,
                    Style::default()
                        .fg(Color::LightYellow)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled("   ║  [→] RIGHT ▶▶ ║", right_style),
            ]),
            Line::from(vec![
                Span::styled("╚═══════════════╝   ", left_style),
                Span::styled("▀███████████████▀", Style::default().fg(accent)),
                Span::styled("   ╚═══════════════╝", right_style),
            ]),
            Line::from(""),
            Line::from(Span::styled(
                self.mr_last_result.clone(),
                Style::default()
                    .fg(Color::LightGreen)
                    .add_modifier(Modifier::BOLD),
            )),
            Line::from(""),
            Line::from(Span::styled(
                "Win 3 out of 5 rounds for a +10 Coin & +15 XP Champion Bonus!",
                Style::default().fg(Color::Gray),
            )),
        ]
    }
}
