# invaders

This is a command-line implementation of the classic arcade game Space Invaders, written in Rust. It uses the crossterm crate to handle terminal input and output, and the rusty_audio crate to play sound effects. The game features a player that can move left and right and shoot bullets, as well as a horde of invaders that move down the screen. The game supports audio effects for various actions, such as moving, shooting, and winning or losing. The game loop runs in a separate thread to allow for smooth rendering and input handling. The game ends when the player is hit by an invader or when all invaders are destroyed. The terminal is restored to its original state after the game ends.

## Project Demo

![Demo](/resources/invaders.gif)

## Usage Instructions

Before getting started, ensure that you have Rust installed on your system.

1. Clone this repository to your local machine:

```bash
git clone https://github.com/Telmo-Sousa/invaders.git
```

2. Navigate to the `invaders` directory:

```bash
cd invaders
```

3. Run the necessary Rust related commands:

```bash
cargo run
```

## Key Bindings

| Key | Action |
| --- | --- |
| `left arrow` | Move left |
| `right arrow` | Move right |
| `space` | Shoot |
| `enter` | Shoot |
| `q` | Quit |
