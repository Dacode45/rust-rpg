use ggez::{
    Context,
    GameResult,
    GameError,
    graphics::{
        Rect,
        Point2,
    }
};
use tiled;

/// Math functions

pub fn add_points(args: &[&Point2]) -> Point2 {
    args.iter().fold(Point2::new(0.0, 0.0), |mut sum, val| {
        sum.x += val.x;
        sum.y += val.y;
        sum
    })
}

#[inline]
pub fn clamp<T: PartialOrd>(input: T, min: T, max: T) -> T {
    debug_assert!(min <= max, "min must be less than or equal to max");
    if input < min {
        min
    } else if input > max {
        max
    } else {
        input
    }
}

pub fn load_tile_map(ctx: &mut Context, tilemap_src: &str) -> GameResult<tiled::Map> {
    let tilemap_file = ctx.filesystem.open(tilemap_src)?;
    match tiled::parse(tilemap_file) {
        Ok(map) => Ok(map),
        Err(_) => Err(GameError::from(String::from("tiled error"))),
    }
}

pub fn generate_uvs(i_width: f32, i_height: f32, t_width: f32, t_height: f32) -> Vec<Rect> {
    let width = t_width / i_width;
    let height = t_height / i_height;
    let cols = i_width / t_width;
    let rows = i_height / t_height;

    let mut ux: f32 = 0.0;
    let mut uy: f32 = 0.0;

    let mut uvs = Vec::new();

    for _ in 0..(rows as u32) {
        for _ in 0..(cols as u32) {
            uvs.push(Rect::new(ux, uy, width, height));
            ux += width;
        }
        ux = 0.0;
        uy += height;
    }
    return uvs;
}
