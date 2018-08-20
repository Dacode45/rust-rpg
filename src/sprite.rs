use ggez::{
    self, graphics::{self, spritebatch::SpriteBatch, Image, Point2, Rect},
};

use util;

pub struct SpriteDrawContext<'a> {
    sprite: &'a mut Sprite,
    comp: &'a (SpriteComponent + 'a),
}

impl<'a> graphics::Drawable for SpriteDrawContext<'a> {
    fn draw_ex(&self, ctx: &mut ggez::Context, param: graphics::DrawParam) -> ggez::GameResult<()> {
        let pos = self.comp.draw_sprite_at();
        let mut param = param.clone();
        param.dest.x += pos.x;
        param.dest.y += pos.y;
        self.sprite.sprite_batch.draw_ex(ctx, param)
    }
    fn set_blend_mode(&mut self, mode: Option<graphics::BlendMode>) {}
    fn get_blend_mode(&self) -> Option<graphics::BlendMode> {
        None
    }
}

pub struct Sprite {
    pub width: f32,
    pub height: f32,

    pub sprite_batch: SpriteBatch,
    pub uvs: Vec<Rect>,
    frame: usize,
}

impl Sprite {
    pub fn new(image: graphics::Image, width: f32, height: f32) -> Self {
        let uvs = util::generate_uvs(image.width() as f32, image.height() as f32, width, height);
        Sprite {
            width,
            height,

            sprite_batch: SpriteBatch::new(image),
            uvs,
            frame: 0,
        }
    }

    pub fn frame(&self) -> usize {
        self.frame
    }

    pub fn set_frame(&mut self, frame: usize) {
        self.frame = frame;
        self.sprite_batch.clear();

        let mut param = graphics::DrawParam::default();

        param.src = self.uvs[frame];
        self.sprite_batch.add(param);
    }

    pub fn with_context<'a>(&'a mut self, comp: &'a SpriteComponent) -> SpriteDrawContext<'a> {
        comp.setup_sprite(self);
        SpriteDrawContext { sprite: self, comp }
    }
}

impl graphics::Drawable for Sprite {
    fn draw_ex(&self, ctx: &mut ggez::Context, param: graphics::DrawParam) -> ggez::GameResult<()> {
        self.sprite_batch.draw_ex(ctx, param)
    }
    fn set_blend_mode(&mut self, mode: Option<graphics::BlendMode>) {}
    fn get_blend_mode(&self) -> Option<graphics::BlendMode> {
        None
    }
}

pub trait SpriteComponent {
    fn setup_sprite(&self, sprite: &mut Sprite);
    fn draw_sprite_at(&self) -> graphics::Point2;
}
