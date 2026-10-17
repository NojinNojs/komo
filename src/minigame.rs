use rand::Rng;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ItemKind {
    Bug,
    Crate,
    Coffee,
    Panic,
}

impl ItemKind {
    pub fn symbol(&self) -> &'static str {
        match self {
            ItemKind::Crate => "[C]",
            ItemKind::Coffee => "[☕]",
            ItemKind::Bug => "[B]",
            ItemKind::Panic => "[!]",
        }
    }

    pub fn is_reward(&self) -> bool {
        matches!(self, ItemKind::Crate | ItemKind::Coffee)
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
    pub width: u16,
    pub height: u16,
    pub player_x: u16,
    pub score: u32,
    pub coins_earned: u32,
    pub xp_earned: u32,
    pub ticks_remaining: u32,
    pub active: bool,
    pub items: Vec<FallingItem>,
    pub status_msg: String,
}

impl MiniGame {
    pub fn new(width: u16, height: u16) -> Self {
        let w = width.max(5);
        let h = height.max(3);
        Self {
            width: w,
            height: h,
            player_x: w / 2,
            score: 0,
            coins_earned: 0,
            xp_earned: 0,
            ticks_remaining: 120,
            active: true,
            items: Vec::new(),
            status_msg: "Catch [C] Crates & [☕] Coffee! Dodge [B] Bugs & [!] Panics!".to_string(),
        }
    }

    pub fn move_left(&mut self) {
        if !self.active {
            return;
        }
        self.player_x = self.player_x.saturating_sub(1);
    }

    pub fn move_right(&mut self) {
        if !self.active {
            return;
        }
        if self.player_x + 1 < self.width {
            self.player_x += 1;
        }
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
        if !self.active || self.ticks_remaining == 0 {
            self.active = false;
            return;
        }

        let bottom_row = self.height.saturating_sub(1);
        let player_x = self.player_x;

        let mut remaining_items = Vec::with_capacity(self.items.len());
        for mut item in self.items.drain(..) {
            item.y = item.y.saturating_add(1);
            if item.y >= bottom_row {
                let dist = item.x.abs_diff(player_x);
                if dist <= 1 {
                    match item.kind {
                        ItemKind::Crate => {
                            self.score = self.score.saturating_add(15);
                            self.coins_earned = self.coins_earned.saturating_add(3);
                            self.xp_earned = self.xp_earned.saturating_add(8);
                            self.status_msg =
                                "Caught a crate [C]! +15 score, +3 coins, +8 XP".to_string();
                        }
                        ItemKind::Coffee => {
                            self.score = self.score.saturating_add(20);
                            self.coins_earned = self.coins_earned.saturating_add(5);
                            self.xp_earned = self.xp_earned.saturating_add(10);
                            self.status_msg =
                                "Caught coffee [☕]! +20 score, +5 coins, +10 XP".to_string();
                        }
                        ItemKind::Bug => {
                            self.score = self.score.saturating_sub(10);
                            self.status_msg = "Hit a bug [B]! -10 score".to_string();
                        }
                        ItemKind::Panic => {
                            self.score = self.score.saturating_sub(20);
                            self.status_msg = "Hit a panic [!]! -20 score".to_string();
                        }
                    }
                }
            } else {
                remaining_items.push(item);
            }
        }
        self.items = remaining_items;

        let mut rng = rand::thread_rng();
        if rng.gen_range(0..100) < 45 {
            let spawn_x = rng.gen_range(0..self.width);
            let roll = rng.gen_range(0..100);
            let kind = if roll < 35 {
                ItemKind::Crate
            } else if roll < 55 {
                ItemKind::Coffee
            } else if roll < 80 {
                ItemKind::Bug
            } else {
                ItemKind::Panic
            };
            self.spawn_item(spawn_x, kind);
        }

        self.ticks_remaining = self.ticks_remaining.saturating_sub(1);
        if self.ticks_remaining == 0 {
            self.active = false;
            self.status_msg = format!(
                "Game Over! Final Score: {} (+{} coins, +{} XP)",
                self.score, self.coins_earned, self.xp_earned
            );
        }
    }

    pub fn is_finished(&self) -> bool {
        !self.active || self.ticks_remaining == 0
    }

    pub fn render_lines(&self) -> Vec<String> {
        let w = self.width as usize;
        let h = self.height as usize;
        let bottom = h.saturating_sub(1);
        let mut lines = Vec::with_capacity(h);

        for y in 0..h {
            if y == bottom {
                let mut row = vec![' '; w];
                let catcher = ['[', '=', '=', '=', ']'];
                let center = self.player_x as isize;
                for (i, ch) in catcher.iter().enumerate() {
                    let pos = center - 2 + i as isize;
                    if pos >= 0 && (pos as usize) < w {
                        row[pos as usize] = *ch;
                    }
                }
                lines.push(row.into_iter().collect());
            } else {
                let mut cells: Vec<&str> = vec![" "; w];
                for item in &self.items {
                    if item.y as usize == y && (item.x as usize) < w {
                        cells[item.x as usize] = item.kind.symbol();
                    }
                }
                lines.push(cells.concat());
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
        let mut game = MiniGame::new(10, 6);
        assert_eq!(game.player_x, 5);
        for _ in 0..20 {
            game.move_left();
        }
        assert_eq!(game.player_x, 0);
        for _ in 0..20 {
            game.move_right();
        }
        assert_eq!(game.player_x, 9);
    }

    #[test]
    fn test_minigame_catch_rewards_and_hazards() {
        let mut game = MiniGame::new(20, 4);
        let px = game.player_x;

        game.spawn_item(px, ItemKind::Crate);
        for _ in 0..3 {
            game.items.retain(|i| i.x == px && i.kind == ItemKind::Crate);
            game.step();
        }
        assert_eq!(game.score, 15);
        assert_eq!(game.coins_earned, 3);
        assert_eq!(game.xp_earned, 8);

        game.items.clear();
        game.spawn_item(px, ItemKind::Coffee);
        for _ in 0..3 {
            game.items.retain(|i| i.x == px && i.kind == ItemKind::Coffee);
            game.step();
        }
        assert_eq!(game.score, 35);
        assert_eq!(game.coins_earned, 8);
        assert_eq!(game.xp_earned, 18);

        game.items.clear();
        game.spawn_item(px, ItemKind::Bug);
        for _ in 0..3 {
            game.items.retain(|i| i.x == px && i.kind == ItemKind::Bug);
            game.step();
        }
        assert_eq!(game.score, 25);

        game.items.clear();
        game.spawn_item(px, ItemKind::Panic);
        for _ in 0..3 {
            game.items.retain(|i| i.x == px && i.kind == ItemKind::Panic);
            game.step();
        }
        assert_eq!(game.score, 5);
    }

    #[test]
    fn test_minigame_render_and_completion() {
        let mut game = MiniGame::new(16, 6);
        game.spawn_item(4, ItemKind::Crate);
        game.items[0].y = 1;
        let lines = game.render_lines();
        assert_eq!(lines.len(), 6);
        assert!(lines[1].contains("[C]"));
        assert!(lines[5].contains("[===]"));

        game.ticks_remaining = 1;
        game.step();
        assert!(game.is_finished());
        assert!(!game.active);
    }
}
