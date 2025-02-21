# WWHD Trainer - Plugin for Aroma

> [!WARNING]  
> This plugin (and rust-wiiu) is still in development. Expect bugs and missing features. The documentation was quickly put together and may contain dated information - once development has stabilized, proper documentation will be provided.

A plugin for the Aroma Homebrew Environment which has a range of cheats, modifications and information about Wind Waker HD. This plugin is written for speedrun practice but can be used by everybody who wants to do shenanigans inside WWHD.

The goal is to have a standalone (i.e. no PC required) trainer which can be controlled from inside the game. The idea was loosely inspired by the [Wind Waker Practice Rom](https://github.com/zsrtww/tww-gz). Other trainers depend of TCPGecko (and therefore Tiramisu) and a PC app.

If you are interested in WWHD speedrunning you might want to [join our Discord server](https://discord.gg/35u82nGCdF). Here you will find all resources about WWHD. You can also check out the [speedrun.com leaderboard](https://www.speedrun.com/twwhd).

This plugin contains:
- Health Cheats
- Item Cheats
- Stage Loader
- Positional Viewer (TBD)

# Install

1. Copy `wwhd-trainer.wps` to the plugins folder of your Wii U SD card
2. Reboot Wii U

# Usage

1. Launch WWHD
2. Hold `L + R` for the menu to appear
3. Navigate the menu with the displayed buttons
    - `D-Left` + `D-Right` for navigation
    - `D-Up` + `D-Down` for selection (when possible)
    - `A` to accept / move into
    - `B` go back

While the "menu buttons" (`L + R`) are held, no button presses or holds are registered by the game. Analog sticks still work.

# Build

1. Install and configure the rust-wiiu dependencies (devkitPro & Rust)
2. ```git clone rust-wiiu/wwhd-trainer```
3. ```cargo make --profile release build```

---
If you use this project in your content (such as tutorials or streams), please consider including a link to either this repo ([https://github.com/rust-wiiu/wwhd-trainer](https://github.com/rust-wiiu/wwhd-trainer)) or the WWHD Speedrun Discord server ([https://discord.gg/35u82nGCdF](https://discord.gg/35u82nGCdF)) in the description. This will help others find the tool.
