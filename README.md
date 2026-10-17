# 🌿 KOMO — Pocket Soul & Living Terminal Companion

[![CI](https://github.com/NojinNojs/komo/actions/workflows/ci.yml/badge.svg)](https://github.com/NojinNojs/komo/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](./LICENSE)

**KOMO** (*Komorebi Pocket Soul*) is a terminal virtual companion built with `ratatui` and `crossterm`. Your companion lives inside a centered retro handheld LCD frame, perceives its environment through four senses (`Sight`, `Hearing`, `Feeling`, `Speech`), looks left, right, and directly at you, addresses you by your chosen name, and enforces binding consequences if neglected.

See [**GUIDE.md**](./GUIDE.md) for the complete Keeper's Handbook.

---

## Core Features

1. **40×22 TrueColor Pixel Art with 3-Way Gaze & Full-Body Poses**:
   - **Three Gaze Directions (`Left ◀`, `Center ●`, `Right ▶`)**: Instead of only flipping left and right, your companion turns to look straight at you (`Center ●`) with two front-facing eyes when you type, speak with it, or when it pauses in the center of the room.
   - **Full-Body Posture & Expressions**:
     - **Sleeping Pose**: Lowers and curls onto a woven pixel sleeping mat on the habitat floor with slow breathing and rising `z Z` particles.
     - **Hungry / Starving Slouch**: Drooping ears, watery eyes (`A`), and stomach-rumble waves (`G`) when Hungry Hearts drop low.
     - **Full & Satisfied**: Contented curved eyes, rosy cheeks (`P`), and a rounded belly after eating.
     - **Fever & Toothache**: Half-lidded eyes, flushed fever cheeks (`R`), and cold sweat drops.

2. **Four Living Senses & Window Diorama**:
   - `👁 Sight`: Watches which bezel icon you highlight, tracks the room lamp, and observes the weather outside the sanctuary window (`DAY`, `BREEZE`, `RAIN`, `NIGHT`).
   - `👂 Hearing`: Reacts to your keyboard rhythm—perks an ear at steady typing, startles at rapid button-mashing, and listens to rain or crickets when quiet.
   - `🫳 Feeling`: Reports internal state (stomach cramps, warmth, molar pain, fatigue, and attachment to you).
   - `💬 Speech`: Grounded, natural English dialogue without clichés or AI-slop.

3. **Custom Caretaker Name (`[O]` / `--call-me`)**:
   - Choose what your companion calls you during Onboarding (`Step 3 of 3` via `Tab`), in-game by pressing `[O]`, or from the CLI via `komo --call-me Aqsan`.

4. **Real, Binding Consequences & The Memorial Hall**:
   - **Action Abuse Penalties**: Force-feeding a full companion (`4/4 ♥`) causes stomach cramps (`-Health`, `-Bond`, `+1 Care Mistake`). Scolding (`[D]`) when not misbehaving damages trust and makes your companion turn away from you. Three sweet snacks in a row cause a **Toothache** requiring Medicine (`[H]`).
   - **One-Time Phoenix Tear (`[H]`)**: A fallen companion can only be revived **once in its entire lifetime** (`revival_count: 0 -> 1`), costing up to `50 coins` and adding `+3 Care Mistakes`. A second death is permanent.
   - **Mandatory 3-Step Burial Vigil (`[B]`)**: You cannot adopt a new companion (`[N]`) while a fallen companion lies unburied. You must complete the 3-step Burial Vigil Rite (`[B]`) to lay them to rest in the permanent **Sanctuary Memorial Hall** and accept the `-15 Caretaker Karma` penalty.

---

## Controls

| Key | Action |
| :--- | :--- |
| `←` / `→` then `Enter` | Select and open an icon on the 8-icon LCD Bezel |
| `W` / `Space` | Warm unhatched egg (`+20%`) or cuddle hatched companion |
| `F` / `M` | Feed favorite meal (`F`) or open the 5-item Pantry (`M`) |
| `Y` | Open Heart-to-Heart Dialogue screen (`1` / `2` to reply) |
| `P` / `G` | Play Left/Right match (`P`) or open the 3-in-1 Retro Arcade (`G`) |
| `E` | Run a discipline & strength training drill |
| `S` / `K` | Toggle Sleep on the woven mat (`S`) / Toggle Room Lamp (`K`) |
| `C` | Wash habitat floor, brush teeth, and take a warm bath |
| `D` / `T` | Scold spoiled call (`D`) / Praise & comfort crying call (`T`) |
| `H` / `B` | Herbal Medicine or 1-time Phoenix Tear (`H`) / 3-step Burial Vigil (`B`) |
| `O` / `R` | Set what your companion calls you (`O`) / Rename companion (`R`) |
| `I` / `L` / `N` | 4-page Status Meter (`I`) / Sanctuary Roster (`L`) / Adopt (`N`) |
| `U` / `Tab` / `?` | Toggle 8-bit audio (`U`) / Cycle shell color (`Tab`) / Help (`?`) |

---

## Installation & CLI Usage

```bash
cargo build --release
cp target/release/mochigotchi ~/.local/bin/komo
```

```bash
komo                                  # Launch interactive handheld TUI
komo --call-me Aqsan                  # Set how your companion addresses you
komo --status                         # Print live vitals, 4 senses, and 40x22 sprite
komo --roster                         # List Sanctuary companions & Memorial Hall
komo --vigil                          # Advance 1 step of the Burial Vigil Rite
```

---

## License

Released under the [MIT License](./LICENSE).
