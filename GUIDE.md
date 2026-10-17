# KOMO: Comprehensive Keeper's Guide

**KOMO** (*Komorebi Pocket Soul*) is a terminal virtual companion built with `ratatui` and `crossterm`. Your companion lives inside a centered retro handheld LCD frame, perceives its room through four active senses, looks left, right, and straight at you, and holds you accountable for how you raise it.

---

## 1. Quick Start & Launching

Launch the interactive handheld TUI from anywhere in your terminal:

```bash
komo
```

*(Aliases `mochi` and `mochigotchi` also point to the same binary.)*

### Command-Line Flags

| Flag | Purpose |
| :--- | :--- |
| `komo` | Open the interactive handheld LCD interface |
| `komo --call-me <TITLE>` | Set how your companion addresses you (e.g., `komo --call-me Aqsan`) |
| `komo --name <NAME>` | Set or rename your active companion |
| `komo --species <SPECIES>` | `cat`, `pup`, `bun`, `pengu`, `dragon`, `wyrm`, `fox`, `phoenix`, `uni`, `slime` |
| `komo --theme <THEME>` | `pink`, `matcha`, `blue`, `lavender`, `gold` |
| `komo --status` | Print live vitals, 4 senses, speech, and the 40x22 sprite in stdout |
| `komo --roster` | List all companions in your Sanctuary and the Memorial Hall ledger |
| `komo --adopt` | Adopt an additional companion into your Sanctuary |
| `komo --switch <ID\|NAME>` | Switch active companion by 1-based number or name |
| `komo --vigil` | Advance 1 step of the 3-step Burial Vigil Rite for a fallen companion |
| `komo --warm` / `--feed` / `--snack` | Warm an egg (`+20%`), feed a meal (`+1 Hungry ♥`), or give a sweet snack |
| `komo --play` / `--pet` / `--talk` | Play a quick match, cuddle/comfort, or have a quiet conversation |
| `komo --train` / `--sleep` / `--light` | Run a discipline drill, toggle sleep on the woven mat, or flip the lamp |
| `komo --scold` / `--clean` / `--heal` | Scold during a false call, wash the floor, or give herbal medicine / Phoenix Tear |
| `komo -V` / `--version` | Print installed version |

---

## 2. Custom Caretaker Name ("What Should I Call You?")

Your companion never uses robotic placeholders if you tell it who you are. You can set your Caretaker Title in three ways:

1. **During Onboarding (Step 3 of 3)**: Press `Tab` (or `↑`/`↓`) to switch between the **Companion Name** field and the **Call You As** field before pressing `Enter`.
2. **Inside the Habitat (`[O]` Key)**: Press `O` at any time in the main Habitat screen, type your preferred name or title (up to 18 characters, e.g., `Aqsan`, `Boss`, `Captain`, `Senpai`), and press `Enter`.
3. **From the Terminal**: Run `komo --call-me Aqsan`.

Every spoken line (`💬`), sensory observation (`👁`/`👂`/`🫳`), dialogue prompt (`[Y]`), and memorial epitaph updates immediately to address you by name.

---

## 3. Three-Way Gaze, Poses, and Living Environment

### 3-Way Gaze (`Left ◀`, `Center ●`, `Right ▶`)
Unlike flat side-scrollers that only flip left and right, KOMO companions have three distinct gaze directions on the 40x22 half-block canvas:
- **Looking at You (`Center ●`)**: When you press keys, open a conversation, or when your companion pauses in the middle of the room, it turns its head forward with both eyes facing the screen directly.
- **Looking Left (`Left ◀`) & Looking Right (`Right ▶`)**: While strolling across the habitat floor, inspecting garden stones, or watching dust motes by the window, it turns in 3/4 profile.
- **Turning Away**: If you scold your companion unfairly when it was not misbehaving, it folds its ears and turns its gaze away from you until you rebuild trust.

### Full-Body Poses & Facial Detail
- **Sleeping Pose**: When tucked in (`[S]`), your companion does not remain standing. It lowers and curls onto a woven pixel sleeping mat on the habitat floor, breathing slowly while `z Z` particles drift upward.
- **Hungry & Starving Slouch**: When Hungry Hearts drop to 1 or 0 (`hunger <= 25`), its ears and posture droop, its eyes turn watery (`A`), and stomach-rumble waves appear beside its belly.
- **Full & Satisfied**: After a hearty meal, its eyes curve into contented arcs, cheeks warm with blush (`P`), and its belly rounds out.
- **Fever & Toothache**: When sick or suffering from a sweet-triggered toothache, its eyelids droop half-shut with flushed cheeks (`R`) and cold sweat drops.

### The Living Diorama Stage
- **Sanctuary Window Backdrop**: Cycles through four living atmospheres (`DAY` sunbeams and drifting clouds, `BREEZE` falling petals, `RAIN` slanted silver drops, and `NIGHT` starlight), or switches to a starry dark-room view when you turn the lamp off (`[K]`).
- **Four Active Senses**:
  - `👁 Sight`: Tracks which bezel icon you are highlighting, the room lamp state, and weather outside the glass.
  - `👂 Hearing`: Reacts to your keyboard rhythm. Steady typing perks one ear; rapid button-mashing (`6+` rapid presses) startles it; silence lets it listen to rain or crickets.
  - `🫳 Feeling`: Reports internal body state—stomach cramps, fever chills, molar pain, muscle fatigue, or warmth from your hand.
  - `💬 Speech`: Grounded, first-person dialogue written in natural English without clichés.

---

## 4. Controls & Hardware Bezel Navigation

| Key | Action |
| :--- | :--- |
| `←` / `→` then `Enter` | Highlight an icon on the top LCD bezel and activate it |
| `W` or `Space` | Warm unhatched egg (`+20%` per 3 min) / Cuddle hatched companion |
| `F` / `M` | Feed favorite meal (`F`) or open the 5-item Pantry menu (`M`) |
| `Y` | Open Heart-to-Heart Dialogue screen (`1` or `2` to reply) |
| `P` / `G` | Quick Left/Right play (`P`) or launch the 3-in-1 Retro Arcade (`G`) |
| `E` | Run a discipline & strength training drill (`-20` Energy, `-2g` weight) |
| `S` / `K` | Toggle Sleep on the woven mat (`S`) / Toggle Room Lamp ON or OFF (`K`) |
| `C` | Wash the habitat floor, brush teeth, and give a warm bath |
| `D` | Scold (ONLY when `🔔 CALL: Scold [D]` is active) |
| `T` | Praise and comfort (when crying or seeking warmth) |
| `H` | Give herbal medicine (when sick/toothache) or use 1-time Phoenix Tear |
| `B` | Advance the 3-step Burial Vigil Rite when a companion has died |
| `O` / `R` | Change what your companion calls you (`O`) / Rename companion (`R`) |
| `I` | Open 4-page Status Meter (`←`/`→` or `Space` to flip pages) |
| `L` / `[` / `]` | Open Sanctuary Roster (`L`) or cycle active companions (`[` / `]`) |
| `N` | Adopt a new companion into the Sanctuary |
| `U` / `Tab` / `?` | Mute/unmute 8-bit audio (`U`), cycle shell color (`Tab`), or view Help (`?`) |

---

## 5. Real Consequences, Death, and the Memorial Hall

KOMO enforces binding consequences so you cannot mistreat a companion or spam buttons without repercussions:

1. **Force-Feeding Penalty**: Trying to force-feed meals when your companion is already at `4/4` Hungry Hearts (`hunger >= 95`) causes stomach cramps and nausea (`-6% Health`, `-6% Happiness`, `-3% Bond`, `+1 Care Mistake`).
2. **Sweet Toothache**: Feeding 3 sweet snacks in a row (`Warm Milk Tea` or `Starlight Delight`) causes a painful **Toothache** (`+1 Care Mistake`). Your companion will refuse all food until treated with Medicine (`[H]`).
3. **Unfair Scolding Penalty**: Scolding (`[D]`) when your companion is not making a fake attention call damages trust (`-6% Bond`, `-14% Happiness`, `+1 Care Mistake`), makes them angry, and causes them to turn their gaze away from you.
4. **Sleeping with the Lamp ON**: Leaving the room light ON (`light_on = true`) while your companion sleeps halves energy recovery and accumulates Care Mistakes every 30 ticks.
5. **Starvation, Untreated Illness & Death**:
   - When `hunger == 0`, `poop_count >= 3`, or sickness goes untreated, **Health** drains steadily.
   - If `Health` hits `0%`, your companion collapses (`is_alive = false`).
6. **The One-Time Phoenix Tear vs. Permanent Death**:
   - A fallen companion can be revived with a **Sacred Phoenix Tear (`[H]`)** at most **ONCE in its entire lifetime** (`revival_count: 0 -> 1`). Doing so costs up to `50 coins`, adds `+3 Care Mistakes`, and lowers Bond because the companion remembers nearly dying.
   - If a companion dies a **second time** (`revival_count >= 1`), the Phoenix Tear can never be used on them again. Death is permanent.
7. **Mandatory 3-Step Burial Vigil (`[B]`) & Caretaker Karma**:
   - You **cannot** press `[N]` to adopt a new egg while a fallen companion lies unburied in the room.
   - You must press `[B]` three times to complete the **Burial Vigil Rite**:
     - **Step 1/3**: Wash the habitat floor and fold their woven sleeping mat.
     - **Step 2/3**: Carve their name, stage, and time lived onto a memorial stone.
     - **Step 3/3**: Speak a final epitaph and lay them to rest in the permanent **Sanctuary Memorial Hall**.
   - Every death lowers your **Caretaker Karma (`-15`)**. When your karma is negative, newly adopted companions sense your history and hatch with lower initial trust (`Bond = 4%`), requiring patient care to warm up to you.
