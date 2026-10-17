# 🍡 Mochigotchi v1.0.0 — Minimalist Retro Handheld V-Pet & 24×16 TrueColor Pixel Art Edition

[![CI](https://github.com/NojinNojs/mochigotchi/actions/workflows/ci.yml/badge.svg)](https://github.com/NojinNojs/mochigotchi/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](./LICENSE)

> **Your Cozy Pocket-Pet Sanctuary in the Terminal.**  
> Play from anywhere in your terminal with `mochigotchi`, `mochi`, or `tamagotchi`! Features a **24×16 TrueColor Half-Block Pixel Art Engine (`▀`, `▄`, `█`)**, a **Minimalist Step-by-Step Retro Handheld LCD Interface**, and reverse-engineered authentic **Bandai Tamagotchi** mechanics (7-Icon Bezel, Hungry/Happy Hearts, Meal vs. Sweet Snack Weight & Toothache system, Room Light switch, Discipline Scold/Praise, Care Mistakes & Care Ranks).

---

## ✨ Highlights (EN)

1. **24×16 TrueColor Half-Block Pixel Art Engine (`▀`, `▄`, `█`)**:
   - Every companion is rendered as a genuine **24×16 pixel bitmap** using Unicode half-blocks (`▀` upper half, `▄` lower half, `█` full block) where two vertical pixels pack into one terminal cell (`1:1` square pixel ratio).
   - Hand-crafted pixel silhouettes across all **5 evolution stages** (`Egg` -> `Baby` -> `Teen` -> `Adult` -> `Starlight Elder`) and **6 moods** (`Idle`, `Eating`, `Sleeping`, `Happy`, `Sick`, `Training`), with dedicated eye-shine/pupil channels (`E`/`K`) so white fur, snow bellies, and clouds never flicker on blink.

2. **Minimalist, Step-by-Step Retro Handheld LCD UI**:
   - Centered **74×25 Retro Handheld V-Pet Frame** (`BorderType::Double` casing + `BorderType::Rounded` inner LCD, minimum viewport `68 × 22`).
   - **Step-by-Step Onboarding**: `Step 1 of 3` (Select Companion with live pixel preview) -> `Enter` -> switches screen to `Step 2 of 3` (Select Shell Color) -> `Enter` -> switches screen to `Step 3 of 3` (Name & Hatch).
   - **Focused Single-Screen Habitat**: The main LCD screen stays clean and uncluttered; opening the **4-Page Status Meter (`[I]`)**, **Pantry (`[M]`)**, **Sanctuary Roster (`[L]`)**, **Arcade (`[G]`)**, or **Keys Guide (`[?]`)** transitions step-by-step inside the LCD screen.

3. **Reverse-Engineered Authentic Bandai Tamagotchi Mechanics**:
   - **7 Interactive LCD Bezel Icons**: `🍙Food`, `💡Light`, `🎮Game`, `💉Meds`, `🚽Bath`, `📏Meter`, `📢Scold` (navigate with Button A `←`/`→`, confirm with Button B `Enter`, cancel with Button C `Esc`).
   - **Meals vs. Sweet Snacks, Weight (`g`) & Toothache (`🦷`)**: Meals fill Hungry Hearts (`♥♥♥♥ 4/4`) and add `+1..2g` weight; Sweet Snacks fill Happy Hearts and add `+2..3g` weight, but **3 sweet snacks in a row** trigger a **Toothache (`🦷`)** requiring **Medicine (`💉 [H]`)**! Play (`[P]`) or Train (`[E]`) to burn `-2g` weight.
   - **Room Light Switch (`💡 [K]`), Discipline (`📢 [D]` / `[T]`), Care Mistakes & Care Ranks**: Turn the light OFF (`[K]`) when your pet sleeps (`[S]`), scold (`[D]`) mischievous attention calls, praise (`[T]`) crying calls, and earn a **Tamagotchi Care Rank** (`👑 S-Rank Perfect Keeper` .. `🩹 C-Rank Rascal`)!
   - **10 Collectible Companions & Fair Sanctuary Standby Care**: Collect 4 *Cozy Classics* (`🐱 Neko Mochi`, `🐶 Boba Pup`, `🐰 Cloud Bun`, `🐧 Star Pengu`) and 6 *Mythical & Fantasy* companions (`🐉 Ignis Drake`, `🐲 Azure Wyrm`, `🦊 Kitsune Nine`, `🦅 Sol Phoenix`, `🦄 Astra Unicorn`, `👑 Royal Jelly`). Inactive companions in your Sanctuary rest in standby care so they never accumulate unfair care mistakes.

---

## 🇮🇩 Fitur Unggulan (Bahasa Indonesia)

1. **Mesin Pixel Art 24×16 TrueColor (`▀`, `▄`, `█`)**:
   - Menggantikan ASCII garis miring dengan **bitmap piksel 24×16** berbasis *Unicode half-block* (`▀`, `▄`, `█`) yang presisi 1:1 di semua font terminal (Ghostty, Kitty, WezTerm, Alacritty).
2. **UI Retro Handheld Minimalis & Murni Step-by-Step**:
   - Satu layar satu fokus: mulai dari pemilihan pet (`Step 1/3` -> `Step 2/3` -> `Step 3/3`) hingga layar Habitat dan **4 Halaman Status Meter (`[I]`)** semuanya berganti halaman secara bersih tanpa menumpuk banyak kotak sekaligus.
3. **Mekanik Asli Bandai Tamagotchi & Multi-Pet Sanctuary**:
   - Lengkap dengan sistem **Meal vs. Sweet Snack**, **Berat Badan (`g`)**, **Sakit Gigi (`🦷`)**, **Saklar Lampu Kamar (`[K]`)**, **Discipline (`[D]` Scold / `[T]` Praise)**, **Care Rank (`S/A/B/C`)**, **Welcome-Back Offline Report**, serta penyimpanan ganda atomik (JSON + SQLite) di `~/.local/share/mochigotchi/`.

---

## ⌨️ Controls / Tombol Kontrol

| Key | Action |
|---|---|
| `←` / `→` (Button A) | Select icon on the 7-Icon Tamagotchi LCD Bezel |
| `Enter` / `B` (Button B) | Open / confirm highlighted LCD Bezel Icon |
| `Esc` (Button C) | Return to main LCD Habitat / close subscreen |
| `W` / `Space` | Warm egg (`+20%`, 3m cooldown) or cuddle hatched companion |
| `F` | Quick-feed Meal (`+1 Hungry ♥`, `+1g weight`, resets sweet streak) |
| `M` | Open the **Pantry Screen** (5 Meals & Sweet Snacks) |
| `K` | Toggle **Room Light ON / OFF** (`💡Light` / `🌙Dark`) |
| `P` | Play Left/Right Game (`+Happy`, `-2g weight`, `+5 coins`) |
| `D` | **Discipline / Scold** (`+25% Discipline` during a spoiled call) |
| `T` | **Praise / Headpat** (`+15% Discipline` when crying, or `+Happy`) |
| `E` | Training session (`-2g weight`, `+8% Discipline`, `+22 XP`, 4m cooldown) |
| `S` | Tuck into bed / Wake up |
| `C` | Flush toilet (`🚽`), brush teeth & warm bubble bath |
| `H` | Give **Medicine (`💉`)** — cures sickness & toothache (`🦷`) |
| `I` | Open **4-Page Tamagotchi Status Meter** (`←`/`→` or `Space` to flip pages) |
| `G` | Play the **Snack Catcher** mini-game arcade |
| `N` | Adopt an additional companion into your Sanctuary |
| `L` | Open **Sanctuary Roster** to browse & switch companions |
| `[` / `]` | Cycle active companion directly from the habitat |
| `Tab` | Cycle LCD shell colorway |
| `R` | Rename active companion |
| `?` | Open in-game Tamagotchi Hardware & Keys Guide |
| `Q` / `Esc` | Auto-save (JSON + SQLite) & quit |

---

## 🚀 Installation & CLI Usage

```bash
cargo build --release
cp target/release/mochigotchi ~/.local/bin/mochigotchi
ln -sf ~/.local/bin/mochigotchi ~/.local/bin/mochi
ln -sf ~/.local/bin/mochigotchi ~/.local/bin/tamagotchi
```

### Terminal Commands

```bash
mochigotchi                           # Launch interactive Retro Handheld TUI
mochi                                 # Short alias
tamagotchi                            # Classic alias
mochigotchi --version                 # Print version (1.0.0)
mochigotchi --status                  # Print active companion summary & pixel art
mochigotchi --roster                  # List all companions in your Sanctuary
mochigotchi --adopt --species dragon --theme gold --name Ignis
mochigotchi --switch 2                # Switch active companion by index or name
```

---

## 📄 License

Released under the [MIT License](./LICENSE).
