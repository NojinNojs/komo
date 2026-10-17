# 🍓 Mochigotchi (`mochigotchi`)

> **EN:** A full-featured, kawaii virtual pet & Tamagotchi LCD simulator for your terminal, built in Rust with `ratatui` and `crossterm`.  
> **ID:** Simulator hewan peliharaan virtual bergaya Tamagotchi LCD kawaii yang lengkap di terminal Anda, dibangun dengan Rust menggunakan `ratatui` dan `crossterm`.

---

## ✨ Key Features / Fitur Utama

### 🇬🇧 English
- **4 Adorable Species:** Choose between **🐱 Neko Mochi**, **🐶 Boba Pup**, **🐰 Cloud Bun**, and **🐧 Star Pengu**, each with unique multi-frame ASCII sprites, default names, and favorite snacks.
- **5 Pastel LCD Shell Themes:** Customize your virtual Tamagotchi casing with **🍓 Strawberry Pink**, **🍵 Matcha Green**, **☁️ Sky Blue**, **💜 Lavender Dream**, or **✨ Sunset Gold** (cycle live anytime with `Tab`).
- **5 Evolution Stages & Interactive Egg Incubation:**
  - `Egg` (0 XP) → `Baby` (50 XP) → `Teen` (150 XP) → `Adult` (350 XP) → `Starlight Elder` (700 XP).
  - Newly adopted pets start inside an **unhatched Egg (`🥚`)** and must be warmed (`[W] Warm Egg`) 4 times to hatch before eating, playing, or training!
- **Authentic Anti-Spam Care Mechanics:**
  - Full pets (`hunger >= 95`) refuse extra meals.
  - Max happiness pets (`happiness >= 98`) enjoy cuddles without infinite XP farming.
  - Feeding every 2 meals produces poop (`💩`) that lowers cleanliness until scrubbed (`[C] Clean`).
  - Tired, hungry, sleeping, or sick pets refuse strenuous play/training until rested or healed.
- **Built-in Snack & Star Catcher Mini-Game (`[G]`):** Move your basket (`\===/`) left and right to catch falling Taiyaki (`🐟`), Strawberries (`🍓`), and Shooting Stars (`⭐`) for bonus Coins, XP, and combo multipliers while dodging Bombs (`💣`)!
- **Responsive 3-Tier TUI Layout:** Adapts automatically to **Compact** (`< 78 cols`), **Standard** (`78..109 cols`), and **Wide Deluxe** (`>= 110 cols`) terminals.
- **Dual Persistence & Real-Time Offline Decay:** Automatically saves state to both human-readable **JSON** and **SQLite** databases, calculating offline stat decay and sleep recovery when you return.

### 🇮🇩 Bahasa Indonesia
- **4 Spesies Menggemaskan:** Pilih antara **🐱 Neko Mochi**, **🐶 Boba Pup**, **🐰 Cloud Bun**, dan **🐧 Star Pengu**, masing-masing dengan animasi sprite ASCII multi-frame, nama bawaan, dan camilan favorit.
- **5 Tema Casing LCD Pastel:** Sesuaikan tampilan perangkat Tamagotchi Anda dengan **🍓 Strawberry Pink**, **🍵 Matcha Green**, **☁️ Sky Blue**, **💜 Lavender Dream**, atau **✨ Sunset Gold** (ganti kapan saja dengan tombol `Tab`).
- **5 Tahap Evolusi & Inkubasi Telur Interaktif:**
  - `Egg` (0 XP) → `Baby` (50 XP) → `Teen` (150 XP) → `Adult` (350 XP) → `Starlight Elder` (700 XP).
  - Peliharaan baru dimulai dari **Telur (`🥚`)** yang harus dihangatkan (`[W] Warm Egg`) 4 kali hingga menetas sebelum bisa diberi makan, diajak bermain, atau berlatih.
- **Mekanisme Perawatan Anti-Spam Otentik:**
  - Peliharaan yang kenyang (`hunger >= 95`) menolak makanan tambahan.
  - Setiap 2 kali makan menghasilkan kotoran (`💩`) yang menurunkan kebersihan kamar hingga dibersihkan (`[C] Clean`).
  - Peliharaan yang lelah, lapar, tidur, atau sakit menolak bermain/berlatih hingga pulih.
- **Mini-Game Snack & Star Catcher (`[G]`):** Gerakkan keranjang (`\===/`) ke kiri dan kanan untuk menangkap Taiyaki (`🐟`), Stroberi (`🍓`), dan Bintang Jatuh (`⭐`) demi Koin & XP ekstra sambil menghindari Bom (`💣`)!
- **Tampilan TUI Responsif 3 Tingkat:** Menyesuaikan ukuran terminal secara otomatis untuk mode **Compact**, **Standard**, dan **Wide Deluxe**.
- **Penyimpanan Ganda (JSON + SQLite) & Perhitungan Waktu Offline:** Menyimpan progres secara otomatis ke file JSON dan database SQLite, serta menghitung perubahan status selama Anda menutup aplikasi.

---

## 🚀 Installation & Quick Start / Instalasi & Cara Menjalankan

```bash
# Clone & enter repository
git clone <repository-url>
cd rustagotchi

# Launch interactive Tamagotchi LCD TUI (default)
cargo run
```

First-time players are greeted by the **Interactive Adoption & Shell Customization Wizard** to select their species, shell theme, and pet name.

---

## 💻 CLI Subcommands / Perintah CLI

You can also care for your pet directly from your shell or scripts:

```bash
# Launch interactive TUI with a custom name (if no save exists)
cargo run -- --name "Boba"

# Check pet status summary in terminal
cargo run -- status

# Warm the unhatched egg (or cuddle after hatching)
cargo run -- warm

# Care actions
cargo run -- feed     # Feed favorite snack
cargo run -- play     # Play together
cargo run -- pet      # Give gentle headpats & love
cargo run -- train    # Train for high XP
cargo run -- sleep    # Toggle sleep / wake up
cargo run -- clean    # Scrub poop & restore room cleanliness
cargo run -- heal     # Give herbal medicine when sick

# Reset and adopt a fresh pet
cargo run -- reset --name "Mochi"
```

---

## 🎮 Keyboard Controls / Kontrol Keyboard (TUI)

### Habitat Screen / Layar Utama

| Key / Tombol | Action / Aksi | Description / Keterangan |
| :--- | :--- | :--- |
| `←` / `→` or `h` / `l` | Navigate Menu | Move selection across the 8 LCD care buttons |
| `Enter` / `Space` | Activate Button | Perform the highlighted care action |
| `w` / `0` | **Warm Egg** | Warm the egg (`+25%` incubation) or cuddle once hatched |
| `f` / `1` | **Feed** | Feed species' favorite snack (`+Hunger`, `+XP`, `+Coins`) |
| `p` / `2` | **Play** | Play joyful games (`+Happiness`, `+Bond`, `+XP`, `+Coins`) |
| `v` / `3` | **Pet Love** | Gentle headpats (`+Happiness`, `+Bond`, `+XP`) |
| `t` / `4` | **Train** | Practice skills (`+XP`, `+Coins`, `-Energy`) |
| `s` / `5` | **Sleep** | Toggle bedtime mode to restore Energy |
| `c` / `6` | **Clean** | Clean up poop (`💩`) and restore 100% Cleanliness |
| `h` / `7` | **Heal** | Cure sickness and restore Health |
| `g` | **Mini-Game** | Launch the **Snack & Star Catcher** arcade mini-game |
| `Tab` | **Cycle Theme** | Switch between the 5 pastel LCD shell themes |
| `n` | **New Adoption** | Open the Adoption Wizard to adopt a new pet |
| `q` / `Esc` | **Save & Quit** | Save to JSON + SQLite and exit cleanly |

### Mini-Game Screen (`Snack & Star Catcher`)

| Key / Tombol | Action / Aksi |
| :--- | :--- |
| `←` / `a` / `h` | Move basket left (`\===/`) |
| `→` / `d` / `l` | Move basket right (`\===/`) |
| `Enter` / `Esc` / `q` | Finish or exit mini-game & return to Habitat |

---

## 🏗️ Architecture / Arsitektur Kode

```text
src/
├── lib.rs        # Crate module exports
├── main.rs       # Clap CLI parser & entrypoint
├── pet.rs        # Core domain model: Pet, Species, ShellTheme, EvolutionStage, Care guards
├── sprites.rs    # Multi-frame ASCII art sprites, theme palettes, dynamic speech bubbles
├── minigame.rs   # Snack & Star Catcher arcade engine
├── storage.rs    # Dual JSON + SQLite persistence & offline decay synchronization
└── tui.rs        # Responsive 3-tier Ratatui LCD UI & Onboarding Wizard
tests/
└── integration_tests.rs  # Comprehensive integration & headless TUI test suite
```

---

## 🧪 Testing / Pengujian

Run the full unit and integration test suite:

```bash
cargo test
```
