# 🍡 Mochigotchi

> **EN:** A cozy, collectible pocket-pet simulator for your terminal — ten species,
> a multi-pet Sanctuary, realistic incubation & care cooldowns, and dual JSON +
> SQLite saves. Built in Rust with `ratatui`.
>
> **ID:** Simulator hewan peliharaan saku yang hangat di terminalmu — sepuluh spesies,
> Sanctuary multi-pet, masa inkubasi & jeda perawatan yang realistis, dan penyimpanan
> ganda JSON + SQLite. Dibuat dengan Rust dan `ratatui`.

<p align="center">
  <em>No prior programming knowledge required. Just warm the egg and say hi.</em>
</p>

---

## ✨ Features / Fitur

| | English | Bahasa Indonesia |
|---|---|---|
| 🐉 | **10 collectible species** — 4 Cozy Classics + 6 Mythical & Fantasy (two dragons!) | **10 spesies koleksi** — 4 Cozy Classics + 6 Mythical & Fantasy (dua naga!) |
| 🏰 | **Multi-pet Sanctuary** — raise a whole menagerie, switch instantly | **Sanctuary multi-pet** — pelihara banyak sekaligus, ganti kapan saja |
| 🥚 | **Realistic incubation** — eggs need warmth and time, never instant | **Inkubasi realistis** — telur butuh kehangatan dan waktu, bukan instan |
| ⏳ | **Care cooldowns** — no spamming headpats or training for free XP | **Jeda perawatan** — tidak bisa spam elus atau latihan demi XP gratis |
| 📈 | **XP + Bond + Age evolution** — all three gates must be satisfied | **Evolusi XP + Bond + Umur** — ketiganya harus terpenuhi |
| 🎮 | **Snack Catcher arcade** mini-game | **Mini-game** Snack Catcher |
| 💾 | **Dual persistence** — pretty JSON *and* SQLite, with legacy upgrade | **Penyimpanan ganda** — JSON rapi *dan* SQLite, plus upgrade save lama |
| 📱 | **Responsive LCD UI** — Compact / Standard / Wide tiers | **UI LCD responsif** — tiga tingkat: Compact / Standard / Wide |

---

## 🐾 The Companions / Para Sahabat

### Cozy Classics
| Species | Rarity | Element | Favorite snack |
|---|---|---|---|
| 🐱 **Neko Mochi** | Common ★ | 🍡 Sweet | Taiyaki Fish 🐟 |
| 🐶 **Boba Pup** | Common ★ | 🧋 Tapioca | Honey Biscuit 🦴 |
| 🐰 **Cloud Bun** | Common ★ | ☁️ Sky | Sweet Carrot 🥕 |
| 🐧 **Star Pengu** | Rare ★★ | ❄️ Aurora | Snowberry 🫐 |

### Mythical & Fantasy
| Species | Rarity | Element | Favorite snack |
|---|---|---|---|
| 🐉 **Ignis Drake** | Legendary ★★★★★ | 🔥 Dragonfire | Flame Chili 🌶️ |
| 🐲 **Azure Wyrm** | Mythic ★★★★ | 🌊 Celestial Tide | Moon Pearl 🫧 |
| 🦊 **Kitsune Nine** | Epic ★★★ | 🦊 Spirit Foxfire | Inari Tofu 🦊 |
| 🐦‍🔥 **Sol Phoenix** | Legendary ★★★★★ | ☀️ Sunburst | Sunfire Berry ☀️ |
| 🦄 **Astra Unicorn** | Epic ★★★ | ✨ Starlight | Starlight Sugar ✨ |
| 👑 **Royal Jelly** | Rare ★★ | 💧 Mystic Slime | Crystal Dew 💧 |

Every species has **five hand-drawn multi-frame ASCII sprites** (Egg, Baby, Teen,
Adult, Starlight Elder) that blink and change expression across all six moods.
<br>
*Setiap spesies punya lima sprite ASCII animasi (Telur, Bayi, Remaja, Dewasa,
Starlight Elder) yang berkedip dan berubah ekspresi di enam suasana hati.*

---

## 🚀 Quick start / Mulai cepat

```bash
git clone <your-fork-url> mochigotchi
cd mochigotchi
cargo run --release
```

On first launch you are greeted by the **Adoption Wizard**:
<br>**EN:** choose a species → pick a shell colorway → type a name.
<br>**ID:** pilih spesies → pilih warna cangkang → ketik nama.

```bash
# Jump straight in with a specific companion
cargo run --release -- --species dragon --theme gold --name Ignis

# Inspect without entering the TUI
cargo run --release -- --status
cargo run --release -- --roster
```

### CLI flags / Opsi CLI

```
-h, --help            Show help
--species <SPECIES>   cat | pup | bun | pengu | dragon | wyrm | fox | phoenix | unicorn | slime
--theme <THEME>       pink | matcha | blue | lavender | gold
--name <NAME>         Name your companion
--status              Print the active companion, then exit
--roster              Print every companion in the Sanctuary, then exit
--warm --feed --play --pet --train --sleep --clean --heal
--json-path <FILE>    Custom JSON save location
--db-path <FILE>      Custom SQLite save location
--reset               Delete existing saves and start over
```

---

## 🎮 Controls / Kontrol

| Key | Action (EN) | Aksi (ID) |
|---|---|---|
| `W` / `Space` | Warm the egg (12s cooldown) / cuddle | Hangatkan telur (jeda 12s) / elus |
| `F` / `1` | Feed the favorite snack | Beri camilan favorit |
| `P` / `2` | Play together | Ajak bermain |
| `T` / `3` | Headpats (8s cooldown) | Elus kepala (jeda 8s) |
| `E` / `4` | Train (15s cooldown) | Latihan (jeda 15s) |
| `S` / `5` | Toggle sleep | Tidurkan / bangunkan |
| `C` / `6` | Clean the habitat | Bersihkan kandang |
| `H` / `7` | Give medicine | Beri obat |
| `G` / `M` | Snack Catcher arcade | Mini-game Snack Catcher |
| `L` | Open the Sanctuary roster | Buka daftar Sanctuary |
| `N` | Adopt another companion | Adopsi sahabat baru |
| `[` / `]` | Switch companion instantly | Ganti sahabat langsung |
| `Tab` | Cycle shell colorway | Ganti warna cangkang |
| `R` | Rename | Ganti nama |
| `Q` / `Esc` | Save & quit | Simpan & keluar |

**Snack Catcher:** `←` / `→` (or `A` / `D`) to move the basket, `Esc` / `Q` to stop.

---

## 🧠 How the simulation works / Cara kerja simulasinya

### 1. The egg comes first / Telur dulu
An unhatched egg **cannot** be fed, played with, trained, cleaned, or medicated —
every one of those actions is politely refused and grants **zero XP**.
The only ways forward are:

* **Manual warmth** — `W` adds `+20%` incubation and `+4` bond, with a **12-second
  cooldown** so you must actually take your time.
* **Passive warmth** — time itself incubates the egg: `+1%` per in-game tick and
  `+1%` per 15 seconds away. **Passive warmth caps at 99%**, so one deliberate
  `W` is always the final push that hatches it.

### 2. Care has limits / Perawatan punya batas
| Situation | Result |
|---|---|
| Stomach is 95%+ full | Refuses food, no XP |
| Happiness is maxed | Refuses more headpats, no XP |
| Habitat already sparkling | Refuses cleaning, no XP |
| Perfectly healthy | Refuses medicine, no XP |
| Too tired / too hungry | Refuses play & training |
| Feeling sick | Needs medicine before play & training |
| Fast asleep | Ignores every action until woken |

Feeding twice fills the litter box: messes lower cleanliness, which slowly damages
health if you never tidy up.

### 3. Evolution needs all three gates / Evolusi butuh tiga syarat
| Stage | XP | Bond | Time together |
|---|---|---|---|
| 🍼 Baby | 80 | 10 | — |
| 🌱 Teen | 250 | 25 | 60s |
| 🌸 Adult | 600 | 45 | 180s |
| 🌟 Starlight Elder | 1200 | 70 | 420s |

Stats also decay in real time — hunger, happiness and cleanliness drop as you play,
and everything keeps ticking while the app is closed (offline decay), so coming back
after a week gives you a pet that genuinely missed you.

### 4. Your Sanctuary / Sanctuary-mu
Adopt as many companions as you like with `N`. Each one is simulated individually:
`[` and `]` hop between them, `L` opens the roster with full vitals for every pet,
and `★` marks whoever is currently on screen.

---

## 💾 Save files / Berkas simpanan

| File | Purpose |
|---|---|
| `mochigotchi_save.json` | Human-readable copy of the active companion |
| `mochigotchi_save.db` | SQLite: `pet_saves` (active pet) + `sanctuary_saves` (full roster) |

Older single-pet saves (including `save.json` / `rustagotchi.db`) load automatically
and are upgraded into a one-pet Sanctuary — nothing is lost.
<br>*Save lama satu-pet otomatis dimuat dan di-upgrade jadi Sanctuary berisi satu
sahabat — tidak ada yang hilang.*

Override locations with `--json-path` and `--db-path`.

---

## 🛠️ Development / Pengembangan

```bash
cargo check --all-targets       # type-check everything
cargo test                      # 27 unit + integration tests
cargo clippy --all-targets -- -D warnings   # zero warnings
cargo build --release
```

### Project layout / Struktur proyek
```
src/
├── lib.rs       crate exports
├── main.rs      CLI entry point
├── pet.rs       species catalogue, lifecycle, cooldowns, Sanctuary
├── sprites.rs   ASCII art engine (templates + mood expressions)
├── storage.rs   dual JSON + SQLite persistence
├── minigame.rs  Snack Catcher arcade logic
└── tui.rs       adoption wizard, habitat, roster, responsive layout
tests/
└── integration_tests.rs
```

### Architecture notes / Catatan arsitektur
* Sprites are **composited** from body templates plus mood-driven expression slots,
  which keeps 10 species × 5 stages × 6 moods (300 combinations) animated without
  300 copies of hand-typed art.
* The TUI never assumes a terminal size: layout tiers are chosen from the width
  (< 78 → Compact, < 110 → Standard, ≥ 110 → Wide) and all panels are `Min`/`Length`
  constrained so nothing panics even at 50×18.
* `Storage` writes JSON first, then SQLite, and always mirrors the active pet into
  the classic `pet_saves` table so older tooling keeps reading your save correctly.

---

## 📜 License

MIT — go hatch something.

🍡 *Mochigotchi v0.3.0 — Sanctuary & Mythical Menagerie Edition*
