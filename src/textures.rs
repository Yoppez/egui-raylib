use std::collections::HashMap;

use egui::{ImageData, TextureId, TexturesDelta};
use sola_raylib::{error::Error, prelude::*};

fn map_texture_filter(filter: egui::TextureFilter) -> TextureFilter {
    match filter {
        egui::TextureFilter::Nearest => TextureFilter::TEXTURE_FILTER_POINT,
        egui::TextureFilter::Linear => TextureFilter::TEXTURE_FILTER_BILINEAR,
    }
}

fn map_texture_wrap_mode(wrap: egui::TextureWrapMode) -> TextureWrap {
    match wrap {
        egui::TextureWrapMode::ClampToEdge => TextureWrap::TEXTURE_WRAP_CLAMP,
        egui::TextureWrapMode::Repeat => TextureWrap::TEXTURE_WRAP_REPEAT,
        egui::TextureWrapMode::MirroredRepeat => TextureWrap::TEXTURE_WRAP_MIRROR_REPEAT,
    }
}

pub(crate) fn handle_textures(
    map: &mut HashMap<TextureId, Texture2D>,
    d: &mut RaylibDrawHandle,
    thread: &RaylibThread,
    textures_delta: &mut TexturesDelta,
) -> Result<(), Error> {
    for (id, image_deltas) in textures_delta.set.drain() {
        for image_delta in image_deltas {
            let image_width = image_delta.image.width();
            let image_height = image_delta.image.height();

            let options = image_delta.options;

            match &image_delta.image {
                ImageData::Color(color_image) => {
                    let bytes = color_image
                        .pixels
                        .iter()
                        .flat_map(|x| [x.r(), x.g(), x.b(), x.a()])
                        .collect::<Vec<u8>>();

                    if let Some(texture) = map.get_mut(&id) {
                        texture
                            .set_texture_filter(thread, map_texture_filter(options.minification));
                        texture.set_texture_wrap(thread, map_texture_wrap_mode(options.wrap_mode));
                        if let Some(pos) = image_delta.pos {
                            texture.update_texture_rec(
                                Rectangle::new(
                                    pos[0] as f32,
                                    pos[1] as f32,
                                    image_width as f32,
                                    image_height as f32,
                                ),
                                &bytes,
                            )?;
                        } else {
                            if image_width as i32 == texture.width
                                && image_height as i32 == texture.height
                            {
                                texture.update_texture(&bytes)?;
                            } else {
                                let image = Image::gen_image_color(
                                    image_width as i32,
                                    image_height as i32,
                                    Color::BLACK,
                                );
                                *texture = d.load_texture_from_image(thread, &image)?;
                                texture.update_texture(&bytes)?;
                            }
                        }
                    } else {
                        let rl_image = Image::gen_image_color(
                            image_width as i32,
                            image_height as i32,
                            Color::BLACK,
                        );
                        let mut new_texture = d.load_texture_from_image(thread, &rl_image)?;
                        new_texture
                            .set_texture_filter(thread, map_texture_filter(options.minification));
                        new_texture
                            .set_texture_wrap(thread, map_texture_wrap_mode(options.wrap_mode));
                        new_texture.update_texture(&bytes)?;

                        map.insert(id, new_texture);
                    }
                }
            }
        }
    }

    for id in textures_delta.free.drain() {
        map.remove(&id);
    }

    Ok(())
}
