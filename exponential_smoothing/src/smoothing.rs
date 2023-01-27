
fn compute_alpha(time_constant: f64, sample_rate: i32) -> f64 {
    let alpha = 1.0 - (-1.0 / ((sample_rate as f64) * time_constant)).exp();
    return alpha;
}


pub struct Smoother {
    sample_rate: i32,
    initial_state: f64,
    time_constant: f64,
    previous_value: f64
}

impl Smoother {
    pub fn new() -> Smoother {
        Smoother{
            sample_rate: 0,
            initial_state: 0.0,
            time_constant: 0.0,
            previous_value: 0.0
        }
    }

    pub fn setup(&mut self) {
        self.previous_value = self.initial_state;
    }

    pub fn set_sample_rate(&mut self, rate: i32) {
        self.sample_rate = rate;
    }

    pub fn set_initial_state(&mut self, state: f64) {
        self.initial_state = state;
    }

    pub fn set_time_contant(&mut self, constant: f64) {
        self.time_constant = constant;
    }
}

pub struct ExponentialSmoother {
    parent: Smoother,
    alpha: f64
}

impl ExponentialSmoother {
    pub fn new() -> ExponentialSmoother {
        ExponentialSmoother {
            parent: Smoother::new(),
            alpha: 0.0
        }
    }

    pub fn setup (&mut self) {
        self.parent.setup();
        self.alpha = compute_alpha(self.parent.time_constant, self.parent.sample_rate);
    }

    pub fn step(&mut self, sample: f64) -> f64 {
        let smoothed_sample =
            self.alpha * sample +
            (1.0 - self.alpha) * self.parent.previous_value;
        self.parent.previous_value = smoothed_sample;

        return smoothed_sample;
    }

    pub fn set_sample_rate(&mut self, rate: i32) {
        self.parent.set_sample_rate(rate);
    }

    pub fn set_initial_state(&mut self, state: f64) {
        self.parent.set_initial_state(state);
    }

    pub fn set_time_contant(&mut self, constant: f64) {
        self.parent.set_time_contant(constant);
    }
}
