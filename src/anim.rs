use std::cmp;

pub struct Animation {
    frames: Vec<usize>,
    should_loop: bool,
    spf: f32,

    index: usize,
    time: f32,
}

impl Animation {
    pub fn new(frames: Vec<usize>, should_loop: bool, spf: f32) -> Self {
        let spf = if spf <= 0.0 { 0.12 } else { spf };
        Animation {
            frames,
            should_loop,
            spf,

            index: 0,
            time: 0.0,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.time = self.time + dt;

        if self.time >= self.spf {
            self.index = self.index + 1;
            self.time = 0.0;

            if self.index >= self.frames.len() {
                if self.should_loop {
                    self.index = 0;
                } else {
                    self.index = self.frames.len() - 1;
                }
            }
        }
    }

    pub fn set_frames(&mut self, frames: Vec<usize>) {
        self.frames = frames;
        self.index = cmp::min(self.index, self.frames.len() - 1);
    }

    pub fn frame(&self) -> usize {
        return self.frames[self.index];
    }

    pub fn is_finished(&self) -> bool {
        !self.should_loop && self.index == self.frames.len() - 1
    }
}
