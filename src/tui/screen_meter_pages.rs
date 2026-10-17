use super::*;

pub(super) fn push_meter_page_lines(
    lines: &mut Vec<Line<'static>>,
    app: &App,
    pet: &Pet,
    page_idx: usize,
) {
    match page_idx {
        0 => {
            let xp_pct = (pet.xp_progress_ratio() * 100.0).round() as u8;
            let target_xp = pet.stage.next_stage().map_or(pet.xp, |n| n.min_xp());
            lines.push(Line::from(Span::styled(
                "PAGE 1/4  •  IDENTITY, EMOTION, WEIGHT & CARE RANK",
                Style::default()
                    .fg(Color::LightYellow)
                    .add_modifier(Modifier::BOLD),
            )));
            lines.push(Line::from(""));
            lines.push(Line::from(format!(
                " • Companion: {} ({})   •   Calls You: '{}'",
                pet.name,
                pet.species.display_name(),
                pet.caretaker_label()
            )));
            lines.push(Line::from(format!(
                " • Emotion:   {}   •   Stage: {} (Lv.{} • {} XP)",
                pet.mood.badge(),
                pet.stage.badge(),
                pet.level,
                pet.xp
            )));
            lines.push(Line::from(format!(
                " • Care Rank: {}",
                pet.care_rank().badge()
            )));
            lines.push(Line::from(format!(
                " • Real Age:  {}",
                format_duration_human(pet.age_seconds)
            )));
            lines.push(Line::from(format!(
                " • Weight:    {}g (Base {}g{})",
                pet.weight_grams,
                pet.stage.base_weight_grams(),
                if pet.is_overweight() {
                    " — Overweight! Play [P]"
                } else {
                    " — Healthy!"
                }
            )));
            lines.push(Line::from(""));
            lines.push(custom_meter_line(
                "⭐ Stage XP",
                xp_pct,
                Color::LightMagenta,
                &format!("{}/{} XP", pet.xp, target_xp),
                Color::LightMagenta,
            ));
        }
        1 => {
            lines.push(Line::from(Span::styled(
                "PAGE 2/4  •  HUNGRY & HAPPY HEARTS",
                Style::default()
                    .fg(Color::LightGreen)
                    .add_modifier(Modifier::BOLD),
            )));
            lines.push(Line::from(""));
            lines.push(custom_meter_line(
                "🍙 Hungry",
                pet.hunger,
                Color::LightGreen,
                &format_hearts(pet.hungry_hearts()),
                Color::LightGreen,
            ));
            lines.push(Line::from(""));
            lines.push(custom_meter_line(
                "💖 Happy",
                pet.happiness,
                Color::LightMagenta,
                &format_hearts(pet.happy_hearts()),
                Color::LightMagenta,
            ));
            lines.push(Line::from(""));
            lines.push(Line::from(format!(
                " • Sweet Streak:   {}/3 snacks {}",
                pet.snacks_in_a_row,
                if pet.has_toothache {
                    "(🦷 TOOTHACHE!)"
                } else {
                    "(Safe)"
                }
            )));
            lines.push(Line::from(format!(
                " • Favorite Meal:  {}",
                pet.species.favorite_snack()
            )));
            lines.push(Line::from(format!(" • Coins Purse:    {} 💰", pet.coins)));
        }
        2 => {
            let disc_blocks = pet.discipline_blocks() as usize;
            let disc_bar = format!(
                "{}{}",
                "■".repeat(disc_blocks),
                "·".repeat(4usize.saturating_sub(disc_blocks))
            );
            lines.push(Line::from(Span::styled(
                "PAGE 3/4  •  DISCIPLINE, HYGIENE & HEALTH",
                Style::default()
                    .fg(Color::LightCyan)
                    .add_modifier(Modifier::BOLD),
            )));
            lines.push(Line::from(""));
            lines.push(custom_meter_line(
                "📢 Discipline",
                pet.discipline,
                Color::LightYellow,
                &format!("{disc_bar} {disc_blocks}/4"),
                Color::LightYellow,
            ));
            lines.push(custom_meter_line(
                "⚡ Energy",
                pet.energy,
                Color::Yellow,
                if pet.is_sleeping { "Sleeping" } else { "Awake" },
                Color::Yellow,
            ));
            lines.push(custom_meter_line(
                "🚽 Hygiene",
                pet.cleanliness,
                Color::LightCyan,
                &format!("Poop: {}", pet.poop_count),
                Color::LightCyan,
            ));
            lines.push(custom_meter_line(
                "💚 Health",
                pet.health,
                Color::LightRed,
                if !pet.is_alive {
                    "Passed Out!"
                } else if pet.has_toothache {
                    "Toothache!"
                } else if pet.is_sick {
                    "Sick!"
                } else {
                    "Healthy"
                },
                Color::LightGreen,
            ));
            lines.push(custom_meter_line(
                "🤝 Bond",
                pet.bond,
                Color::LightBlue,
                &format!("Mistakes: {}", pet.care_mistakes),
                Color::Gray,
            ));
        }
        _ => {
            lines.push(Line::from(Span::styled(
                "PAGE 4/4  •  EVOLUTION GOAL, MEMORIALS & CARE DIARY",
                Style::default()
                    .fg(Color::LightMagenta)
                    .add_modifier(Modifier::BOLD),
            )));
            lines.push(Line::from(Span::styled(
                pet.next_evolution_requirements(),
                Style::default().fg(Color::LightYellow),
            )));
            lines.push(Line::from(format!(
                "Keeper: '{}'  •  Karma: {:+}  •  Memorials: {}  •  Revivals Used: {}/1",
                pet.caretaker_label(),
                app.sanctuary.caretaker_karma,
                app.sanctuary.memorials.len(),
                pet.revival_count
            )));
            lines.push(Line::from(Span::styled(
                "Recent Care Diary:",
                Style::default().fg(Color::White),
            )));
            for entry in pet.action_log.iter().rev().take(4) {
                lines.push(Line::from(Span::styled(
                    format!(" • {entry}"),
                    Style::default().fg(Color::Gray),
                )));
            }
        }
    }
}
