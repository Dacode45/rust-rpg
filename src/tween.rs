type TweenFn = Fn(f32, f32, f32, f32) -> f32;

pub fn ease_in_quad(t: f32, b: f32, c: f32, d: f32) -> f32 {
    let t = t / d;
    c * t * t + b
}

pub struct Tween {
    distance: f32,
    start_value: f32,
    current: f32,
    total_duration: f32,
    time_passed: f32,
    is_finished: bool,
}

impl Tween {
    pub fn new(start: f32, finish: f32, total_duration: f32) -> Self {
        Tween {
            distance: finish - start,
            start_value: start,
            current: start,
            total_duration,
            time_passed: 0.0,
            is_finished: false,
        }
    }

    pub fn finish_value(&self) -> f32 {
        self.start_value + self.distance
    }

    pub fn update(&mut self, elapsed_time: f32, tween_f: &TweenFn) {
        self.time_passed = self.time_passed + (elapsed_time);
        self.current = (tween_f)(
            self.time_passed,
            self.start_value,
            self.distance,
            self.total_duration,
        );

        if self.time_passed > self.total_duration {
            self.current = self.start_value + self.distance;
            self.is_finished = true;
        }
    }

    pub fn is_finished(&self) -> bool {
        return self.is_finished;
    }

    pub fn value(&self) -> f32 {
        self.current
    }
}
