# KOMO — a small soul that lives in your terminal

[![CI](https://github.com/NojinNojs/komo/actions/workflows/ci.yml/badge.svg)](https://github.com/NojinNojs/komo/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](./LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.87%2B-orange.svg)](/Cargo.toml)

`komo --status`, run for real:

```
Hoshi — 🦊 Kitsune Nine Epic ★★★ | Stage: 🥚 Egg | Mood: 💖 Joyful | Lv.1 (0 XP) |
Rank: 🌱 B-Rank (Cozy Friend) | Weight: 5g | Hungry: ♥♥♥· 3/4 (65%) • Happy:
♥♥♥· 3/4 (80%) • Discipline: 25% • Energy: 90% • Health: 100% • Clean: 100% •
Bond: 10% | Mistakes: 0 | Coins: 0
💭 👁 Sight (Looking at You ●) • Soft amber light filtering through the shell.
💬 "A steady pulse beats beneath the shell."
                ▄▄        ▄▄
               ▄▄██████████▄▄
            ▄██████████████████▄
           ██████████████████████
          ████████████████████████
```

**KOMO** (*Komorebi Pocket Soul*) is a terminal virtual companion built with
`ratatui` and `crossterm`. Your companion hatches as an egg, grows through five
stages, turns to look straight at you when you type, reacts to your keyboard
rhythm through four senses — and holds you accountable.

The rule here is simple: **neglect has real consequences.**

* A fallen companion can only be revived **once, ever**, with the Phoenix Tear.
Death a second time is permanent.
* You cannot adopt again while a fallen companion lies unburied. The [B] Burial
Vigil is a mandatory three-step rite, and it leaves a permanent record in the
Memorial Hall.
* Force-feed a full companion and they get stomach cramps. Scold them when they
did nothing wrong and they turn away from you. Three sweet snacks in a row
cause a toothache.

Your keeper name is woven into every spoken line, sense, and epitaph. And your
*Keeper Karma* follows you across companions: future hatchlings sense it.

> [!TIP]
> People who skip the guide usually meet consequence #3 the hard way. The
> full handbook lives in [GUIDE.md](./GUIDE.md).

## Features

* **A pet-shaped device, not a dashboard.** A fixed retro-handheld frame with
  a TrueColor 40×22 half-block LCD screen, an 8-icon bezel, and 10 species ×
  5 evolution stages of 16-bit-style pixel art with full-body poses (sleep,
  slouch, fever) and 3-way gaze.
* **Four senses running live.** `👁 Sight` tracks the window weather and which
  bezel icon you're hovering; `👂 Hearing` perks an ear at steady typing and
  startles when you spam; `🫳 Feeling` reports stomach cramps and warm hands;
  `💬 Speech` answers in grounded, first-person English — no canned fluff.
* **Heart-to-Heart dialogue.** Press [Y], pick a reply, and your choice moves
  bond, happiness, and coins. Speak thoughtfully; they listen.
* **A 3-in-1 retro arcade.** Snack Catcher with combo multipliers, Mind
  Reader, and Dash Runner — all feeding XP and coins back into real care.
* **Pantry of 5 snacks and 2 meals.** Choose wisely: sweets are loved, but
  KOMO will remember what sugar does to molars.
* **Living environment.** Animated window diorama (DAY / BREEZE / RAIN /
  NIGHT), woven sleeping mat, habitat floor that gets dirty if you're late
  with the [C] wash, and lamps that actually matter at night.
* **Sound without the bloat.** 8-bit chiptune square-wave effects synthesized
  in pure Rust at 22,050 Hz — no heavy audio crates, no streaming.
* **Sanctuary roster.** Raise multiple companions, switch active ones with
  [L] or `komo --switch`, and keep every death on record in the Memorial Hall.

## Installation

Rust 1.87+ is required. No other setup, no accounts, no network calls.

```bash
git clone https://github.com/NojinNojs/komo.git
cd komo
cargo build --release
cp target/release/mochigotchi ~/.local/bin/komo   # Short name
```

Or install straight from the repo:

```bash
cargo install --path .   # Installs as 'mochigotchi'
```

## First 3 Minutes

```bash
komo                       # Launch the handheld LCD interface
```

1. Pick a species (Step 1), a shell color (Step 2), and tell it what to call
   you (Step 3, press Tab) — for example `Boss`, `Keeper`, or `Captain`.
2. Warm the egg with [W]. It incubates +20% per 3 minutes; patience matters
   not only in the egg stage.
3. Once hatched, try [F] to feed, [Y] to talk, [P] to play, and [U] for
   chiptune sound. Notice how it reacts differently to steady typing vs.
   button spam.

## Command-Line Usage

| Command | Effect |
|:---|:---|
| `komo` | Open the interactive handheld LCD interface |
| `komo --call-me <title>` | Set what it calls you (woven into all speech) |
| `komo --name <name>` | Set or rename the companion |
| `komo --species fox` | Spawn as cat, pup, bun, pengu, dragon, wyrm, fox, phoenix, uni, or slime |
| `komo --theme pink` | Shell palette: pink, matcha, blue, lavender, gold |
| `komo --status` | Print live vitals, senses, and the 40×22 sprite |
| `komo --roster` | List companions and the Memorial Hall |
| `komo --adopt` / `--switch <id name>` | Add companions to the Sanctuary, or pick one as active |
| `komo --vigil` | Advance one step of the Burial Vigil |
| `komo --feed / --warm / --snack` | Quick care actions, same as in-game |
| `komo --play / --pet / --talk` | Quick emotional actions |
| `komo --train / --sleep / --light` | Discipline & body state |
| `komo --scold / --clean / --heal` | Corrective actions (scold involves consequences) |
| `komo --reset` | Clear saves for a fresh start |
| `komo -V` | Print version |

## Under the Hood

* **Pixel pipeline:** 40×22 bitmaps → pose & expression transforms → gaze
  layer → palette mapping → half-block terminal glyphs (`▀▄█`). Transparent
  pixels stay background-free so your terminal theme shows through.
* **Persistence:** every save is written twice — an atomic JSON file + SQLite
  (via `rusqlite` bundled). Legacy single-pet saves transparently upgrade to
  a full Sanctuary roster on load.
* **Audio:** pure-Rust RIFF WAV synthesis (16-bit mono, square waves, soft
  attack/decay envelopes), cached in `std::env::temp_dir()`, played in the
  background by `pw-play` / `paplay` / `aplay` — mute via [U].
* **Architecture gate:** the whole `src/` tree is 80+ atomic modules, none
  larger than 200 lines — enforced by `tests/architecture_gate.rs`, so the
  modular structure can't rot.

If you want to work on it:

```bash
cargo fmt --check                          # zero diffs
cargo clippy --all-targets -- -D warnings  # zero warnings
cargo test                                 # 29 tests pass
```

## License

Released under the [MIT License](./LICENSE) — © 2026 Muhammad Raffi Aqsan
(NojinNojs). Use it, fork it, ship it. Just be kind to whatever hatches in
your terminal.
