# Bevy Center

A [Bevy][bevy] debug plugin that draws a colored marker to the center of the screen on startup.

```rust
use bevy::prelude::*;
use bevy_center::prelude::*;

fn main() {
    App::new()
        .add_plugins(CenterMarkerPlugin)
        .run();
}
```