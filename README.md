# 🍡 Mochigotchi v0.4.0 — Sanctuary, TrueColor Sprites & Real-Time Life Edition

> **Your Cozy Pocket-Pet Sanctuary in the Terminal.**  
> Adopt dragons, kitsune, phoenixes, unicorns, and cozy classics — each rendered in multi-line TrueColor ASCII art with a realistic multi-day lifecycle, an interactive Pantry, and a Welcome-Back report whenever you reopen your terminal.

---

## ✨ Highlights (EN)

1. **Detailed TrueColor Multi-Line ASCII Sprites**:
   - Every companion has hand-crafted multi-line ASCII art across all **5 evolution stages** (`Egg` -> `Baby` -> `Teen` -> `Adult` -> `Starlight Elder`) and **6 moods** (`Idle`, `Eating`, `Sleeping`, `Happy`, `Sick`, `Training`).
   - Per-character RGB colorization highlights dragon flames, horns, crowns, cheeks, waves, and starlight auras, plus a subtle breathing animation.
   - In the Adoption Wizard, press `[V]` or `[Space]` to preview any species across all 5 evolution stages before adopting!

2. **10 Collectible Companions (Cozy Classics + Mythical & Fantasy)**:
   - **Cozy Classics**: `🐱 Neko Mochi`, `🐶 Boba Pup`, `🐰 Cloud Bun`, `🐧 Star Pengu`
   - **Mythical & Fantasy**: `🐉 Ignis Drake` (Fire Dragon), `🐲 Azure Wyrm` (Celestial Sea Dragon), `🦊 Kitsune Nine`, `🐦‍🔥 Sol Phoenix`, `🦄 Astra Unicorn`, `👑 Royal Jelly`

3. **Real-Time Multi-Day Lifecycle & Offline Life Simulation**:
   - **Egg Incubation**: Eggs only accept warmth (`[W]`, `+20%` per session with a **3-minute cooldown**, or passive ambient warmth up to `99%`).
   - **Meaningful Cooldowns**: Feeding (`2m`), Playing (`2.5m`), Training (`4m`), and Headpats (`45s`) prevent instant button-mashing so your pet grows naturally over days.
   - **Real Companion Age Gates**: Reaching `Teen` requires **6 hours** together (`450 XP`, `35 Bond`), `Adult` requires **24 hours** (`1,400 XP`, `65 Bond`), and `Starlight Elder` requires **72 hours** (`3,500 XP`, `90 Bond`).
   - **Offline Welcome-Back Report**: When you close the terminal and return later, Mochigotchi simulates sleep recovery, appetite, bathroom messes (`💩`), and garden exploration (finding coins & XP for you!) and greets you with a **Welcome Back Summary Modal**.

4. **Friendly UI/UX with Care Advisor & Pantry Menu**:
   - **Live Care Advisor (`💡`)**: Always tells you in plain language what your companion needs most right now.
   - **Sanctuary Pantry (`[M]`)**: Spend earned coins on *Hearty Bento*, *Warm Boba Milk Tea*, *Soothing Herbal Soup* (cures sickness), or *Starlight Delight*, or press `[F]` for their free Favorite Snack.
   - **Multi-Pet Sanctuary (`[L]`, `[N]`, `[` / `]`)**: Collect as many companions as you want and switch between them anytime.

---

## 🇮🇩 Fitur Unggulan (Bahasa Indonesia)

1. **Sprite ASCII Multi-Baris yang Detail & Berwarna (TrueColor)**:
   - Setiap pet punya gambar ASCII detail yang berbeda di **5 tahap evolusi** (`Egg`, `Baby`, `Teen`, `Adult`, `Starlight Elder`) lengkap dengan warna RGB per bagian (tanduk, sayap, api naga, mutiara air, mahkota, dan pipi).
   - Saat memilih pet di **Adoption Wizard**, tekan `[V]` atau `[Space]` untuk mengintip wujud Egg hingga Elder sebelum mengadopsi!

2. **Simulasi Waktu Nyata (Real-Time) & Kehidupan Saat Terminal Ditutup**:
   - **Inkubasi Telur Realistis**: Telur butuh dihangatkan (`[W]`, `+20%` tiap sesi dengan jeda **3 menit**, atau hangat pasif hingga `99%`).
   - **Progresi Jangka Panjang**: Evolusi butuh kombinasi **XP + Bond + Umur Real-Time** (`Teen` = 6 jam, `Adult` = 24 jam / 1 hari, `Starlight Elder` = 72 jam / 3 hari).
   - **Laporan Welcome Back Saat Terminal Dibuka Kembali**: Saat terminal ditutup, pet tetap hidup! Kalau sedang tidur energinya pulih, kalau ditinggal lama ia bisa lapar, buang air (`💩` setiap 2x makan / beberapa jam), atau jalan-jalan di taman Sanctuary dan menemukan koin untukmu.

3. **UI/UX Ramah & Mudah Dipakai**:
   - **Care Advisor (`💡`)**: Baris panduan pintar di bagian atas layar yang selalu memberi tahu apa yang paling dibutuhkan pet kamu saat ini.
   - **Menu Pantry (`[M]`)**: Pilih berbagai makanan spesial (Bento, Boba Milk Tea, Herbal Soup, Starlight Delight) atau tekan `[F]` untuk camilan favorit gratis.
   - **Panduan Bantuan (`[?]`)**: Tekan `?` kapan saja untuk membuka panduan lengkap di dalam game.

---

## ⌨️ Controls / Tombol Kontrol

| Key | Action |
|---|---|
| `W` / `Space` | Warm egg (`+20%`, 3m cooldown) or cuddle hatched companion |
| `F` | Quick-feed Favorite Snack (free) |
| `M` | Open the **Sanctuary Pantry** menu (5 meals) |
| `P` | Play together (`+18 Happy`, `+5 coins`, `+15 XP`) |
| `T` | Give headpats (`+8 Happy`, `+5 Bond`, 45s cooldown) |
| `E` | Training session (`+22 XP`, `+10 coins`, 4m cooldown) |
| `S` | Tuck into bed / Wake up |
| `C` | Bubble bath & clean habitat (`💩`) |
| `H` | Give herbal medicine when sick |
| `G` | Play the **Snack Catcher** mini-game arcade |
| `N` | Adopt an additional companion into your Sanctuary |
| `L` | Open the **Sanctuary Roster** to inspect & switch companions |
| `[` / `]` | Cycle active companion directly from the habitat |
| `Tab` | Cycle LCD shell colorway |
| `R` | Rename active companion |
| `?` | Open in-game Care Guide |
| `Q` / `Esc` | Auto-save (JSON + SQLite) & quit |

---

## 🚀 Quick Start

```bash
cargo run --release
```

### CLI Flags

```bash
mochigotchi --status
mochigotchi --roster
mochigotchi --species dragon --theme gold --name Ignis
mochigotchi --warm
mochigotchi --feed
```
