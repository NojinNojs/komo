//! 3-in-1 Retro Arcade Center (`src/minigame.rs`) for **Mochigotchi v1.1.0**:
//! 1. **🍡 Snack & Star Catcher** (`GameMode::SnackCatcher`): Move the pixel
//!    basket `←`/`→`, build up a **Combo Multiplier (`x2`, `x3`, `FEVER!`)**,
//!    catch `🍙 Snack`, `⭐ Star`, `💎 Fever Gem`, and dodge `💣 Bomb`!
//! 2. **🔮 Bandai Left/Right Mind-Reader** (`GameMode::MindReader`): Authentic
//!    1996 Tamagotchi Gen 1 5-round guessing game! Guess whether your companion
//!    will look Left (`←`) or Right (`→`).
//! 3. **⚡ Starlight Hurdle Dash** (`GameMode::DashRunner`): Retro side-scrolling
//!    endless runner! Press `[Space]` or `[↑]` to jump over obstacles (`🔥`)
//!    and collect floating star coins (`🪙`).

use rand::Rng;

/// The 3 playable retro minigames in the Arcade Center (`[G]`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameMode {
    SnackCatcher,
    MindReader,
    DashRunner,
}

impl GameMode {
    pub fn all() -> &'static [GameMode] {
        &[Self::SnackCatcher, Self::MindReader, Self::DashRunner]
    }

    pub fn title(&self) -> &'static str {
        match self {
            Self::SnackCatcher => "1. 🍡 Snack & Star Catcher",
            Self::MindReader => "2. 🔮 Left/Right Mind-Reader",
            Self::DashRunner => "3. ⚡ Starlight Hurdle Dash",
        }
    }

    pub fn next(&self) -> Self {
        match self {
            Self::SnackCatcher => Self::MindReader,
            Self::MindReader => Self::DashRunner,
            Self::DashRunner => Self::SnackCatcher,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ItemKind {
    Snack,
    Star,
    FeverGem,
    Hazard,
}

impl ItemKind {
    pub fn symbol(&self) -> &'static str {
        match self {
            ItemKind::Snack => "[O]",
            ItemKind::Star => "[★]",
            ItemKind::FeverGem => "<♦>",
            ItemKind::Hazard => "[X]",
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            ItemKind::Snack => "🍙 Snack",
            ItemKind::Star => "⭐ Star",
            ItemKind::FeverGem => "💎 Gem",
            ItemKind::Hazard => "💣 Bomb",
        }
    }

    pub fn is_reward(&self) -> bool {
        !matches!(self, ItemKind::Hazard)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FallingItem {
    pub x: u16,
    pub y: u16,
    pub kind: ItemKind,
}

#[derive(Debug, Clone)]
pub struct MiniGame {
    pub mode: GameMode,
    pub width: u16,
    pub height: u16,
    pub player_x: u16,
    pub basket_width: u16,
    pub items: Vec<FallingItem>,
    pub combo: u32,
    pub max_combo: u32,
    pub score: u32,
    pub coins_earned: u32,
    pub xp_earned: u32,
    pub caught_snacks: u32,
    pub caught_stars: u32,
    pub hits_taken: u32,
    pub ticks_remaining: u32,
    // Mode 2: Bandai Left/Right Mind-Reader state
    pub mr_round: u8,
    pub mr_total_rounds: u8,
    pub mr_wins: u8,
    pub mr_last_pet_dir: Option<bool>, // false = Left, true = Right
    pub mr_last_result: String,
    // Mode 3: Starlight Hurdle Dash state
    pub dash_jump_ticks: u8,
    pub dash_obstacles: Vec<u16>,
    pub dash_coins: Vec<u16>,
}

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
            mr_last_pet_dir: None,
            mr_last_result: "Guess which way your pet will look: press [←] Left or [→] Right!"
                .to_string(),
            dash_jump_ticks: 0,
            dash_obstacles: vec![w.saturating_sub(4)],
            dash_coins: vec![w / 2],
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

    /// Jump in `DashRunner` mode (or trigger a guess in `MindReader`).
    pub fn jump(&mut self) {
        if self.is_finished() {
            return;
        }
        if self.mode == GameMode::DashRunner && self.dash_jump_ticks == 0 {
            self.dash_jump_ticks = 4;
        }
    }

    /// Play one round of the classic Bandai Left/Right Mind-Reader game!
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

        if won_round {
            self.mr_wins = self.mr_wins.saturating_add(1);
            self.combo = self.combo.saturating_add(1);
            self.max_combo = self.max_combo.max(self.combo);
            self.score = self.score.saturating_add(25 * self.combo);
            self.coins_earned = self.coins_earned.saturating_add(4);
            self.xp_earned = self.xp_earned.saturating_add(10);
            self.mr_last_result = format!(
                "🎉 Round {}/{}: Pet looked {dir_label} — MATCH! (+4 coins, +10 XP)",
                self.mr_round, self.mr_total_rounds
            );
        } else {
            self.combo = 0;
            self.xp_earned = self.xp_earned.saturating_add(3);
            self.mr_last_result = format!(
                "💫 Round {}/{}: Pet looked {dir_label} — Missed! (+3 XP for trying)",
                self.mr_round, self.mr_total_rounds
            );
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
                // MindReader advances when the player presses Left/Right, with a gentle timeout
                self.ticks_remaining = self.ticks_remaining.saturating_sub(1);
            }
            GameMode::DashRunner => self.step_dash_runner(),
            GameMode::SnackCatcher => self.step_snack_catcher(),
        }
    }

    fn step_dash_runner(&mut self) {
        let player_col = 5u16;
        let in_air = self.dash_jump_ticks > 0;
        if self.dash_jump_ticks > 0 {
            self.dash_jump_ticks -= 1;
        }

        // Advance obstacles left
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

        // Advance floating star coins left
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

    fn step_snack_catcher(&mut self) {
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

    pub fn render_lines(&self) -> Vec<String> {
        match self.mode {
            GameMode::MindReader => self.render_mind_reader(),
            GameMode::DashRunner => self.render_dash_runner(),
            GameMode::SnackCatcher => self.render_snack_catcher(),
        }
    }

    fn render_mind_reader(&self) -> Vec<String> {
        let dir_art = match self.mr_last_pet_dir {
            Some(false) => "   ◀◀  ( ^ ω ^ )      ",
            Some(true) => "       ( ^ ω ^ )  ▶▶  ",
            None => "   ◀ ? ( o . o ) ? ▶  ",
        };
        vec![
            format!(
                "ROUND {}/{}   •   WINS: {}   •   STREAK: x{}",
                self.mr_round,
                self.mr_total_rounds,
                self.mr_wins,
                self.combo.max(1)
            ),
            String::new(),
            "      .-------------------.".to_string(),
            format!("      |{dir_art}|"),
            "      '-------------------'".to_string(),
            String::new(),
            self.mr_last_result.clone(),
            String::new(),
            "Press [← / A] to guess LEFT   or   [→ / D] to guess RIGHT!".to_string(),
        ]
    }

    fn render_dash_runner(&self) -> Vec<String> {
        let w = self.width as usize;
        let player_col = 5usize.min(w.saturating_sub(2));
        let in_air = self.dash_jump_ticks > 0;

        let mut sky_row = vec![' '; w];
        for &cx in &self.dash_coins {
            if (cx as usize) < w {
                sky_row[cx as usize] = '★';
            }
        }
        if in_air {
            sky_row[player_col] = '♦';
        }

        let mut ground_row = vec![' '; w];
        for &ox in &self.dash_obstacles {
            if (ox as usize) < w {
                ground_row[ox as usize] = '▲';
            }
        }
        if !in_air {
            ground_row[player_col] = '♦';
        }

        vec![
            format!(
                "STREAK x{}  •  JUMP WITH [SPACE / ↑]  •  COLLECT ★  •  DODGE ▲",
                self.combo.max(1)
            ),
            String::new(),
            sky_row.into_iter().collect(),
            ground_row.into_iter().collect(),
            "▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀".to_string(),
        ]
    }

    fn render_snack_catcher(&self) -> Vec<String> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minigame_movement_and_bounds() {
        let mut game = MiniGame::new(12, 6, 50);
        assert_eq!(game.player_x, 6);
        for _ in 0..20 {
            game.move_left();
        }
        assert_eq!(game.player_x, 0);
        for _ in 0..20 {
            game.move_right();
        }
        assert_eq!(game.player_x, 11);
    }

    #[test]
    fn test_minigame_catch_snacks_stars_and_hazards() {
        let mut game = MiniGame::new(20, 6, 50);
        let px = game.player_x;

        game.spawn_item(px, ItemKind::Snack);
        for _ in 0..5 {
            game.items.retain(|i| i.x == px && i.kind == ItemKind::Snack);
            game.step();
        }
        assert!(game.score >= 15);
        assert_eq!(game.coins_earned, 3);
        assert_eq!(game.xp_earned, 8);
        assert_eq!(game.caught_snacks, 1);

        game.items.clear();
        game.spawn_item(px, ItemKind::Star);
        for _ in 0..5 {
            game.items.retain(|i| i.x == px && i.kind == ItemKind::Star);
            game.step();
        }
        assert!(game.score >= 40);
        assert_eq!(game.coins_earned, 8);
        assert_eq!(game.xp_earned, 20);
        assert_eq!(game.caught_stars, 1);

        game.items.clear();
        game.spawn_item(px, ItemKind::Hazard);
        for _ in 0..5 {
            game.items.retain(|i| i.x == px && i.kind == ItemKind::Hazard);
            game.step();
        }
        assert_eq!(game.hits_taken, 1);
    }

    #[test]
    fn test_mind_reader_and_dash_runner_modes() {
        let mut mr = MiniGame::new_with_mode(GameMode::MindReader, 30, 8, 100);
        for _ in 0..5 {
            mr.guess_direction(true);
        }
        assert!(mr.is_finished());
        assert!(mr.xp_earned > 0);
        assert!(!mr.render_lines().is_empty());

        let mut dash = MiniGame::new_with_mode(GameMode::DashRunner, 30, 8, 25);
        dash.jump();
        assert!(dash.dash_jump_ticks > 0);
        for _ in 0..25 {
            dash.step();
        }
        assert!(dash.is_finished());
        assert!(!dash.render_lines().is_empty());
    }

    #[test]
    fn test_minigame_render_and_completion() {
        let mut game = MiniGame::new(20, 6, 5);
        game.spawn_item(8, ItemKind::Snack);
        game.items[0].y = 1;
        let lines = game.render_lines();
        assert_eq!(lines.len(), 6);
        assert!(lines[1].contains("[O]"));
        assert!(lines[5].contains("\\====/"));

        game.ticks_remaining = 1;
        game.step();
        assert!(game.is_finished());
    }
}
