use ggez::graphics::{self, Point2};

use map::Map;
use sprite::{Sprite, SpriteComponent};

#[derive(Debug, Clone)]
pub struct Entity {
    pub(crate) id: String,

    pos: Point2,
    pub(crate) dimensions: Point2,

    pub(crate) tile_x: usize,
    pub(crate) tile_y: usize,
    pub(crate) layer: usize,

    frame: usize,
}

impl Entity {
    pub fn new(id: String, dimensions: Point2) -> Self {
        let zero = Point2::new(0.0, 0.0);
        Entity {
            id,

            pos: zero.clone(),
            dimensions,

            tile_x: 0,
            tile_y: 0,
            layer: 0,

            frame: 0,
        }
    }

    pub fn pos(&self) -> graphics::Point2 {
        self.pos
    }
    
    pub fn layer(&self) -> usize {
        self.layer
    }

    pub fn tile_x(&self) -> usize {
        self.tile_x
    }

    pub fn tile_y(&self) -> usize {
        self.tile_y
    }

    pub fn set_frame(&mut self, frame: usize) {
        self.frame = frame;
    }

    pub fn set_pos(&mut self, pos: Point2) {
        self.pos = pos;
    }
}

impl SpriteComponent for Entity {
    fn setup_sprite(&self, sprite: &mut Sprite) {
        if self.frame != sprite.frame() {
            sprite.set_frame(self.frame);
        }
    }
    fn draw_sprite_at(&self) -> graphics::Point2 {
        self.pos.clone()
    }
}
