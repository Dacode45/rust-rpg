use ggez::{
    self, event::{Keycode, Mod}, graphics::Point2, Context,
};

#[derive(Debug)]
pub struct InputState {
    pub just_pressed: Option<Keycode>,
    pub just_pressed_axis: Option<Point2>,

    pub axis: Point2,
    pub fire: bool,
}

impl Default for InputState {
    fn default() -> InputState {
        InputState {
            just_pressed: None,
            just_pressed_axis: None,

            axis: Point2::new(0.0, 0.0),
            fire: false,
        }
    }
}

impl ggez::event::EventHandler for InputState {
    fn update(&mut self, _: &mut Context) -> ggez::GameResult<()> {
        self.just_pressed = match self.just_pressed {
            Some(_) => None,
            None => None,
        };
        self.just_pressed_axis = match self.just_pressed_axis {
            Some(axis) => None,
            None => None,
        };
        Ok(())
    }
    fn key_down_event(&mut self, _: &mut Context, keycode: Keycode, _keymod: Mod, _repeat: bool) {
        if _repeat {
            return;
        }
        let current = self.axis;
        match keycode {
            Keycode::Up => {
                self.axis.y = -1.0;
            }
            Keycode::Down => {
                self.axis.y = 1.0;
            }
            Keycode::Left => {
                self.axis.x = -1.0;
            }
            Keycode::Right => {
                self.axis.x = 1.0;
            }
            Keycode::Space => {
                self.fire = true;
            }
            _ => (),
        }
        if current != self.axis {
            self.just_pressed_axis = Some(self.axis);
        }
    }

    fn key_up_event(&mut self, _: &mut Context, keycode: Keycode, _keymod: Mod, _repeat: bool) {
        if _repeat {
            return;
        }
        match keycode {
            Keycode::Up => {
                self.axis.y = 0.0;
            }
            Keycode::Down => {
                self.axis.y = 0.0;
            }
            Keycode::Left => {
                self.axis.x = 0.0;
            }
            Keycode::Right => {
                self.axis.x = 0.0;
            }
            Keycode::Space => {
                self.fire = false;
            }
            _ => (), // Do nothing
        }
    }

    fn draw(&mut self, ctx: &mut Context) -> ggez::GameResult<()> {
        Ok(())
    }
}
