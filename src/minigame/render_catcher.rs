//! Plain and colored rendering for the snack catcher and shared render entry points.

use super::types::*;
use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span},
};

impl MiniGame {
    /// Plain string representation (used by unit tests).
    pub fn render_lines(&self) -> Vec<String> {
        match self.mode {
            GameMode::MindReader => vec![
                format!("ROUND {}/{}", self.mr_round, self.mr_total_rounds),
                self.mr_last_result.clone(),
            ],
            GameMode::DashRunner => vec![
                format!("STREAK x{}", self.combo.max(1)),
                "▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀".to_string(),
            ],
            GameMode::SnackCatcher => {
                let w = self.width as usize;
                let h = self.height as usize;
                let bottom = h.saturating_sub(1);
                let mut lines = Vec::with_capacity(h);
                for y in 0..h {
                    if y == bottom {
                        let mut row = vec![' '; w];
                        let basket = ['\\', '=', '=', '=', '=', '/'];
                        let center = self.player_x as isize;
                        for (i, ch) in basket.iter().enumerate() {
                            let pos = center - 3 + i as isize;
                            if pos >= 0 && (pos as usize) < w {
                                row[pos as usize] = *ch;
                            }
                        }
                        lines.push(row.into_iter().collect());
                    } else {
                        let mut row = vec![' '; w];
                        for item in &self.items {
                            if item.y as usize == y {
                                let sym = item.kind.symbol();
                                let start = (item.x as isize) - 1;
                                for (i, ch) in sym.chars().enumerate() {
                                    let pos = start + i as isize;
                                    if pos >= 0 && (pos as usize) < w {
                                        row[pos as usize] = ch;
                                    }
                                }
                            }
                        }
                        lines.push(row.into_iter().collect());
                    }
                }
                lines
            }
        }
    }

    /// Full-Color Retro Arcade Cabinet [`Line`]s for the TUI!
    pub fn render_colored_lines(&self, accent: Color) -> Vec<Line<'static>> {
        match self.mode {
            GameMode::SnackCatcher => self.render_colored_catcher(accent),
            GameMode::MindReader => self.render_colored_mind_reader(accent),
            GameMode::DashRunner => self.render_colored_dash(accent),
        }
    }

    pub(super) fn render_colored_catcher(&self, accent: Color) -> Vec<Line<'static>> {
        let w = self.width as usize;
        let h = self.height as usize;
        let bottom = h.saturating_sub(1);
        let mut out = Vec::with_capacity(h + 3);

        // Fever bar & legend
        let fever_blocks = (self.combo.min(8)) as usize;
        let fever_bar = format!(
            "{}{}",
            "█".repeat(fever_blocks),
            "░".repeat(8usize.saturating_sub(fever_blocks))
        );
        out.push(Line::from(vec![
            Span::styled(
                format!("🔥 FEVER [{fever_bar}] x{}   ", self.combo.max(1)),
                Style::default()
                    .fg(if self.combo >= 3 {
                        Color::LightYellow
                    } else {
                        Color::Cyan
                    })
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("[O] Snack ", Style::default().fg(Color::LightGreen)),
            Span::styled("[★] Star ", Style::default().fg(Color::LightYellow)),
            Span::styled("<♦> Gem ", Style::default().fg(Color::LightMagenta)),
            Span::styled("[X] Bomb", Style::default().fg(Color::LightRed)),
        ]));

        // Top arcade cabinet border
        out.push(Line::from(Span::styled(
            format!("╔{}╗", "═".repeat(w)),
            Style::default().fg(accent),
        )));

        for y in 0..h {
            if y == bottom {
                let mut cells: Vec<(char, Color)> = vec![(' ', Color::DarkGray); w];
                let cart = ['▀', '█', '█', '▀', '▀', '█', '█', '▀'];
                let center = self.player_x as isize;
                for (i, ch) in cart.iter().enumerate() {
                    let pos = center - 4 + i as isize;
                    if pos >= 0 && (pos as usize) < w {
                        cells[pos as usize] = (*ch, Color::LightYellow);
                    }
                }
                let mut spans = vec![Span::styled("║", Style::default().fg(accent))];
                for (ch, col) in cells {
                    spans.push(Span::styled(
                        ch.to_string(),
                        Style::default().fg(col).add_modifier(Modifier::BOLD),
                    ));
                }
                spans.push(Span::styled("║", Style::default().fg(accent)));
                out.push(Line::from(spans));
            } else {
                let mut cells: Vec<(char, Color)> = vec![(' ', Color::DarkGray); w];
                // Subtle retro starfield dots
                for x in (0..w).step_by(9) {
                    if (x + y * 3) % 7 == 0 {
                        cells[x] = ('·', Color::DarkGray);
                    }
                }
                for item in &self.items {
                    if item.y as usize == y {
                        let sym = item.kind.symbol();
                        let col = item.kind.color();
                        let start = (item.x as isize) - 1;
                        for (i, ch) in sym.chars().enumerate() {
                            let pos = start + i as isize;
                            if pos >= 0 && (pos as usize) < w {
                                cells[pos as usize] = (ch, col);
                            }
                        }
                    }
                }
                let mut spans = vec![Span::styled("║", Style::default().fg(accent))];
                for (ch, col) in cells {
                    spans.push(Span::styled(
                        ch.to_string(),
                        Style::default().fg(col).add_modifier(Modifier::BOLD),
                    ));
                }
                spans.push(Span::styled("║", Style::default().fg(accent)));
                out.push(Line::from(spans));
            }
        }

        out.push(Line::from(Span::styled(
            format!("╚{}╝", "═".repeat(w)),
            Style::default().fg(accent),
        )));
        out
    }
}
