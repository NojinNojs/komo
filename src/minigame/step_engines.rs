//! Per-mode simulation steps for the dash runner and snack catcher.

use super::types::*;
use rand::Rng;

impl MiniGame {
    pub(super) fn step_dash_runner(&mut self) {
        self.dash_scroll = self.dash_scroll.wrapping_add(1);
        let player_col = 6u16;
        let in_air = self.dash_jump_ticks > 0;
        if self.dash_jump_ticks > 0 {
            self.dash_jump_ticks -= 1;
        }

        let mut next_obs = Vec::new();
        for obs_x in self.dash_obstacles.drain(..) {
            if obs_x == player_col {
                if in_air {
                    self.combo = self.combo.saturating_add(1);
                    self.max_combo = self.max_combo.max(self.combo);
                    self.score = self.score.saturating_add(20);
                    self.xp_earned = self.xp_earned.saturating_add(6);
                } else {
                    self.combo = 0;
                    self.hits_taken = self.hits_taken.saturating_add(1);
                    self.score = self.score.saturating_sub(10);
                }
            }
            if obs_x > 0 {
                next_obs.push(obs_x - 1);
            }
        }
        self.dash_obstacles = next_obs;

        let mut next_coins = Vec::new();
        for coin_x in self.dash_coins.drain(..) {
            if coin_x.abs_diff(player_col) <= 1 && in_air {
                self.score = self.score.saturating_add(25);
                self.coins_earned = self.coins_earned.saturating_add(3);
                self.xp_earned = self.xp_earned.saturating_add(8);
                self.caught_stars = self.caught_stars.saturating_add(1);
            } else if coin_x > 0 {
                next_coins.push(coin_x - 1);
            }
        }
        self.dash_coins = next_coins;

        let mut rng = rand::thread_rng();
        let min_gap = self
            .dash_obstacles
            .iter()
            .all(|&x| x < self.width.saturating_sub(9));
        if min_gap && rng.gen_range(0..100) < 35 {
            self.dash_obstacles.push(self.width.saturating_sub(2));
        }
        if self.dash_coins.len() < 3 && rng.gen_range(0..100) < 30 {
            self.dash_coins.push(self.width.saturating_sub(2));
        }

        self.ticks_remaining = self.ticks_remaining.saturating_sub(1);
    }

    pub(super) fn step_snack_catcher(&mut self) {
        let bottom_row = self.height.saturating_sub(1);
        let player_x = self.player_x;
        let catch_radius = (self.basket_width / 2).max(2);

        let mut remaining_items = Vec::with_capacity(self.items.len());
        for mut item in self.items.drain(..) {
            item.y = item.y.saturating_add(1);
            if item.y >= bottom_row {
                if item.x.abs_diff(player_x) <= catch_radius {
                    match item.kind {
                        ItemKind::Snack => {
                            self.combo = self.combo.saturating_add(1);
                            self.max_combo = self.max_combo.max(self.combo);
                            let mult = 1 + (self.combo / 4);
                            self.score = self.score.saturating_add(15 * mult);
                            self.coins_earned = self.coins_earned.saturating_add(3);
                            self.xp_earned = self.xp_earned.saturating_add(8);
                            self.caught_snacks = self.caught_snacks.saturating_add(1);
                        }
                        ItemKind::Star => {
                            self.combo = self.combo.saturating_add(1);
                            self.max_combo = self.max_combo.max(self.combo);
                            let mult = 1 + (self.combo / 3);
                            self.score = self.score.saturating_add(25 * mult);
                            self.coins_earned = self.coins_earned.saturating_add(5);
                            self.xp_earned = self.xp_earned.saturating_add(12);
                            self.caught_stars = self.caught_stars.saturating_add(1);
                        }
                        ItemKind::FeverGem => {
                            self.combo = self.combo.saturating_add(2);
                            self.max_combo = self.max_combo.max(self.combo);
                            self.score = self.score.saturating_add(45);
                            self.coins_earned = self.coins_earned.saturating_add(8);
                            self.xp_earned = self.xp_earned.saturating_add(18);
                            self.caught_stars = self.caught_stars.saturating_add(1);
                        }
                        ItemKind::Hazard => {
                            self.combo = 0;
                            self.score = self.score.saturating_sub(15);
                            self.hits_taken = self.hits_taken.saturating_add(1);
                        }
                    }
                } else if item.kind.is_reward() {
                    self.combo = 0;
                }
            } else {
                remaining_items.push(item);
            }
        }
        self.items = remaining_items;

        let mut rng = rand::thread_rng();
        if rng.gen_range(0..100) < 48 {
            let spawn_x = rng.gen_range(1..self.width.saturating_sub(1).max(2));
            let roll = rng.gen_range(0..100);
            let kind = if roll < 45 {
                ItemKind::Snack
            } else if roll < 72 {
                ItemKind::Star
            } else if roll < 82 {
                ItemKind::FeverGem
            } else {
                ItemKind::Hazard
            };
            self.spawn_item(spawn_x, kind);
        }

        self.ticks_remaining = self.ticks_remaining.saturating_sub(1);
    }
}
