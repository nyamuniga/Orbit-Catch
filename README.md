# Orbit Catch

Orbit Catch is a rhythm and timing-based game that embodies the "low floor, high ceiling" design philosophy. Players manipulate gravity to catch incoming projectiles into stable orbits. The game scales from a simple sensory experience to a highly complex orbital management puzzle.

## Core Mechanics

- **Pulse**: Tap or click to emit a gravitational pulse from the central Sun.
- **Moons**: Shapes that fly in from the edges.
- **Catch**: Time your pulse so it hits a Moon exactly as it crosses a designated orbital ring.
- **Survive**: Don't let un-captured Moons hit the Sun, and don't let orbiting Moons collide!

## Built With

- [Rust](https://www.rust-lang.org/)
- [Macroquad](https://macroquad.rs/)

## Dependencies

Before running or building the game, you will need to install some system dependencies depending on your operating system.

### Linux (Debian/Ubuntu)
```bash
sudo apt update
sudo apt install build-essential pkg-config libx11-dev libxi-dev libgl1-mesa-dev libasound2-dev
```

### Linux (Fedora)
```bash
sudo dnf install gcc gcc-c++ pkgconfig libX11-devel libXi-devel mesa-libGL-devel alsa-lib-devel
```

### Linux (Arch)
```bash
sudo pacman -S base-devel pkgconf libx11 libxi mesa alsa-lib
```

### macOS
Ensure you have the Xcode Command Line Tools installed:
```bash
xcode-select --install
```

### Windows
Install the [Build Tools for Visual Studio](https://visualstudio.microsoft.com/downloads/#build-tools-for-visual-studio). Make sure to select the **"Desktop development with C++"** workload during installation.

## How to Play

Run the game locally:
```bash
cargo run --release
```

## License

Dual-licensed under MIT or Apache-2.0. See `LICENSE-MIT` and `LICENSE-APACHE` for more details.
