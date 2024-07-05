# Strategem Hero - CLI Game

This is simple cli game written in Rust, copying (actually just loosely following) Strategem Hero mini-game in Helldivers 2 that available with deluxe edition of the game.

![Demo GIF](./misc/demo.gif)

## Main features

- Includes all in-game strategems as of June 8th (63 total)
- Chain of successfull strategem inputs gives a multiplier to the score
- Strategems are divided into three different levels:
  - Easy - up to 3 inputs
  - Medium - from 4 to 6 inputs
  - Hard - from 7 and above inputs
- Each level of strategems gives different score
- Tracking of best score between launches
- Subcommands to:
  - Print leaderboard: `strategem-hero leaderboard`
  - Delete all game files: `strategem-hero delete-data`

## How to play

You can install this game via `cargo install strategem-hero` command or download precompiled binary included in latest release (for Windows only).

Alternatively you can build this game from source, following next steps:

```sh
# clone repo from Github
git clone https://github.com/Milinkov-N/strategem-hero.git

# Go to the project directory
cd strategem-hero

# run the game in release mode
cargo run --release

# or run this command if you just need to build the binary
cargo build --release
```

## Game Files

As of now in version 0.7.\* the only game file is sqlite database located in
folders as follows:

- Windows: `C:\Users\<User>\AppData\Roaming\strategem-hero`
- MacOS: `$HOME/Library/Application Support/strategem-hero`
- Linux: `$HOME/.local/share/strategem-hero`

And for each minor or major version there is different folder (mostly for me to not worry about database migrations).

> Note: `strategem-hero delete-data` command deletes whole game folder.
