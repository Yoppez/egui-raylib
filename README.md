# egui-raylib

Egui bindings for Raylib.

Built against egui 0.35 and raylib 6.0.

## Installation

Inside the `Cargo.toml` file:

```toml
[dependencies]
raylib = "6.0" # you can use any raylib feature
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

You can check the examples for testing the default egui demo.

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
- No URL opening (although it can be implemented with the help of other
  libraries)
- No image copy/pasting

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your
option.

See [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT) for details.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in egui-raylib by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
