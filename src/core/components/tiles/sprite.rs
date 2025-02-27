use std::ops::Range;

use wgpu::util::BufferInitDescriptor;

use crate::{
    core::components::{
        material::Material, maths::coordinates::Coordinates, tiles::tileset::Tileset,
    },
    rendering::{gl_representations::TexturedGlVertex, scion2d::Renderable2D},
};

const INDICES: &[u16] = &[0, 1, 3, 3, 1, 2];

/// Renderable Sprite.
#[derive(Debug)]
pub struct Sprite {
    /// Desired tile to render for this material.
    tile_number: usize,
    /// Current computed content for vertex
    contents: Option<[TexturedGlVertex; 4]>,
    /// Flag to keep track of changed tile number
    dirty: bool,
}

impl Sprite {
    pub fn new(tile_number: usize) -> Self { Self { tile_number, contents: None, dirty: false } }

    /// Modify the current sprite tile number
    pub fn set_tile_nb(&mut self, new_tile_nb: usize) {
        self.tile_number = new_tile_nb;
        self.dirty = true;
    }

    fn uv_refs(&self, tileset: &Tileset) -> [Coordinates; 4] {
        let line = (self.tile_number / tileset.length) as f32;
        let column = (self.tile_number % tileset.length) as f32;

        let unit_line = 1.0 / tileset.height as f32;
        let unit_column = 1.0 / tileset.length as f32;

        let a = Coordinates::new(column * unit_column, line * unit_line);
        let b = Coordinates::new(a.x(), a.y() + unit_line);
        let c = Coordinates::new(a.x() + unit_column, a.y() + unit_line);
        let d = Coordinates::new(a.x() + unit_column, a.y());
        [a, b, c, d]
    }

    pub(crate) fn upsert_content(&mut self, material: Option<&Material>) -> [TexturedGlVertex; 4] {
        if (self.dirty || self.contents.is_none()) && material.is_some() {
            if let Material::Tileset(tileset) = material.unwrap() {
                let a = Coordinates::new(0., 0.);
                let b = Coordinates::new(0., tileset.tile_size as f32);
                let c = Coordinates::new(tileset.tile_size as f32, tileset.tile_size as f32);
                let d = Coordinates::new(tileset.tile_size as f32, 0.);
                let uvs_ref = self.uv_refs(&tileset);
                let contents = [
                    TexturedGlVertex::from((&a, &uvs_ref[0])),
                    TexturedGlVertex::from((&b, &uvs_ref[1])),
                    TexturedGlVertex::from((&c, &uvs_ref[2])),
                    TexturedGlVertex::from((&d, &uvs_ref[3])),
                ];
                self.contents = Some(contents);
            }
        }
        self.contents.as_ref().expect("A computed content is missing in Sprite component").clone()
    }

    pub(crate) fn indices() -> Vec<u16> { INDICES.to_vec() }
}

impl Renderable2D for Sprite {
    fn vertex_buffer_descriptor(&mut self, material: Option<&Material>) -> BufferInitDescriptor {
        self.upsert_content(material);
        wgpu::util::BufferInitDescriptor {
            label: Some("Sprite Vertex Buffer"),
            contents: bytemuck::cast_slice(
                self.contents.as_ref().expect("A computed content is missing in Sprite component"),
            ),
            usage: wgpu::BufferUsage::VERTEX,
        }
    }

    fn indexes_buffer_descriptor(&self) -> BufferInitDescriptor {
        wgpu::util::BufferInitDescriptor {
            label: Some("Square Index Buffer"),
            contents: bytemuck::cast_slice(&INDICES),
            usage: wgpu::BufferUsage::INDEX,
        }
    }

    fn range(&self) -> Range<u32> { 0..INDICES.len() as u32 }

    fn dirty(&self) -> bool { self.dirty }

    fn set_dirty(&mut self, is_dirty: bool) { self.dirty = is_dirty; }
}
