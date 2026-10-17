# 🦀 Rustagotchi — Terminal Virtual Pet for Developers

```text
  ____            _                   _       _     _ 
 |  _ \ _   _ ___| |_ __ _  __ _  ___| |_ ___| |__ (_)
 | |_) | | | / __| __/ _` |/ _` |/ _ \ __/ __| '_ \| |
 |  _ <| |_| \__ \ || (_| | (_| | (_) | || (__| | | | |
 |_| \_\\__,_|___/\__\__,_|\__, |\___/ \__\___|_| |_|_|
                           |___/                       
```

> **[English](#english) | [Bahasa Indonesia](#bahasa-indonesia)**

---

<a name="english"></a>
## 🌐 English

**Rustagotchi** is a retro-futuristic terminal virtual pet (Tamagotchi) built in pure Rust with **Ratatui** and **Crossterm**. Designed as a coding companion for developers, your pet lives inside your terminal, evolves as you write code, gains XP and coins whenever you make `git commit`s, plays an interactive terminal mini-game, and persists its state automatically across both **SQLite** and **JSON**.

### ✨ Terminal Dashboard Preview

```text
┌─ 🦀 Rustagotchi: Ferris [Stage: Teen | Lv.4 | Coins: 65 | Commits: 7] ───────────────┐
│┌─ Pet Habitat ───────────────────────────┐┌─ Vital Signs & XP ──────────────────────┐│
││        ✨  🦀  ✨                       ││ Hunger      [████████████████░░░░]  80% ││
││       /=========\                       ││ Happiness   [██████████████████░░]  90% ││
││      |  ^     ^  |   " git commit       ││ Energy      [██████████████░░░░░░]  70% ││
││      |   \___/   |     tastes great! "  ││ Health      [████████████████████] 100% ││
││     /|  [RUST]   |\                     ││ Cleanliness [██████████████████░░]  95% ││
││      |___________|                      ││ Stage XP    [████████████░░░░░░░░]  60% ││
││       /         \                       ││ Mood: Happy  | Age: 2h 15m              ││
│└─────────────────────────────────────────┘└─────────────────────────────────────────┘│
│┌─ Activity Log & Controls ──────────────────────────────────────────────────────────┐│
││ • Fed commit `a1b2c3d feat: add sqlite persistence` (+40 XP, +20 Coins!)           ││
││ • Played Bug Catcher mini-game! Score: 85 (+25 Coins, +35 XP)                      ││
││ [F]eed [P]lay [S]leep [C]lean [K]Code [H]eal [G]itSync [M]iniGame [R]ename [Q]uit  ││
│└────────────────────────────────────────────────────────────────────────────────────┘│
└──────────────────────────────────────────────────────────────────────────────────────┘
```

### 🚀 Key Features

- **5 Evolution Stages (`Egg → Baby → Teen → Adult → CyberElder`)**:
  Unlock multi-frame animated ASCII sprites, new speech bubbles, and stat bonuses as your pet levels up:
  - `Egg` (`0 XP`) → `Baby` (`50 XP`) → `Teen` (`150 XP`) → `Adult` (`350 XP`) → `CyberElder` (`700+ XP`).
- **Real-Time & Offline Time-Decay Engine**:
  Uses `chrono` timestamps (`last_updated`) to calculate exact elapsed offline seconds on launch. Hunger, happiness, cleanliness, and energy decay naturally over time; sleeping pets recover energy offline; severe neglect (`hunger <= 15` or `cleanliness <= 15`) triggers sickness (`PetMood::Sick`).
- **Git Commit XP Hooks (`post-commit` & `--sync-git`)**:
  Reward your pet automatically every time you commit code! Built-in commit hash/message deduplication (`seen_commits`) ensures each commit is rewarded once (`+40 XP`, `+20 Coins`, `+15 Hunger`, `+15 Happiness`).
- **Interactive Terminal Mini-Game (*Bug & Crate Catcher*)**:
  Play right inside the TUI! Move left/right to catch falling `Crate`s (`📦`) and `Coffee` (`☕`) for bonus XP and coins while dodging `Bug`s (`🐛`) and `Panic`s (`💥`).
- **Dual Persistence (`SQLite` + `JSON`)**:
  Every state change is saved to both `rustagotchi.db` (via bundled `rusqlite`) and `save.json` (via `serde_json`). Loads from SQLite primary storage with seamless JSON fallback.

### 🏗️ Architecture & Module Diagram

```text
                    ┌───────────────────────────┐
                    │       src/main.rs         │
                    │  CLI Flags & Entrypoint   │
                    └─────────────┬─────────────┘
                                  │
                    ┌─────────────▼─────────────┐
                    │        src/lib.rs         │
                    └─┬───────┬───┴───┬───────┬─┘
        ┌─────────────┘       │       │       └─────────────┐
        ▼                     ▼       ▼                     ▼
┌───────────────┐   ┌───────────────┐ ┌───────────────┐   ┌───────────────┐
│   src/pet.rs  │   │src/sprites.rs │ │src/minigame.rs│   │src/git_hook.rs│
│State, Decay & │   │ASCII Frames & │ │Bug & Crate    │   │post-commit &  │
│Evolution Math │   │Mood Particles │ │Catcher Engine │   │git log Sync   │
└───────┬───────┘   └───────┬───────┘ └───────┬───────┘   └───────┬───────┘
        │                   │                 │                   │
        └───────────┬───────┴────────┬────────┴───────────────────┘
                    ▼                ▼
            ┌───────────────┐┌───────────────┐
            │src/storage.rs ││  src/tui.rs   │
            │SQLite + JSON  ││Ratatui Layout │
            │Dual Storage   ││& Event Loop   │
            └───────────────┘└───────────────┘
```

### 💻 CLI Usage & Flags

Run `rustagotchi` with no arguments to launch the interactive TUI dashboard, or pass headless CLI flags for quick terminal actions and Git hooks:

```bash
# Launch interactive Ratatui dashboard
cargo run

# Inspect pet status summary in stdout
cargo run -- --status

# Perform quick care actions from your shell
cargo run -- --feed
cargo run -- --play
cargo run -- --sleep
cargo run -- --clean
cargo run -- --code
cargo run -- --heal

# Reward pet for a specific commit (used by .git/hooks/post-commit)
cargo run -- --git-commit "a1b2c3d feat: implement new module"

# Scan recent git commits in the current repository and reward unseen ones
cargo run -- --sync-git

# Install automatic .git/hooks/post-commit hook in current repo
cargo run -- --install-hook

# Set or rename pet and customize storage paths
cargo run -- --name "Ferris" --json-path save.json --db-path rustagotchi.db
```

### ⌨️ Interactive TUI Keybindings

| Key | Action | Description |
| :--- | :--- | :--- |
| `f` | **Feed** | Feed your pet (`+25 Hunger`, `+5 Happiness`, `+10 XP`) |
| `p` | **Play** | Play together (`+25 Happiness`, `+15 XP`, `+5 Coins`) |
| `s` | **Sleep** | Toggle sleep mode (recovers energy over time) |
| `c` | **Clean** | Clean habitat (`Cleanliness = 100`, `+10 Happiness`, `+10 XP`) |
| `k` | **Code Session** | Pair-program (`+25 XP`, `+15 Coins`, `Coding` mood) |
| `h` | **Heal** | Cure sickness and restore health to `100%` |
| `g` | **Git Sync** | Scan recent `git log` commits and reward unseen commits |
| `m` | **Mini-Game** | Launch interactive *Bug & Crate Catcher* mini-game |
| `r` | **Rename** | Enter inline rename prompt (`Enter` to confirm, `Esc` to cancel) |
| `←` / `→` or `a` / `d` | **Move** | Move player left/right inside the Mini-Game |
| `q` / `Esc` | **Quit** | Save state to SQLite + JSON and exit cleanly |

### 🛠️ Build, Run & Test

```bash
# Build and launch interactive TUI
cargo run

# Run full unit + integration test suite
cargo test

# Run linter checks
cargo clippy --all-targets
```

---

<a name="bahasa-indonesia"></a>
## 🇮🇩 Bahasa Indonesia

**Rustagotchi** adalah aplikasi hewan peliharaan virtual terminal (*Tamagotchi*) bergaya retro-futuristik yang dibangun menggunakan **Rust**, **Ratatui**, dan **Crossterm**. Dirancang khusus sebagai teman *ngoding* bagi para developer, hewan peliharaan Anda hidup di dalam terminal, berevolusi seiring Anda menulis kode, mendapatkan XP dan koin setiap kali Anda melakukan `git commit`, memiliki mini-game interaktif di terminal, serta menyimpan datanya secara otomatis ke **SQLite** dan **JSON**.

### 🌟 Fitur Utama

- **5 Tahap Evolusi (`Egg → Baby → Teen → Adult → CyberElder`)**:
  Buka animasi sprite ASCII multi-frame, balon percakapan baru, dan bonus atribut seiring naiknya level peliharaan Anda:
  - `Egg` (`0 XP`) → `Baby` (`50 XP`) → `Teen` (`150 XP`) → `Adult` (`350 XP`) → `CyberElder` (`700+ XP`).
- **Mesin Kalkulasi Waktu Nyata & Offline (*Offline Time-Decay*)**:
  Menggunakan stempel waktu `chrono` (`last_updated`) untuk menghitung selisih detik secara akurat saat aplikasi dibuka kembali. Rasa kenyang (*hunger*), kebahagiaan (*happiness*), kebersihan (*cleanliness*), dan energi berkurang secara alami seiring waktu; tidur memulihkan energi; kelalaian ekstrem (`hunger <= 15` atau `cleanliness <= 15`) menyebabkan peliharaan jatuh sakit (`PetMood::Sick`).
- **Integrasi Git Commit Hook (`post-commit` & `--sync-git`)**:
  Berikan hadiah XP dan koin secara otomatis setiap kali Anda melakukan `git commit`! Dilengkapi fitur deduplikasi (`seen_commits`) sehingga setiap commit hanya diklaim satu kali (`+40 XP`, `+20 Koin`, `+15 Hunger`, `+15 Happiness`).
- **Mini-Game Terminal Interaktif (*Bug & Crate Catcher*)**:
  Bermain langsung di dalam TUI! Bergerak ke kiri/kanan untuk menangkap `Crate` (`📦`) dan `Coffee` (`☕`) demi XP serta koin tambahan sambil menghindari `Bug` (`🐛`) dan `Panic` (`💥`).
- **Penyimpanan Ganda (*Dual Persistence*: `SQLite` + `JSON`)**:
  Setiap perubahan status disimpan secara simultan ke database `rustagotchi.db` (`rusqlite`) dan file `save.json` (`serde_json`). Memuat data utama dari SQLite dengan cadangan otomatis dari JSON.

### 💻 Penggunaan CLI & Opsi

Jalankan `rustagotchi` tanpa argumen untuk membuka antarmuka TUI interaktif, atau gunakan opsi CLI untuk aksi cepat maupun integrasi Git hook:

```bash
# Menjalankan dashboard TUI interaktif
cargo run

# Menampilkan ringkasan status peliharaan di terminal
cargo run -- --status

# Melakukan aksi perawatan cepat melalui CLI
cargo run -- --feed
cargo run -- --play
cargo run -- --sleep
cargo run -- --clean
cargo run -- --code
cargo run -- --heal

# Memberi hadiah XP dari git commit (digunakan oleh .git/hooks/post-commit)
cargo run -- --git-commit "a1b2c3d feat: tambah fitur baru"

# Memindai riwayat git log terbaru dan memberi hadiah untuk commit baru
cargo run -- --sync-git

# Memasang script otomatis .git/hooks/post-commit pada repositori saat ini
cargo run -- --install-hook

# Mengubah nama peliharaan atau mengatur lokasi file penyimpanan
cargo run -- --name "Ferris" --json-path save.json --db-path rustagotchi.db
```

### 🛠️ Cara Kompilasi, Menjalankan & Pengujian

```bash
# Kompilasi dan jalankan TUI interaktif
cargo run

# Jalankan seluruh pengujian (unit test + integration test)
cargo test

# Periksa kualitas kode dengan Clippy
cargo clippy --all-targets
```
