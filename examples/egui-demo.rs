use egui_raylib::EguiRaylib;

use egui_demo_lib::DemoWindows;
use sola_raylib::prelude::*;

fn main() {
    let (mut rl, thread) = sola_raylib::init()
        .size(640, 480)
        .title("Hello, World")
        .resizable()
        .build();
    let mut egui = EguiRaylib::default();
    let mut demo = DemoWindows::default();

    while !rl.window_should_close() {
        demo.logic(egui.ctx());

        egui.run(&mut rl, |ui| {
            demo.ui(ui);
        });

        rl.draw(&thread, |mut d| {
            d.clear_background(Color::WHITE);
            d.draw_text("Hello, world!", 100, 100, 20, Color::BLACK);

            egui.draw(&mut d, &thread).unwrap();
        });
    }
}
