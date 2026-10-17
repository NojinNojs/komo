use ratatui::style::Color;

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
        game.items
            .retain(|i| i.x == px && i.kind == ItemKind::Snack);
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
        game.items
            .retain(|i| i.x == px && i.kind == ItemKind::Hazard);
        game.step();
    }
    assert_eq!(game.hits_taken, 1);
}

#[test]
fn test_mind_reader_and_dash_runner_modes_and_colored_render() {
    let mut mr = MiniGame::new_with_mode(GameMode::MindReader, 36, 8, 100);
    for _ in 0..5 {
        mr.guess_direction(true);
    }
    assert!(mr.is_finished());
    assert!(mr.xp_earned > 0);
    assert!(!mr.render_colored_lines(Color::Cyan).is_empty());

    let mut dash = MiniGame::new_with_mode(GameMode::DashRunner, 36, 8, 25);
    dash.jump();
    assert!(dash.dash_jump_ticks > 0);
    for _ in 0..25 {
        dash.step();
    }
    assert!(dash.is_finished());
    assert!(!dash.render_colored_lines(Color::LightYellow).is_empty());
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
    assert!(!game.render_colored_lines(Color::LightGreen).is_empty());

    game.ticks_remaining = 1;
    game.step();
    assert!(game.is_finished());
}
