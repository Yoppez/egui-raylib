use std::collections::HashMap;

use egui::{ClippedPrimitive, TextureId, epaint::Primitive};
use sola_raylib::{
    ffi::{
        RL_TRIANGLES, rlBegin, rlColor4ub, rlDisableBackfaceCulling, rlDisableDepthTest,
        rlDisableTexture, rlEnd, rlSetTexture, rlTexCoord2f, rlVertex2f,
    },
    prelude::*,
};

pub(crate) fn render(
    d: &mut RaylibDrawHandle,
    pixels_per_point: f32,
    primitives: &[ClippedPrimitive],
    texture_map: &HashMap<TextureId, Texture2D>,
) {
    unsafe {
        rlDisableDepthTest();
        rlDisableBackfaceCulling();
    }

    d.draw_blend_mode(BlendMode::BLEND_ALPHA_PREMULTIPLY, |mut d| {
        for primitive in primitives {
            match &primitive.primitive {
                Primitive::Mesh(mesh) => {
                    if let Some(texture) = texture_map.get(&mesh.texture_id) {
                        let vertices = &mesh.vertices;
                        let indices = &mesh.indices;
                        let clip = primitive.clip_rect * pixels_per_point;
                        d.draw_scissor_mode(
                            clip.min.x as i32,
                            clip.min.y as i32,
                            clip.max.x as i32 - clip.min.x as i32,
                            clip.max.y as i32 - clip.min.y as i32,
                            |_| {
                                unsafe {
                                    rlSetTexture(texture.id);
                                    rlBegin(RL_TRIANGLES as i32);
                                }
                                for indices_triple in indices.as_chunks::<3>().0 {
                                    for index in indices_triple {
                                        let vertex = vertices[*index as usize];

                                        unsafe {
                                            rlColor4ub(
                                                vertex.color.r(),
                                                vertex.color.g(),
                                                vertex.color.b(),
                                                vertex.color.a(),
                                            );

                                            rlTexCoord2f(vertex.uv.x, vertex.uv.y);
                                            rlVertex2f(
                                                vertex.pos.x * pixels_per_point,
                                                vertex.pos.y * pixels_per_point,
                                            );
                                        }
                                    }
                                }
                                unsafe {
                                    rlEnd();
                                    rlDisableTexture();
                                }
                            },
                        );
                    }
                }
                Primitive::Callback(_) => (),
            };
        }
    });
}
