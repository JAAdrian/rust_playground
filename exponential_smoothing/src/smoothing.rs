struct Smoother {
    initial_state: f64,
    previous_value: f64,
    alpha: f64,
}

impl Smoother {
    pub fn new() -> Smoother {
        Self {
            initial_state: 0.0,
            previous_value: 0.0,
            alpha: 0.5,
        }
    }

    pub fn set_initial_state(mut self, state: f64) {
        self.initial_state = state;
    }

    pub fn step(mut self, sample: f64) -> f64 {
        let smoothed_sample =
            self.alpha * sample +
            (1.0 - self.alpha) * self.previous_value;
        self.previous_value = smoothed_sample;

        return smoothed_sample;
    }
}
