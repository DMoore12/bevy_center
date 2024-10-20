# Bevy Center

[![crates.io](https://img.shields.io/crates/v/bevy_center.svg)](https://crates.io/crates/bevy_center)
[![docs](https://docs.rs/bevy_center/badge.svg)](https://docs.rs/bevy_center)
[![license](https://img.shields.io/crates/l/bevy_center)](https://github.com/DMoore12/bevy_center#license)
[![crates.io](https://img.shields.io/crates/d/bevy_center.svg)](https://crates.io/crates/bevy_center)

A [Bevy](https://bevyengine.org) debug plugin that draws a colored marker to the center of the screen on startup.

## Usage

```rust
use bevy::prelude::*;
use bevy_center::prelude::*;

fn main() {
    App::new()
        // Draws a 2x2 pixel, red center marker
        .add_plugins(CenterMarkerPlugin)
        .run();
}
```