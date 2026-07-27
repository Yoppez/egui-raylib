# egui-raylib

Egui bindings for Raylib.

Built against egui 0.35 and raylib 6.0.

## Installation

Inside the `Cargo.toml` file:

```toml
[dependencies]
egui-raylib = "0.1"
```

## Usage

A minimal usage of this library is like this:

```Rust
use raylib::prelude::*;

use egui_raylib::EguiRaylib;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(640, 480)
        .title("Egui demo")
        .resizable()
        .build();

    let mut egui = EguiRaylib::default();

    // Initialization code

    while !rl.window_should_close() {
        // Logic

        egui.run(&mut rl, |ui| {
            // Ui
        });

        rl.draw(&thread, |mut d| {
            d.clear_background(Color::BLACK);
            
            // Draw before egui

            egui.draw(&mut d, &thread).unwrap();

            // Draw after egui
        });
    }
}
```

## Todo

These features are not yet implemented, maybe they will be in the future:

- [ ] Inputs for PointerGone and WindowFocused
- [ ] Touch-screen inputs
- [ ] Custom mouse cursor

## Raylib limitations

There are egui features that are not implemented because of Raylib limitations:

- No files drag and drop
- No OS theme awareness (light or dark mode)
- No IME
- Not all mouse cursor types
- No URL opening (although it can be implemented with the help of other libraries)
- No image copy/pasting
