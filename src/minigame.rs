use rand::Rng;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ItemKind {
    Snack,
    Star,
    Hazard,
}

impl ItemKind {
    pub fn symbol(&self) -> &'static str {
        match self {
            ItemKind::Snack => "[O]",
            ItemKind::Star => "[*]",
            ItemKind::Hazard => "[X]",
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            ItemKind::Snack => "🍙 Snack",
            ItemKind::Star => "⭐ Star",
            ItemKind::Hazard => "⚡ Storm",
        }
    }

    pub fn is_reward(&self) -> bool {
        matches!(self, ItemKind::Snack | ItemKind::Star)
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
    pub basket_width: u16,
    pub items: Vec<FallingItem>,
    pub score: u32,
    pub coins_earned: u32,
    pub xp_earned: u32,
    pub caught_snacks: u32,
    pub caught_stars: u32,
    pub hits_taken: u32,
    pub ticks_remaining: u32,
}

impl MiniGame {
    pub fn new(width: u16, height: u16, duration_ticks: u32) -> Self {
        let w = width.max(6);
        let h = height.max(4);
        Self {
            width: w,
            height: h,
            player_x: w / 2,
            basket_width: 6,
            items: Vec::new(),
            score: 0,
            coins_earned: 0,
            xp_earned: 0,
            caught_snacks: 0,
            caught_stars: 0,
            hits_taken: 0,
            ticks_remaining: duration_ticks,
        }
    }

    pub fn move_left(&mut self) {
        if self.is_finished() {
            return;
        }
        self.player_x = self.player_x.saturating_sub(2);
    }

    pub fn move_right(&mut self) {
        if self.is_finished() {
            return;
        }
        let max_x = self.width.saturating_sub(1);
        self.player_x = (self.player_x + 2).min(max_x);
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
                            self.score = self.score.saturating_add(15);
                            self.coins_earned = self.coins_earned.saturating_add(3);
                            self.xp_earned = self.xp_earned.saturating_add(8);
                            self.caught_snacks = self.caught_snacks.saturating_add(1);
                        }
                        ItemKind::Star => {
                            self.score = self.score.saturating_add(25);
                            self.coins_earned = self.coins_earned.saturating_add(5);
                            self.xp_earned = self.xp_earned.saturating_add(12);
                            self.caught_stars = self.caught_stars.saturating_add(1);
                        }
                        ItemKind::Hazard => {
                            self.score = self.score.saturating_sub(15);
                            self.hits_taken = self.hits_taken.saturating_add(1);
                        }
                    }
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
            let kind = if roll < 50 {
                ItemKind::Snack
            } else if roll < 78 {
                ItemKind::Star
            } else {
                ItemKind::Hazard
            };
            self.spawn_item(spawn_x, kind);
        }

        self.ticks_remaining = self.ticks_remaining.saturating_sub(1);
    }

    pub fn render_lines(&self) -> Vec<String> {
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
        let mut game = MiniGame::new(20, 4, 50);
        let px = game.player_x;

        game.spawn_item(px, ItemKind::Snack);
        for _ in 0..3 {
            game.items.retain(|i| i.x == px && i.kind == ItemKind::Snack);
            game.step();
        }
        assert_eq!(game.score, 15);
        assert_eq!(game.coins_earned, 3);
        assert_eq!(game.xp_earned, 8);
        assert_eq!(game.caught_snacks, 1);

        game.items.clear();
        game.spawn_item(px, ItemKind::Star);
        for _ in 0..3 {
            game.items.retain(|i| i.x == px && i.kind == ItemKind::Star);
            game.step();
        }
        assert_eq!(game.score, 40);
        assert_eq!(game.coins_earned, 8);
        assert_eq!(game.xp_earned, 20);
        assert_eq!(game.caught_stars, 1);

        game.items.clear();
        game.spawn_item(px, ItemKind::Hazard);
        for _ in 0..3 {
            game.items.retain(|i| i.x == px && i.kind == ItemKind::Hazard);
            game.step();
        }
        assert_eq!(game.score, 25);
        assert_eq!(game.hits_taken, 1);
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
