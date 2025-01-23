# GAT - GlazeWM Alternating Tiler, written in Rust.

## What is this?
This is an auto tiler for the [GlazeWM](https://github.com/glzr-io/glazewm) window manager. It is written in Rust
which allows for easy and portable compilation.

Any time a window is focused, the tiling direction of the window will be set that a new window will be placed in the
direction with the highest length. 

i.e.

```
-----------------------
|                     |
|                     |
|      A       B      |
|                     |
|                     |
-----------------------
```
becomes

-----------------------
|                     |

It is a fork of the archived [GAT-GWM](https://github.com/ParasiteDelta/GAT-GWM) project by [ParasiteDelta](https://github.com/ParasiteDelta), but has mostly been rewritten.

## Installation

### Dependencies

- Rust (Can be installed with [rustup](https://rustup.rs/))

### Installing with Cargo

```bash
cargo install --git https://github.com/Dutch-Raptor/GAT-GWM.git --features=no_console
```

> If ~/.cargo/bin is in your PATH, you can run `gat-gwm` from anywhere.

### Building from source

```bash
git clone https://github.com/glzr-io/gat-gwm
cd gat-gwm
cargo build --release
```

> The executable will be in `target/release/gat-gwm`.