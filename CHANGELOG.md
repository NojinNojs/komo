# 🎉 KOMO Patch Notes & Feature Changelog

Every update keeps your existing companion save files (`~/.local/share/mochigotchi/`) 100% safe.
Whenever you launch a newly updated version of `komo`, the **What's New** card will automatically appear inside your handheld screen!

---

## ✨ Version 1.1.0 — The Living Companion Update

### 🏡 Welcome Home Hosting
- Your companion now notices how long you were away and **runs up to the front glass** to welcome you home!
- Choose how to greet them back (`Pat head`, `Share a treat`, or `Ask what they did while you were out`) — after a medium break, they might even bring you a lucky coin they found in the garden.

### 🎀 Companion Wardrobe & Accessories (`[V]`)
- Open the new **Wardrobe (`[V]`)** anytime to customize your companion's look:
  - **Coat Palettes**: Classic, Sakura Mochi, Matcha Cream, Shiba Gold, Calico Sunset, Midnight Tuxedo, and Starlight Prism.
  - **Equippable Accessories**: Silk Ribbon, Bell Collar, Cozy Scarf, Tiny Crown, Wizard Hat, and Starlight Halo.

### 📸 Pixel Photo Booth & Share Cards (`[X]` / `komo --share`)
- Snap a crisp, retro handheld **Photo Card** of your companion inside their room!
- Press **`[X]`** in-game (or run `komo --share 16:9|9:16|4:3|3:4`) to save a ready-to-share PNG card for Desktop/GitHub (`16:9`), Instagram/TikTok Stories (`9:16`), or Posts (`4:3` & `3:4`).

### 🍱 Lifelike Daily Pacing, Real Weight & Expressive Animations
- **Safe Lunch & Sleep Breaks**: Leaving your companion for lunch, classes, or overnight sleep will no longer cause sudden death. They gently build appetite or curl up for a nap while waiting for you.
- **Realistic Weight (`g` / `kg`)**: Companions now grow from a `120g` Egg and `450g` Baby into a `1.80kg` Teen, `4.20kg` Adult, and `6.50kg` Starlight Elder.
- **New Room Animations**: Watch your companion eat from a **steaming food bowl** (`[F]`), or spot **buzzing flies and stink waves** when their room gets messy and needs a warm bath (`[C]`).
- **Curated Starter Trio**: New keepers now begin their journey by choosing from the three signature companions: **🐱 Neko Mochi**, **🐶 Boba Pup**, or **🐰 Cloud Bun**.

---

## 🔄 How to Update KOMO (Without Losing Your Companion!)

### Linux & NixOS (if installed in `~/.local/bin/komo`)
```bash
cd komo
git pull
cargo build --release
cp target/release/mochigotchi ~/.local/bin/komo
```

### macOS, Linux & Windows (via Cargo Git Install)
```bash
cargo install --git https://github.com/NojinNojs/komo.git --force
```
*(On Linux/NixOS, if you also use the `komo` alias in `~/.local/bin`, copy `~/.cargo/bin/mochigotchi` to `~/.local/bin/komo`.)*

Once updated, simply run `komo` — your companion will greet you with the **What's New** popup automatically!
