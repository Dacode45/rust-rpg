use ggez::event::{Keycode};

#[derive(Debug,Default)]
pub struct InputState {

    pub just_pressed_xaxis: Option<f32>,
    pub holding_xaxis: Option<f32>, 
    pub xaxis: f32,


    pub just_pressed_yaxis: Option<f32>,
    pub holding_yaxis: Option<f32>,
    pub yaxis: f32,
    pub fire: bool,
}

impl InputState {
    pub fn key_down(&mut self, keycode: Keycode) {
        match keycode {
            Keycode::Up => {
                self.yaxis = -1.0;
            }
            Keycode::Down => {
                self.yaxis = 1.0;
            }
            Keycode::Left => {
                self.xaxis = -1.0;
            }
            Keycode::Right => {
                self.xaxis = 1.0;
            }
            Keycode::Space => {
                self.fire = true;
            }
            _ => (),
        }
        self.advance();
    }

    // advance a frame clear just pressed
    pub fn advance(&mut self) {
        // xasis
        match (self.just_pressed_xaxis, self.holding_xaxis) {
            (None, None) if self.xaxis != 0.0 => {
                self.just_pressed_xaxis = Some(self.xaxis);
            },
            (Some(_), None) => {
                self.just_pressed_xaxis = None;
                self.holding_xaxis = Some(self.xaxis);
            },
            _ => ()
        }
        // yaxis
         match (self.just_pressed_yaxis, self.holding_yaxis) {
            (None, None) if self.yaxis != 0.0 => {
                self.just_pressed_yaxis = Some(self.yaxis);
            },
            (Some(_), None) => {
                self.just_pressed_yaxis = None;
                self.holding_yaxis = Some(self.yaxis);
            },
            _ => ()
        }
    }

    pub fn key_up(&mut self, keycode: Keycode) {
        match keycode {
            Keycode::Up => {
                self.yaxis = 0.0;
            }
            Keycode::Down => {
                self.yaxis = 0.0;
            }
            Keycode::Left => {
                self.xaxis = 0.0;
            }
            Keycode::Right => {
                self.xaxis = 0.0;
            }
            Keycode::Space => {
                self.fire = false;
            }
            _ => (), // Do nothing
        }
        // xasis
        match (self.just_pressed_xaxis, self.holding_xaxis) {
            (_, Some(_)) => {
                self.just_pressed_xaxis = None;
                self.holding_xaxis = None;
            },
            (Some(_), _) => {
                self.just_pressed_xaxis = None;
                self.holding_xaxis = None;
            },
            _ => ()
        }
        // yaxis
        match (self.just_pressed_yaxis, self.holding_yaxis) {
            (_, Some(_)) => {
                self.just_pressed_yaxis = None;
                self.holding_yaxis = None;
            },
            _ => ()
        }
    }
}