# Bevy Center

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