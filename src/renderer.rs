use std::collections::HashMap;

use egui::{ClippedPrimitive, TextureId, epaint::Primitive};
use raylib::prelude::*;

pub(crate) fn render(
    d: &mut RaylibDrawHandle,
    pixels_per_point: f32,
    primitives: &[ClippedPrimitive],
    texture_map: &HashMap<TextureId, Texture2D>,
) {
    d.rl_disable_depth_test();
    d.rl_disable_backface_culling();
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
                            |mut s| {
                                s.rl_set_texture(texture);
                                s.rl_draw(DrawMode::Triangles, |v| {
                                    for indices_triple in indices.chunks_exact(3) {
                                        for index in indices_triple {
                                            let vertex = vertices[*index as usize];

                                            v.color4ub((
                                                vertex.color.r(),
                                                vertex.color.g(),
                                                vertex.color.b(),
                                                vertex.color.a(),
                                            ));
                                            v.texcoord2f(vertex.uv.x, vertex.uv.y);
                                            v.vertex2f(
                                                vertex.pos.x * pixels_per_point,
                                                vertex.pos.y * pixels_per_point,
                                            );
                                        }
                                    }
                                });
                                s.rl_disable_texture();
                            },
                        );
                    }
                }
                Primitive::Callback(_) => (),
            };
        }
    });
}
