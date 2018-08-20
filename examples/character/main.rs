extern crate ggez;
extern crate rpg;
extern crate tiled;

use ggez::conf;
use ggez::event::{self, EventHandler, Keycode, Mod};
use ggez::graphics;
use ggez::graphics::spritebatch::SpriteBatch;
use ggez::graphics::{Point2, Rect};
use ggez::timer;
use ggez::{Context, ContextBuilder, GameResult};

use std::env;
use std::path;

use rpg::entity::Entity;
use rpg::input::InputState;
use rpg::map::{uvs_from_tiled, Map};
use rpg::sprite::Sprite;
use rpg::state;
use rpg::tween;
use rpg::util::{self, load_tile_map};

const DESIRED_FPS: u32 = 60;

struct SharedState {
    map_sprite: Sprite,
    player_sprite: Sprite,
    map: Map,
    player: Entity,
    input: InputState,
}

struct WaitState;

impl<'a> state::State<SharedState> for WaitState {
    fn on_start(&mut self, _data: state::StateData<SharedState>) {
        let sd = _data.data;
        let sf = sd.player.start_frame;
        sd.player.set_frame(sf);
    }

    fn update(&mut self, _data: state::StateData<SharedState>) -> state::Trans<SharedState> {
        let sd = _data.data;
        if let Some(axis) = sd.input.just_pressed_axis {
            if axis != Point2::new(0.0, 0.0) {
                return state::Trans::Push(Box::new(MoveState::new(sd.input.axis)));
            }
        }
        state::Trans::None
    }
    fn state_name(&self) -> &str {
        "WaitState"
    }
}

struct MoveState {
    dir: Point2,
    tween: tween::Tween,

    start: Point2,
    should_move: bool,
}

impl MoveState {
    pub fn new(dir: Point2) -> Self {
        MoveState {
            dir,
            tween: tween::Tween::new(0.0, 1.0, 0.2),

            start: Point2::new(0.0, 0.0),
            should_move: false,
        }
    }
}

impl<'a> state::State<SharedState> for MoveState {
    fn update(&mut self, _data: state::StateData<SharedState>) -> state::Trans<SharedState> {
        let sd = _data.data;
        if !self.should_move || self.tween.is_finished() {
            return state::Trans::Pop;
        }
        self.tween
            .update(1.0 / (DESIRED_FPS as f32), &tween::ease_in_quad);
        let value = self.tween.value();
        let next = Point2::new(
            self.start.x + (value * self.dir.x) * sd.map.tile_dimensions.x,
            self.start.y + (value * self.dir.y) * sd.map.tile_dimensions.y,
        );
        sd.player.set_position(next);
        println!("value: {}", value);
        return state::Trans::None;
    }
    fn on_start(&mut self, _data: state::StateData<SharedState>) {
        let sd = _data.data;
        let next = Point2::new(
            sd.player.tile_x as f32 + self.dir.x,
            sd.player.tile_y as f32 + self.dir.y,
        );
        if next.x >= 0.0
            && next.x < sd.map.dimensions.x
            && next.y >= 0.0
            && next.y < sd.map.dimensions.y
        {
            sd.player.tile_x = next.x as usize;
            sd.player.tile_y = next.y as usize;
            self.should_move = true;
        }
        self.start = sd.player.pos;
    }
    fn on_stop(&mut self, _data: state::StateData<SharedState>) {
        let sd = _data.data;

        let (tx, ty) = (sd.player.tile_x, sd.player.tile_y);
        sd.player.teleport(tx, ty, &sd.map);
    }
    fn state_name(&self) -> &str {
        "MoveState"
    }
}

struct MainState<'a> {
    shared_state: SharedState,
    state_machine: state::StateMachine<'a, SharedState>,
}

impl<'a> MainState<'a> {
    pub fn new(
        map_sprite: Sprite,
        player_sprite: Sprite,
        mut map: Map,
        mut player: Entity,
    ) -> GameResult<MainState<'a>> {
        let camera = Rect::new(0.0, 0.0, map.pixel_dimensions.x, map.pixel_dimensions.y);
        map.camera = camera;
        player.teleport(10, 2, &map);

        Ok(MainState {
            shared_state: SharedState {
                map_sprite,
                player_sprite,

                map,
                player,

                input: InputState::default(),
            },
            state_machine: state::StateMachine::new(WaitState),
        })
    }
}

impl<'a> EventHandler for MainState<'a> {
    fn update<'b>(&mut self, ctx: &'b mut Context) -> GameResult<()> {
        while timer::check_update_time(ctx, DESIRED_FPS) {
            let sm = &mut self.state_machine;
            if sm.is_running() {
                sm.update(state::StateData::new(&mut self.shared_state));
            } else {
                sm.start(state::StateData::new(&mut self.shared_state))
            }
            self.shared_state.input.update(ctx);
            // println!("pos {:?} {}", self.input.just_pressed_xaxis, self.hero_tile_x);
            // xaxis
            // if let Some(xaxis) = self.input.just_pressed_xaxis {
            //     let next = self.player.tile_x as f32 + xaxis;
            //     if next >= 0.0 && next < self.map.dimensions.x {
            //         self.player.tile_x = next as usize;
            //     }
            // }
            // if let Some(yaxis) = self.input.just_pressed_yaxis {
            //     let next = self.player.tile_y as f32 + yaxis;
            //     if next >= 0.0 && next < self.map.dimensions.y {
            //         self.player.tile_y = next as usize;
            //     }
            // }

            // let (tx, ty) = (self.player.tile_x, self.player.tile_y);
            // self.player.teleport(tx, ty, &self.map);
            // self.input.advance();
        }
        // println!("Done");
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
        {
            let s = self.shared_state
                .map_sprite
                .with_context(&self.shared_state.map);
            graphics::draw(ctx, &s, Point2::new(0.0, 0.0), 0.0);
        }
        {
            let s = self.shared_state
                .player_sprite
                .with_context(&self.shared_state.player);
            graphics::draw(ctx, &s, Point2::new(0.0, 0.0), 0.0);
        }
        graphics::present(ctx);

        timer::yield_now();
        Ok(())
    }

    // Handle key events.  These just map keyboard events
    // and alter our input state appropriately.
    fn key_down_event(&mut self, ctx: &mut Context, keycode: Keycode, _keymod: Mod, _repeat: bool) {
        self.shared_state
            .input
            .key_down_event(ctx, keycode, _keymod, _repeat);
        if keycode == Keycode::Escape {
            ctx.quit().unwrap()
        }
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: Keycode, _keymod: Mod, _repeat: bool) {
        self.shared_state
            .input
            .key_up_event(_ctx, keycode, _keymod, _repeat);
    }
}

fn main() {
    let mut cb = ContextBuilder::new("tilemap", "ggez")
        .window_setup(conf::WindowSetup::default().title("Tilemap!"))
        .window_mode(conf::WindowMode::default().dimensions(256 * 2, 224 * 2));

    // We add the CARGO_MANIFEST_DIR/resources to the filesystems paths so
    // we we look in the cargo project for files.
    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        println!("Adding path {:?}", path);
        // We need this re-assignment alas, see
        // https://aturon.github.io/ownership/builders.html
        // under "Consuming builders"
        cb = cb.add_resource_path(path);
    } else {
        println!("Not building from cargo?  Ok.");
    }

    let ctx = &mut cb.build().unwrap();

    ctx.print_resource_stats();
    graphics::set_background_color(ctx, (0, 0, 0, 255).into());

    let mut image = graphics::Image::new(ctx, "/character/rpg_indoor.png").unwrap();
    let tilemap = load_tile_map(ctx, "/character/small_room.tmx").unwrap();
    let mut sprite = Sprite::new(image, 0.0, 0.0);
    sprite.uvs = uvs_from_tiled(&tilemap, 0);
    let map = Map::new(&tilemap, 0, 0);

    let mut p_image = graphics::Image::new(ctx, "/character/walk_cycle.png").unwrap();
    let p_sprite = Sprite::new(p_image, 16.0, 24.0);
    let mut player = Entity::new(Point2::new(16.0, 24.0), 9);

    let mut game = MainState::new(sprite, p_sprite, map, player).unwrap();
    let result = event::run(ctx, &mut game);
    if let Err(e) = result {
        println!("Error encountered running game: {}", e);
    } else {
        println!("Game exited cleanly.");
    }
}
