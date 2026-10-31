mod input;
mod output;
mod renderer;
mod textures;

use std::collections::HashMap;

use egui::{ClippedPrimitive, Context, TextureId, TexturesDelta, Ui};
use sola_raylib::{error::Error, prelude::*};

pub struct EguiRaylib {
    ctx: egui::Context,
    textures: HashMap<TextureId, Texture2D>,
    pixels_per_point: f32,
    last_textures_delta: Option<TexturesDelta>,
    last_clipped_primitives: Option<Vec<ClippedPrimitive>>,
}

impl EguiRaylib {
    pub fn new(ctx: egui::Context, pixels_per_point: f32) -> Self {
        Self {
            ctx,
            textures: HashMap::new(),
            last_textures_delta: None,
            last_clipped_primitives: None,
            pixels_per_point,
        }
    }

    pub fn ctx(&self) -> &Context {
        &self.ctx
    }

    pub fn run(&mut self, rl: &mut RaylibHandle, run_ui: impl FnMut(&mut Ui)) {
        let raw_input: egui::RawInput = input::gather_input(rl, self.pixels_per_point);

        let full_output = self.ctx.run_ui(raw_input, run_ui);

        output::handle_platform_output(rl, full_output.platform_output);

        self.pixels_per_point = full_output.pixels_per_point;
        self.last_clipped_primitives = Some(
            self.ctx
                .tessellate(full_output.shapes, full_output.pixels_per_point),
        );

        self.last_textures_delta = Some(full_output.textures_delta);
    }

    pub fn draw(&mut self, d: &mut RaylibDrawHandle, thread: &RaylibThread) -> Result<(), Error> {
        if let Some(textures_delta) = &mut self.last_textures_delta {
            match textures::handle_textures(&mut self.textures, d, thread, textures_delta) {
                Ok(()) => self.last_textures_delta = None,
                Err(err) => return Err(err),
            }
        }

        if let Some(clipped_primitives) = self.last_clipped_primitives.take() {
            renderer::render(
                d,
                self.pixels_per_point,
                &clipped_primitives,
                &self.textures,
            );
        }

        Ok(())
    }
}

impl Default for EguiRaylib {
    fn default() -> Self {
        Self::new(egui::Context::default(), 1.)
    }
}
