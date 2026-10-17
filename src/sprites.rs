use crate::pet::{EvolutionStage, Pet, PetMood};
use ratatui::style::Color;

const EGG_IDLE: &[&str] = &[
    "      .---.      \n     /     \\     \n    |  o o  |    \n    |   .   |    \n     \\_____/     ",
    "      .---.      \n     /     \\     \n    |  - -  |    \n    |   .   |    \n     \\_____/     ",
];

const EGG_EATING: &[&str] = &[
    "      .---.      \n     / * * \\     \n    |  ^ ^  |    \n    |   O   |    \n     \\_____/     ",
    "      .---.      \n     /     \\     \n    |  > <  |    \n    |   w   |    \n     \\_____/     ",
];

const EGG_SLEEPING: &[&str] = &[
    "      .---.      \n     /     \\     \n    |  - -  | z  \n    |   .   |    \n     \\_____/     ",
    "      .---.   Z  \n     /     \\ z   \n    |  u u  |    \n    |   .   |    \n     \\_____/     ",
];

const EGG_HAPPY: &[&str] = &[
    "     \\.---./     \n     /     \\     \n    |  ^ ^  |    \n    |   v   |    \n     \\_____/     ",
    "      .---.      \n    '/     \\'    \n    |  * *  |    \n    |   v   |    \n     \\_____/     ",
];

const EGG_SICK: &[&str] = &[
    "      .---.      \n     / ~ ~ \\     \n    |  x x  |    \n    |   ~   |    \n     \\_____/     ",
    "      .---.      \n     /     \\     \n    |  @ @  |    \n    |   ^   |    \n     \\_____/     ",
];

const EGG_CODING: &[&str] = &[
    "      .---.      \n     / 0 1 \\     \n    |  o o  |    \n    |  [=]  |    \n     \\_____/     ",
    "      .---.      \n     / 1 0 \\     \n    |  O O  |    \n    |  [_]  |    \n     \\_____/     ",
];

const BABY_IDLE: &[&str] = &[
    "     (\\___/)     \n    (  o.o  )    \n   c(  \"  \" )o   \n     \"- - -\"     ",
    "     (\\___/)     \n    (  o.o  )    \n   o(  \"  \" )c   \n     \"- - -\"     ",
];

const BABY_EATING: &[&str] = &[
    "     (\\___/)     \n    (  ^O^  ) *  \n   c(  \"  \" )o   \n     \"- - -\"     ",
    "     (\\___/)     \n    (  >w<  ) ~  \n   o(  \"  \" )c   \n     \"- - -\"     ",
];

const BABY_SLEEPING: &[&str] = &[
    "     (\\___/)  z  \n    (  -.-  )    \n   c(  \"  \" )c   \n     \"- - -\"     ",
    "     (\\___/) zZ  \n    (  u.u  )    \n   c(  \"  \" )c   \n     \"- - -\"     ",
];

const BABY_HAPPY: &[&str] = &[
    "    \\(\\___/)/    \n    (  ^v^  )    \n   c(  \"  \" )o   \n     \"- - -\"     ",
    "     (\\___/)     \n   \\(  *v*  )/   \n   o(  \"  \" )c   \n     \"- - -\"     ",
];

const BABY_SICK: &[&str] = &[
    "     (\\___/)     \n    (  x_x  ) ~  \n   c(  \"  \" )c   \n     \"- - -\"     ",
    "     (\\___/)     \n    (  @_@  ) !  \n   c(  \"  \" )c   \n     \"- - -\"     ",
];

const BABY_CODING: &[&str] = &[
    "     (\\___/)     \n    (  o_o  )    \n   c( [===] )o   \n     \"- - -\"     ",
    "     (\\___/)     \n    (  O_O  )    \n   o( [###] )c   \n     \"- - -\"     ",
];

const TEEN_IDLE: &[&str] = &[
    "   (\\/)   (\\/)   \n    \\  .---.  /  \n     |( o o )|   \n    /| `---' |\\  \n   /  \"     \"  \\ ",
    "   (\\/)   (\\/)   \n    \\  .---.  /  \n     |( o - )|   \n    /| `---' |\\  \n   \\  \"     \"  / ",
];

const TEEN_EATING: &[&str] = &[
    "   (\\/) * (\\/)   \n    \\  .---.  /  \n     |( ^O^ )|   \n    /| `---' |\\  \n   /  \"     \"  \\ ",
    "   (\\/) ~ (\\/)   \n    \\  .---.  /  \n     |( >w< )|   \n    /| `---' |\\  \n   \\  \"     \"  / ",
];

const TEEN_SLEEPING: &[&str] = &[
    "   (--)   (--) z \n    \\  .---.  /  \n     |( - - )|   \n    /| `---' |\\  \n   /  \"     \"  \\ ",
    "   (--)   (--) Z \n    \\  .---.  /  \n     |( u u )|   \n    /| `---' |\\  \n   /  \"     \"  \\ ",
];

const TEEN_HAPPY: &[&str] = &[
    "  \\(\\/)   (\\/)/  \n    \\  .---.  /  \n     |( ^v^ )|   \n    /| `---' |\\  \n   /  \"     \"  \\ ",
    "   (\\/)   (\\/)   \n   \\ \\ .---. / / \n     |( *v* )|   \n    /| `---' |\\  \n   \\  \"     \"  / ",
];

const TEEN_SICK: &[&str] = &[
    "   (\\/)   (\\/)   \n    \\  .---.  /  \n     |( x_x )| ~ \n    /| `---' |\\  \n   /  \"     \"  \\ ",
    "   (\\/)   (\\/)   \n    \\  .---.  /  \n     |( @_@ )| ! \n    /| `---' |\\  \n   /  \"     \"  \\ ",
];

const TEEN_CODING: &[&str] = &[
    "   (\\/)   (\\/)   \n    \\  .---.  /  \n     |([o_o])|   \n    /|_[===]_|\\  \n   /  \"     \"  \\ ",
    "   (\\/)   (\\/)   \n    \\  .---.  /  \n     |([O_O])|   \n    /|_[###]_|\\  \n   \\  \"     \"  / ",
];

const ADULT_IDLE: &[&str] = &[
    "    _~^~^~_    \n\\) /  o o  \\ (/\n  '_   -   _'  \n  / '-----' \\  ",
    "    _~^~^~_    \n(/ /  o o  \\ \\)\n  '_   v   _'  \n  \\ '-----' /  ",
];

const ADULT_EATING: &[&str] = &[
    "    _~^~^~_    \n\\) /  ^ O  \\ (/\n  '_  [=]  _'  \n  / '-----' \\  ",
    "    _~^~^~_    \n(/ /  > w  \\ \\)\n  '_   ~   _'  \n  \\ '-----' /  ",
];

const ADULT_SLEEPING: &[&str] = &[
    "    _~^~^~_  z \n-) /  - -  \\ (-\n  '_   .   _'  \n  / '-----' \\  ",
    "    _~^~^~_  Z \n-) /  u u  \\ (-\n  '_   o   _'  \n  / '-----' \\  ",
];

const ADULT_HAPPY: &[&str] = &[
    "  \\ _~^~^~_ /  \n\\) /  ^ ^  \\ (/\n  '_   v   _'  \n  / '-----' \\  ",
    "    _~^~^~_    \n\\) /  * *  \\ (/\n  '_  \\_/  _'  \n  \\ '-----' /  ",
];

const ADULT_SICK: &[&str] = &[
    "    _~^~^~_  ~ \n\\) /  x x  \\ (/\n  '_   ~   _'  \n  / '-----' \\  ",
    "    _~^~^~_  ! \n(/ /  @ @  \\ \\)\n  '_   ^   _'  \n  / '-----' \\  ",
];

const ADULT_CODING: &[&str] = &[
    "    _~^~^~_    \n\\) / [o_o] \\ (/\n  '_ [===] _'  \n  / '-----' \\  ",
    "    _~^~^~_    \n(/ / [O_O] \\ \\)\n  '_ [###] _'  \n  \\ '-----' /  ",
];

const ELDER_IDLE: &[&str] = &[
    "    _[===]_    \n\\] / [o_o] \\ [/\n  '==  v  =='  \n  // '---' \\\\  ",
    "    _[===]_    \n[/ / [o_O] \\ \\]\n  '==  -  =='  \n  \\\\ '---' //  ",
];

const ELDER_EATING: &[&str] = &[
    "    _[===]_    \n\\] / [^O^] \\ [/\n  '== [+] =='  \n  // '---' \\\\  ",
    "    _[===]_    \n[/ / [>w<] \\ \\]\n  '== [*] =='  \n  \\\\ '---' //  ",
];

const ELDER_SLEEPING: &[&str] = &[
    "    _[===]_  z \n-] / [-_-] \\ [-\n  '==  .  =='  \n  // '---' \\\\  ",
    "    _[===]_  Z \n-] / [u_u] \\ [-\n  '==  _  =='  \n  // '---' \\\\  ",
];

const ELDER_HAPPY: &[&str] = &[
    "  \\ _[===]_ /  \n\\] / [^*^] \\ [/\n  '== \\_/ =='  \n  // '---' \\\\  ",
    "    _[===]_    \n\\] / [*v*] \\ [/\n  '== \\o/ =='  \n  \\\\ '---' //  ",
];

const ELDER_SICK: &[&str] = &[
    "    _[===]_  ~ \n\\] / [x_x] \\ [/\n  '== ERR =='  \n  // '---' \\\\  ",
    "    _[===]_  ! \n[/ / [@_@] \\ \\]\n  '== NAN =='  \n  // '---' \\\\  ",
];

const ELDER_CODING: &[&str] = &[
    "    _[===]_    \n\\] / [0_1] \\ [/\n  '==[RUST]==' \n  // '---' \\\\  ",
    "    _[===]_    \n[/ / [1_0] \\ \\]\n  '==[CARGO]=='\n  \\\\ '---' //  ",
];

pub fn get_sprite_frames(stage: EvolutionStage, state: PetMood) -> &'static [&'static str] {
    match (stage, state) {
        (EvolutionStage::Egg, PetMood::Idle) => EGG_IDLE,
        (EvolutionStage::Egg, PetMood::Eating) => EGG_EATING,
        (EvolutionStage::Egg, PetMood::Sleeping) => EGG_SLEEPING,
        (EvolutionStage::Egg, PetMood::Happy) => EGG_HAPPY,
        (EvolutionStage::Egg, PetMood::Sick) => EGG_SICK,
        (EvolutionStage::Egg, PetMood::Coding) => EGG_CODING,

        (EvolutionStage::Baby, PetMood::Idle) => BABY_IDLE,
        (EvolutionStage::Baby, PetMood::Eating) => BABY_EATING,
        (EvolutionStage::Baby, PetMood::Sleeping) => BABY_SLEEPING,
        (EvolutionStage::Baby, PetMood::Happy) => BABY_HAPPY,
        (EvolutionStage::Baby, PetMood::Sick) => BABY_SICK,
        (EvolutionStage::Baby, PetMood::Coding) => BABY_CODING,

        (EvolutionStage::Teen, PetMood::Idle) => TEEN_IDLE,
        (EvolutionStage::Teen, PetMood::Eating) => TEEN_EATING,
        (EvolutionStage::Teen, PetMood::Sleeping) => TEEN_SLEEPING,
        (EvolutionStage::Teen, PetMood::Happy) => TEEN_HAPPY,
        (EvolutionStage::Teen, PetMood::Sick) => TEEN_SICK,
        (EvolutionStage::Teen, PetMood::Coding) => TEEN_CODING,

        (EvolutionStage::Adult, PetMood::Idle) => ADULT_IDLE,
        (EvolutionStage::Adult, PetMood::Eating) => ADULT_EATING,
        (EvolutionStage::Adult, PetMood::Sleeping) => ADULT_SLEEPING,
        (EvolutionStage::Adult, PetMood::Happy) => ADULT_HAPPY,
        (EvolutionStage::Adult, PetMood::Sick) => ADULT_SICK,
        (EvolutionStage::Adult, PetMood::Coding) => ADULT_CODING,

        (EvolutionStage::CyberElder, PetMood::Idle) => ELDER_IDLE,
        (EvolutionStage::CyberElder, PetMood::Eating) => ELDER_EATING,
        (EvolutionStage::CyberElder, PetMood::Sleeping) => ELDER_SLEEPING,
        (EvolutionStage::CyberElder, PetMood::Happy) => ELDER_HAPPY,
        (EvolutionStage::CyberElder, PetMood::Sick) => ELDER_SICK,
        (EvolutionStage::CyberElder, PetMood::Coding) => ELDER_CODING,
    }
}

pub fn get_mood_particles(state: PetMood, tick: usize) -> &'static str {
    let idx = tick % 4;
    match state {
        PetMood::Idle => ["  .  *  .  ", "  *  .  *  ", "  .  o  .  ", "  o  .  o  "][idx],
        PetMood::Eating => [" [nom nom] ", " * crunch * ", " [yum yum] ", " ~ tasty! ~ "][idx],
        PetMood::Sleeping => ["   zZz...  ", "  zZzZ...  ", "   Zzz...  ", "  ...zZz   "][idx],
        PetMood::Happy => ["  \\(^o^)/  ", "  * <3 *   ", "  ~(^-^)~  ", "  * \\o/ *  "][idx],
        PetMood::Sick => ["  @ _ @ ~  ", "  ~ ugh ~  ", "  x _ x !  ", "  ~ fever ~"][idx],
        PetMood::Coding => [" fn main() ", " cargo run ", " git push! ", " 0101_RUST "][idx],
    }
}

pub fn render_animated_sprite(stage: EvolutionStage, state: PetMood, tick: usize) -> Vec<String> {
    let frames = get_sprite_frames(stage, state);
    let selected = frames[tick % frames.len()];
    let mut lines = vec![get_mood_particles(state, tick).to_string()];
    lines.extend(selected.lines().map(String::from));
    lines
}

pub fn speech_bubble(pet: &Pet, tick: usize) -> String {
    if pet.is_sick || pet.mood == PetMood::Sick {
        return "💬 \"I don't feel so good... press [h] to heal me!\"".to_string();
    }
    if pet.is_sleeping || pet.mood == PetMood::Sleeping {
        return "💬 \"zZz... dreaming of zero-cost abstractions...\"".to_string();
    }
    if pet.hunger <= 30 {
        return "💬 \"My stomach is rumbling for some crates! [f]\"".to_string();
    }
    if pet.happiness <= 30 {
        return "💬 \"I'm feeling lonely... let's play or pet me! [p/g]\"".to_string();
    }
    match pet.mood {
        PetMood::Eating => "💬 \"Mmm, delicious memory-safe snacks!\"".to_string(),
        PetMood::Coding => "💬 \"Borrow checker approves this commit! 🚀\"".to_string(),
        PetMood::Happy => "💬 \"Life is great when the code compiles!\"".to_string(),
        PetMood::Idle => {
            let idle_lines = [
                "💬 \"Ready to write some blazingly fast Rust!\"",
                "💬 \"Make a git commit to feed me XP!\"",
                "💬 \"Press [g] to catch falling crates and bugs!\"",
            ];
            idle_lines[tick % idle_lines.len()].to_string()
        }
        PetMood::Sick | PetMood::Sleeping => {
            "( resting quietly... )".to_string()
        }
    }
}

pub fn stage_color(stage: EvolutionStage) -> Color {
    match stage {
        EvolutionStage::Egg => Color::White,
        EvolutionStage::Baby => Color::LightYellow,
        EvolutionStage::Teen => Color::LightRed,
        EvolutionStage::Adult => Color::Red,
        EvolutionStage::CyberElder => Color::Cyan,
    }
}

pub fn mood_color(state: PetMood) -> Color {
    match state {
        PetMood::Idle => Color::Gray,
        PetMood::Eating => Color::Yellow,
        PetMood::Sleeping => Color::Blue,
        PetMood::Happy => Color::Green,
        PetMood::Sick => Color::Magenta,
        PetMood::Coding => Color::LightCyan,
    }
}
