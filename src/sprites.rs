use crate::pet::{EvolutionStage, Pet, PetMood, PetSpecies, ShellTheme};
use ratatui::style::Color;

const EGG_IDLE_FRAMES: &[&str] = &[
    "    .---.    \n   / o o \\   \n  |  ~~~  |  \n   \\_____/   \n  [=======]  ",
    "    .---.    \n   / ^ ^ \\   \n  |  ~~~  |  \n   \\_____/   \n  [=======]  ",
];

const EGG_HAPPY_FRAMES: &[&str] = &[
    "  * .---. *  \n   / ^o^ \\   \n  |  ~~~  |  \n   \\_____/   \n  [=======]  ",
    "  ~ .---. ~  \n   / *o* \\   \n  |  ~~~  |  \n   \\_____/   \n  [=======]  ",
];

const EGG_SLEEPING_FRAMES: &[&str] = &[
    "    .---. zZ \n   / - - \\   \n  |  ~~~  |  \n   \\_____/   \n  [=======]  ",
    "    .---.  Z \n   / - - \\z  \n  |  ~~~  |  \n   \\_____/   \n  [=======]  ",
];

// MochiCat Sprites
const CAT_BABY_IDLE: &[&str] = &[
    "   /\\_/\\   \n  ( o.o )  \n   > ^ <   ",
    "   /\\_/\\   \n  ( -.- )  \n   > ^ <   ",
];
const CAT_BABY_EATING: &[&str] = &[
    "   /\\_/\\   \n  ( ^O^ )🐟\n   > ^ <   ",
    "   /\\_/\\   \n  ( ^w^ )~ \n   > ^ <   ",
];
const CAT_BABY_SLEEPING: &[&str] = &[
    "   /\\_/\\ zZ\n  ( -.- )  \n   (   )   ",
    "   /\\_/\\  Z\n  ( -w- )z \n   (   )   ",
];
const CAT_BABY_HAPPY: &[&str] = &[
    " * /\\_/\\ * \n  ( ^ω^ )  \n  /> ♥ <\\  ",
    " ~ /\\_/\\ ~ \n  ( >ω< )  \n  \\> ♥ </  ",
];
const CAT_BABY_SICK: &[&str] = &[
    "   /\\_/\\   \n  ( x_x )💧\n   > ~ <   ",
    "   /\\_/\\   \n  ( @_@ )💧\n   > ~ <   ",
];
const CAT_BABY_TRAINING: &[&str] = &[
    "   /\\_/\\ ! \n  ( >_> )💨\n  />   <\\  ",
    " ! /\\_/\\   \n 💨( <_< )  \n  />   <\\  ",
];

const CAT_TEEN_IDLE: &[&str] = &[
    "   /\\___/\\   \n  (  o.o  )  \n  (  =^=  )  \n   (\")_(\")   ",
    "   /\\___/\\   \n  (  o.o  )  \n  (  =^=  )~ \n   (\")_(\")   ",
];
const CAT_TEEN_EATING: &[&str] = &[
    "   /\\___/\\   \n  (  ^O^  )🐟\n  (  =^=  )  \n   (\")_(\")   ",
    "   /\\___/\\   \n  (  ^w^  )✨\n  (  =^=  )  \n   (\")_(\")   ",
];
const CAT_TEEN_SLEEPING: &[&str] = &[
    "   /\\___/\\ zZ\n  (  -.-  )  \n  (  =^=  )  \n   (\")_(\")   ",
    "   /\\___/\\  Z\n  (  -w-  )z \n  (  =^=  )  \n   (\")_(\")   ",
];
const CAT_TEEN_HAPPY: &[&str] = &[
    " * /\\___/\\ * \n  (  ^ω^  )  \n ~(  ♥^♥  )~ \n   (\")_(\")   ",
    " ~ /\\___/\\ ~ \n  (  >ω<  )  \n *(  ♥^♥  )* \n   (\")_(\")   ",
];
const CAT_TEEN_SICK: &[&str] = &[
    "   /\\___/\\   \n  (  T_T  )💧\n  (  =~=  )  \n   (\")_(\")   ",
    "   /\\___/\\   \n  (  x_x  )💧\n  (  =~=  )  \n   (\")_(\")   ",
];
const CAT_TEEN_TRAINING: &[&str] = &[
    "   /\\___/\\ ! \n  (  •̀_•́  )ง \n  (  =^=  )  \n   (\")_(\")   ",
    " ! /\\___/\\   \n ง(  •̀_•́  )  \n  (  =^=  )💨\n   (\")_(\")   ",
];

const CAT_ADULT_IDLE: &[&str] = &[
    "    /\\_____/\\    \n   /  o   o  \\   \n  ( ==  ^  == )  \n   )         (   \n  (           )  \n ( (  )   (  ) ) \n(__(__)___(__)__)",
    "    /\\_____/\\    \n   /  -   -  \\   \n  ( ==  ^  == )  \n   )         ( ~ \n  (           )  \n ( (  )   (  ) ) \n(__(__)___(__)__)",
];
const CAT_ADULT_HAPPY: &[&str] = &[
    "  * /\\_____/\\ *  \n   /  ^   ^  \\   \n  ( ==  ω  == )  \n   )   ♥♥♥   (   \n  (           )  \n ( (  )   (  ) ) \n(__(__)___(__)__)",
    "  ~ /\\_____/\\ ~  \n   /  >   <  \\   \n  ( ==  ω  == )  \n   )   ♥♥♥   ( ~ \n  (           )  \n ( (  )   (  ) ) \n(__(__)___(__)__)",
];

const CAT_ELDER_IDLE: &[&str] = &[
    "  ✨ /\\_____/\\ ✨ \n   /  *   *  \\   \n  ( ==  ω  == )  \n   )  ~★~★~  (   \n  (   (___)   )  ",
    "  ★  /\\_____/\\  ★ \n   /  ^   ^  \\   \n  ( ==  ω  == )  \n   )  ★~★~★  (   \n  (   (___)   )  ",
];

// BobaPup Sprites
const PUP_BABY_IDLE: &[&str] = &[
    "  / \\__/ \\ \n (  •ᴥ•  ) \n  /     \\  ",
    "  / \\__/ \\ \n (  •ᴥ•  )~ \n  /     \\  ",
];
const PUP_BABY_HAPPY: &[&str] = &[
    " * /\\__/\\ *\n (  ^ᴥ^  ) \n  /  ♥  \\~ ",
    " ~ /\\__/\\ ~\n (  >ᴥ<  ) \n ~/  ♥  \\  ",
];
const PUP_BABY_EATING: &[&str] = &[
    "  / \\__/ \\ \n (  ^ᴥ^  )🦴\n  /     \\  ",
    "  / \\__/ \\ \n (  •ᴥ•  )✨\n  /     \\  ",
];
const PUP_BABY_SLEEPING: &[&str] = &[
    "  / \\__/ \\zZ\n (  -ᴥ-  ) \n  (_____)  ",
    "  / \\__/ \\ Z\n (  -ᴥ-  )z\n  (_____)  ",
];
const PUP_BABY_SICK: &[&str] = &[
    "  / \\__/ \\ \n (  xᴥx  )💧\n  /  ~  \\  ",
    "  / \\__/ \\ \n (  TᴥT  )💧\n  /  ~  \\  ",
];
const PUP_BABY_TRAINING: &[&str] = &[
    "  / \\__/ \\ !\n (  •̀ᴥ•́  )💨\n  /     \\  ",
    "! / \\__/ \\ \n💨(  •̀ᴥ•́  ) \n  /     \\  ",
];

// CloudBun Sprites
const BUN_BABY_IDLE: &[&str] = &[
    "   (\\_/)   \n  ( •.• )  \n  / >☁< \\  ",
    "   (\\_/)   \n  ( -.- )  \n  / >☁< \\  ",
];
const BUN_BABY_HAPPY: &[&str] = &[
    " * (\\_/) * \n  ( ^.^ )  \n  / >♥< \\  ",
    " ~ (\\_/) ~ \n  ( >.< )  \n  / >♥< \\  ",
];
const BUN_BABY_EATING: &[&str] = &[
    "   (\\_/)   \n  ( ^O^ )🥕\n  / > < \\  ",
    "   (\\_/)   \n  ( ^w^ )✨\n  / > < \\  ",
];
const BUN_BABY_SLEEPING: &[&str] = &[
    "   (\\_/) zZ\n  ( -.- )  \n  (_____)  ",
    "   (\\_/)  Z\n  ( -w- )z \n  (_____)  ",
];
const BUN_BABY_SICK: &[&str] = &[
    "   (\\_/)   \n  ( x.x )💧\n  / >~< \\  ",
    "   (\\_/)   \n  ( T.T )💧\n  / >~< \\  ",
];
const BUN_BABY_TRAINING: &[&str] = &[
    "   (\\_/) ! \n  ( •̀.•́ )💨\n  / > < \\  ",
    " ! (\\_/)   \n 💨( •̀.•́ )  \n  / > < \\  ",
];

// StarPengu Sprites
const PENGU_BABY_IDLE: &[&str] = &[
    "    .--.    \n   |o_o |   \n   |:_/ |   \n  //   \\ \\  \n (|     | ) ",
    "    .--.    \n   |-_- |   \n   |:_/ |   \n  //   \\ \\  \n (|     | ) ",
];
const PENGU_BABY_HAPPY: &[&str] = &[
    "  * .--. *  \n   |^v^ |   \n  /|:_/ |\\  \n   | ♥ |    \n  /     \\   ",
    "  ~ .--. ~  \n   |>v< |   \n  \\|:_/ |/  \n   | ♥ |    \n  /     \\   ",
];
const PENGU_BABY_EATING: &[&str] = &[
    "    .--.    \n   |^O^ |🫐 \n   |:_/ |   \n  //   \\ \\  ",
    "    .--.    \n   |^v^ |✨ \n   |:_/ |   \n  //   \\ \\  ",
];
const PENGU_BABY_SLEEPING: &[&str] = &[
    "    .--. zZ \n   |-_- |   \n   |:_/ |   \n  (______)  ",
    "    .--.  Z \n   |-_- |z  \n   |:_/ |   \n  (______)  ",
];
const PENGU_BABY_SICK: &[&str] = &[
    "    .--.    \n   |x_x |💧 \n   |:_/ |   \n  (______)  ",
    "    .--.    \n   |T_T |💧 \n   |:_/ |   \n  (______)  ",
];
const PENGU_BABY_TRAINING: &[&str] = &[
    "    .--. !  \n   |•̀v•́ |💨 \n  /|:_/ |\\  \n  /     \\   ",
    " !  .--.    \n 💨|•̀v•́ |   \n  \\|:_/ |/  \n  /     \\   ",
];

pub fn get_sprite_frames(
    species: PetSpecies,
    stage: EvolutionStage,
    state: PetMood,
) -> &'static [&'static str] {
    if stage == EvolutionStage::Egg {
        return match state {
            PetMood::Sleeping => EGG_SLEEPING_FRAMES,
            PetMood::Happy | PetMood::Eating | PetMood::Training => EGG_HAPPY_FRAMES,
            _ => EGG_IDLE_FRAMES,
        };
    }

    match species {
        PetSpecies::MochiCat => match (stage, state) {
            (EvolutionStage::CyberElder, _) => CAT_ELDER_IDLE,
            (EvolutionStage::Adult, PetMood::Happy) => CAT_ADULT_HAPPY,
            (EvolutionStage::Adult, PetMood::Idle) => CAT_ADULT_IDLE,
            (EvolutionStage::Teen, PetMood::Idle) => CAT_TEEN_IDLE,
            (EvolutionStage::Teen | EvolutionStage::Adult, PetMood::Eating) => CAT_TEEN_EATING,
            (EvolutionStage::Teen | EvolutionStage::Adult, PetMood::Sleeping) => CAT_TEEN_SLEEPING,
            (EvolutionStage::Teen, PetMood::Happy) => CAT_TEEN_HAPPY,
            (EvolutionStage::Teen | EvolutionStage::Adult, PetMood::Sick) => CAT_TEEN_SICK,
            (EvolutionStage::Teen | EvolutionStage::Adult, PetMood::Training) => CAT_TEEN_TRAINING,
            (_, PetMood::Idle) => CAT_BABY_IDLE,
            (_, PetMood::Eating) => CAT_BABY_EATING,
            (_, PetMood::Sleeping) => CAT_BABY_SLEEPING,
            (_, PetMood::Happy) => CAT_BABY_HAPPY,
            (_, PetMood::Sick) => CAT_BABY_SICK,
            (_, PetMood::Training) => CAT_BABY_TRAINING,
        },
        PetSpecies::BobaPup => match state {
            PetMood::Idle => PUP_BABY_IDLE,
            PetMood::Eating => PUP_BABY_EATING,
            PetMood::Sleeping => PUP_BABY_SLEEPING,
            PetMood::Happy => PUP_BABY_HAPPY,
            PetMood::Sick => PUP_BABY_SICK,
            PetMood::Training => PUP_BABY_TRAINING,
        },
        PetSpecies::CloudBun => match state {
            PetMood::Idle => BUN_BABY_IDLE,
            PetMood::Eating => BUN_BABY_EATING,
            PetMood::Sleeping => BUN_BABY_SLEEPING,
            PetMood::Happy => BUN_BABY_HAPPY,
            PetMood::Sick => BUN_BABY_SICK,
            PetMood::Training => BUN_BABY_TRAINING,
        },
        PetSpecies::StarPengu => match state {
            PetMood::Idle => PENGU_BABY_IDLE,
            PetMood::Eating => PENGU_BABY_EATING,
            PetMood::Sleeping => PENGU_BABY_SLEEPING,
            PetMood::Happy => PENGU_BABY_HAPPY,
            PetMood::Sick => PENGU_BABY_SICK,
            PetMood::Training => PENGU_BABY_TRAINING,
        },
    }
}

pub fn get_mood_particles(state: PetMood, tick: usize) -> &'static str {
    match (state, tick % 2) {
        (PetMood::Happy, 0) => "✨  💖  ✨  🌸  ✨",
        (PetMood::Happy, _) => "🌸  ✨  💖  ✨  🌸",
        (PetMood::Eating, 0) => "🍴  😋  🎵  🍰  🍴",
        (PetMood::Eating, _) => "🎵  🍰  🍴  😋  🎵",
        (PetMood::Sleeping, 0) => "🌙   z   Z   zZ  ⭐",
        (PetMood::Sleeping, _) => "⭐   Z   zZ  z   🌙",
        (PetMood::Sick, 0) => "💧  🤒  💊  🌧️  💧",
        (PetMood::Sick, _) => "🌧️  💊  💧  🤒  🌧️",
        (PetMood::Training, 0) => "🔥  ⚡  💪  ⭐  🔥",
        (PetMood::Training, _) => "⭐  💪  🔥  ⚡  ⭐",
        (PetMood::Idle, 0) => "☁️   ~   ✨   ~  ☁️",
        (PetMood::Idle, _) => "✨   ~   ☁️   ~  ✨",
    }
}

pub fn render_animated_sprite(pet: &Pet, tick: usize) -> Vec<String> {
    let frames = get_sprite_frames(pet.species, pet.stage, pet.mood);
    let frame = frames[tick % frames.len()];
    let mut lines = Vec::new();

    lines.push(format!("   {}   ", get_mood_particles(pet.mood, tick)));
    lines.push(String::new());
    for line in frame.lines() {
        lines.push(line.to_string());
    }

    if pet.poop_count > 0 {
        let messes = "💩 ".repeat(pet.poop_count.min(5) as usize);
        lines.push(format!("  {} (needs cleaning!)", messes.trim()));
    }

    lines
}

pub fn speech_bubble(pet: &Pet, tick: usize) -> String {
    if pet.stage == EvolutionStage::Egg {
        return if pet.incubation >= 75 {
            "💬 \"*crack* *wiggle* Ready to hatch soon!\"".to_string()
        } else {
            "💬 \"*warmth* Press [1/F] to warm the egg!\"".to_string()
        };
    }

    if pet.is_sleeping || pet.mood == PetMood::Sleeping {
        return "💬 \"Zzz... dreaming of sweet snacks... 🌙\"".to_string();
    }

    if pet.is_sick || pet.mood == PetMood::Sick {
        return "💬 \"Ugh... my tummy hurts... need medicine [6]! 💊\"".to_string();
    }

    if pet.poop_count > 0 {
        return "💬 \"Eep! My room is messy... please clean [5]! 🧹\"".to_string();
    }

    if pet.hunger >= 65 {
        return format!(
            "💬 \"I'm super hungry! Could I have {}? [1]\"",
            pet.species.favorite_snack()
        );
    }

    if pet.energy <= 25 {
        return "💬 \"*yawn* So sleepy... time for a nap [4]? 💤\"".to_string();
    }

    let chatter = [
        format!(
            "💬 \"Yay! {} loves spending time with you! 💖\"",
            pet.name
        ),
        format!(
            "💬 \"Mmm, thinking about {}~ ✨\"",
            pet.species.favorite_snack()
        ),
        "💬 \"Let's play the Snack Catcher mini-game [G]! 🎮\"".to_string(),
        "💬 \"Pat-pats [3] make our bond grow stronger! 🌸\"".to_string(),
    ];

    chatter[(tick / 3) % chatter.len()].clone()
}

pub fn theme_color(theme: ShellTheme) -> Color {
    match theme {
        ShellTheme::StrawberryPink => Color::LightMagenta,
        ShellTheme::MatchaGreen => Color::LightGreen,
        ShellTheme::SkyBlue => Color::LightCyan,
        ShellTheme::LavenderDream => Color::Magenta,
        ShellTheme::SunsetGold => Color::Yellow,
    }
}

pub fn stage_color(stage: EvolutionStage) -> Color {
    match stage {
        EvolutionStage::Egg => Color::White,
        EvolutionStage::Baby => Color::LightGreen,
        EvolutionStage::Teen => Color::LightCyan,
        EvolutionStage::Adult => Color::LightMagenta,
        EvolutionStage::CyberElder => Color::Yellow,
    }
}

pub fn mood_color(state: PetMood) -> Color {
    match state {
        PetMood::Idle => Color::Cyan,
        PetMood::Eating => Color::Green,
        PetMood::Sleeping => Color::Blue,
        PetMood::Happy => Color::LightMagenta,
        PetMood::Sick => Color::LightRed,
        PetMood::Training => Color::Yellow,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_species_stages_and_moods_have_multi_frame_sprites() {
        for &species in PetSpecies::all() {
            for &stage in EvolutionStage::all() {
                for &mood in PetMood::all() {
                    let frames = get_sprite_frames(species, stage, mood);
                    assert!(
                        frames.len() >= 2,
                        "Expected at least 2 frames for {:?}/{:?}/{:?}",
                        species,
                        stage,
                        mood
                    );
                    for frame in frames {
                        assert!(!frame.trim().is_empty());
                    }
                }
            }
        }
    }

    #[test]
    fn test_render_animated_sprite_and_speech_bubble() {
        let mut pet = Pet::new("Mochi");
        let lines_t0 = render_animated_sprite(&pet, 0);
        let lines_t1 = render_animated_sprite(&pet, 1);
        assert!(!lines_t0.is_empty());
        assert_ne!(lines_t0, lines_t1);

        let egg_speech = speech_bubble(&pet, 0);
        assert!(egg_speech.contains("warm") || egg_speech.contains("hatch"));

        pet.stage = EvolutionStage::Baby;
        pet.poop_count = 2;
        let dirty_lines = render_animated_sprite(&pet, 0);
        assert!(dirty_lines.iter().any(|l| l.contains("💩")));
        assert!(speech_bubble(&pet, 0).contains("clean"));
    }

    #[test]
    fn test_colors_and_particles() {
        for &theme in ShellTheme::all() {
            let _ = theme_color(theme);
        }
        for &stage in EvolutionStage::all() {
            let _ = stage_color(stage);
        }
        for &mood in PetMood::all() {
            let _ = mood_color(mood);
            assert!(!get_mood_particles(mood, 0).is_empty());
            assert_ne!(get_mood_particles(mood, 0), get_mood_particles(mood, 1));
        }
    }
}
