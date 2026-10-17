# 🍡 Mochigotchi v0.5.0 — Authentic Bandai Tamagotchi Hardware & Sanctuary Edition

> **Your Cozy Pocket-Pet Sanctuary in the Terminal.**  
> Play from anywhere in your terminal with `mochigotchi`, `mochi`, or `tamagotchi`! Features reverse-engineered authentic Bandai Tamagotchi hardware mechanics (8 LCD Bezel Icons, 3-Button A/B/C navigation, 4 Hungry/Happy Hearts, Meal vs. Sweet Snack Weight & Toothache system, Room Light switch, Discipline Scold/Praise, Care Mistakes) + 10 TrueColor ASCII companions in a multi-pet Sanctuary.

---

## ✨ What's New in v0.5.0 (Authentic Tamagotchi Reverse-Engineering)

1. **Launch Anywhere from Terminal (`mochigotchi` / `mochi` / `tamagotchi`)**:
   - Installed directly into `~/.local/bin` and saves globally in `~/.local/share/mochigotchi/` so you can summon your Sanctuary from any directory in your terminal.
2. **8 Classic LCD Bezel Icons + 3-Button Navigation (`A = Select`, `B = Confirm`, `C = Cancel`)**:
   - Top Bezel: `🍙 Food`, `💡 Light`, `🎮 Game`, `💉 Meds`
   - Bottom Bezel: `🚽 Bath`, `📏 Meter`, `📢 Scold`, `🔔 Call`
   - Use `←`/`→` (Button A) to move the highlight across the LCD bezel icons, `Enter` (Button B) to activate, and `Esc` (Button C) to cancel — or use fast single-key shortcuts!
3. **Meals vs. Sweet Snacks, Weight (`g`) & Toothache (`🦷`)**:
   - **Meals** (`[F]` or Pantry #1/#2/#4) fill **Hungry Hearts (`♥♥♥♥`)**, add `+1..2g` weight, and are refused when 4/4 full.
   - **Sweet Snacks** (Pantry #3 Boba Tea / #5 Starlight Tart) fill **Happy Hearts (`♥♥♥♥`)** and add `+2..3g` weight — but feeding **3 sweet snacks in a row** causes a **Toothache (`🦷`)** requiring **Medicine (`💉 [H]`)**!
   - Playing games (`[P]`), training (`[E]`), or the Arcade (`[G]`) burns `-1..2g` of weight down to your stage's healthy Base Weight (`5g` Baby, `15g` Teen, `30g` Adult, `40g` Elder).
4. **Room Light Switch (`💡 [K]`), Attention Calls (`🔔`) & Discipline (`📢 [D]` / `[T]`)**:
   - When your companion goes to sleep (`[S]`), remember to switch the **Room Light OFF (`[K]`)**! Leaving the light on while sleeping causes a **Care Mistake** and restless sleep.
   - When the **Attention Icon (`🔔`)** calls: if your pet is *Acting Spoiled* even though its belly & mood are fine, press `[D]` to **Scold** (`+25% Discipline`). If it is *Crying*, press `[T]` to **Praise & Comfort** (`+15% Discipline`).
5. **4-Page Bandai Health Meter (`📏 [I]`)**:
   - Flip through 4 authentic status screens: (1) Age, Stage & Weight (`g`), (2) Discipline (`■ ■ ■ □`) & Care Mistakes, (3) Hungry Hearts (`♥♥♥♥`) & Toothache streak, (4) Happy Hearts (`♥♥♥♥`), Bond & Light status.

---

## ⌨️ Controls / Tombol Kontrol

| Key | Action |
|---|---|
| `←` / `→` (Button A) | Select icon on the 8-Icon Tamagotchi LCD Bezel |
| `Enter` / `B` (Button B) | Open / confirm highlighted LCD Bezel Icon |
| `W` / `Space` | Warm egg (`+20%`, 3m cooldown) or cuddle hatched companion |
| `F` | Quick-feed Meal (`+1 Hungry ♥`, `+1g weight`) |
| `M` | Open the **Pantry Menu** (Meals vs. Sweet Snacks) |
| `K` | Toggle **Room Light ON / OFF** (`💡` / `🌙`) |
| `P` | Play Left/Right Game (`+Happy`, `-2g weight`, `+5 coins`) |
| `D` | **Discipline / Scold** (`+25% Discipline` during a spoiled call) |
| `T` | **Praise / Headpat** (`+15% Discipline` when crying, or `+Happy`) |
| `E` | Training session (`-2g weight`, `+Discipline`, `+22 XP`, 4m cooldown) |
| `S` | Tuck into bed / Wake up |
| `C` | Flush toilet (`🚽`) & warm bubble bath |
| `H` | Give **Medicine (`💉`)** — cures sickness & toothache (`🦷`) |
| `I` | Open **4-Page Tamagotchi Health Meter** |
| `G` | Play the **Snack Catcher** mini-game arcade |
| `N` | Adopt an additional companion into your Sanctuary |
| `L` | Open **Sanctuary Roster** to inspect & switch companions |
| `[` / `]` | Cycle active companion directly from the habitat |
| `Tab` | Cycle LCD shell colorway |
| `R` | Rename active companion |
| `?` | Open in-game Tamagotchi Hardware & Care Guide |
| `Q` / `Esc` | Auto-save (JSON + SQLite) & quit |

---

## 🚀 Terminal Commands

```bash
mochigotchi              # Launch interactive TUI from any directory!
mochi                    # Short alias for mochigotchi
tamagotchi               # Classic alias for mochigotchi
mochigotchi --status     # Inspect active companion in terminal
mochigotchi --roster     # List all companions in your Sanctuary
```
