//! `MiniGame` constructors, mode switching, player controls, and per-tick stepping.

use super::types::*;
use rand::Rng;

impl MiniGame {
    pub fn new(width: u16, height: u16, duration_ticks: u32) -> Self {
        Self::new_with_mode(GameMode::SnackCatcher, width, height, duration_ticks)
    }

    pub fn new_with_mode(mode: GameMode, width: u16, height: u16, duration_ticks: u32) -> Self {
        let w = width.max(12);
        let h = height.max(6);
        Self {
            mode,
            width: w,
            height: h,
            player_x: w / 2,
            basket_width: 6,
            items: Vec::new(),
            combo: 0,
            max_combo: 0,
            score: 0,
            coins_earned: 0,
            xp_earned: 0,
            caught_snacks: 0,
            caught_stars: 0,
            hits_taken: 0,
            ticks_remaining: duration_ticks,
            mr_round: 1,
            mr_total_rounds: 5,
            mr_wins: 0,
            mr_history: Vec::new(),
            mr_last_pet_dir: None,
            mr_last_result: "Press [← / A] to guess LEFT  or  [→ / D] to guess RIGHT!".to_string(),
            dash_jump_ticks: 0,
            dash_obstacles: vec![w.saturating_sub(4)],
            dash_coins: vec![w / 2],
            dash_scroll: 0,
        }
    }

    pub fn switch_mode(&mut self, mode: GameMode) {
        *self = Self::new_with_mode(mode, self.width, self.height, 120);
    }

    pub fn move_left(&mut self) {
        if self.is_finished() {
            return;
        }
        if self.mode == GameMode::MindReader {
            self.guess_direction(false);
            return;
        }
        self.player_x = self.player_x.saturating_sub(2);
    }

    pub fn move_right(&mut self) {
        if self.is_finished() {
            return;
        }
        if self.mode == GameMode::MindReader {
            self.guess_direction(true);
            return;
        }
        let max_x = self.width.saturating_sub(1);
        self.player_x = (self.player_x + 2).min(max_x);
    }

    pub fn jump(&mut self) {
        if self.is_finished() {
            return;
        }
        if self.mode == GameMode::DashRunner && self.dash_jump_ticks == 0 {
            self.dash_jump_ticks = 4;
        }
    }

    pub fn guess_direction(&mut self, guessed_right: bool) -> bool {
        if self.is_finished() || self.mode != GameMode::MindReader {
            return false;
        }
        let mut rng = rand::thread_rng();
        let pet_looked_right: bool = rng.gen_bool(0.5);
        self.mr_last_pet_dir = Some(pet_looked_right);

        let dir_label = if pet_looked_right {
            "RIGHT ▶"
        } else {
            "◀ LEFT"
        };
        let won_round = guessed_right == pet_looked_right;
        self.mr_history.push(won_round);

        if won_round {
            self.mr_wins = self.mr_wins.saturating_add(1);
            self.combo = self.combo.saturating_add(1);
            self.max_combo = self.max_combo.max(self.combo);
            self.score = self.score.saturating_add(25 * self.combo);
            self.coins_earned = self.coins_earned.saturating_add(4);
            self.xp_earned = self.xp_earned.saturating_add(10);
            self.mr_last_result =
                format!("🎉 MATCH! Companion turned {dir_label}! (+4 coins, +10 XP)");
        } else {
            self.combo = 0;
            self.xp_earned = self.xp_earned.saturating_add(3);
            self.mr_last_result =
                format!("💫 Companion turned {dir_label} — Missed! (+3 XP for trying)");
        }

        if self.mr_round >= self.mr_total_rounds {
            if self.mr_wins >= 3 {
                self.coins_earned = self.coins_earned.saturating_add(10);
                self.xp_earned = self.xp_earned.saturating_add(15);
            }
            self.ticks_remaining = 0;
        } else {
            self.mr_round += 1;
        }
        won_round
    }

    pub fn is_finished(&self) -> bool {
        self.ticks_remaining == 0
    }

    pub fn spawn_item(&mut self, x: u16, kind: ItemKind) {
        let clamped_x = x.min(self.width.saturating_sub(1));
        self.items.push(FallingItem {
            x: clamped_x,
            y: 0,
            kind,
        });
    }

    pub fn step(&mut self) {
        if self.is_finished() {
            return;
        }

        match self.mode {
            GameMode::MindReader => {
                self.ticks_remaining = self.ticks_remaining.saturating_sub(1);
            }
            GameMode::DashRunner => self.step_dash_runner(),
            GameMode::SnackCatcher => self.step_snack_catcher(),
        }
    }
}
