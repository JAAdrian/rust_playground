/// The smoothing base class. This holds common variables for all smoothers.
pub struct Smoother {
    sample_rate: i32,
    initial_state: f64,
    time_constant: f64,
    previous_value: f64,
}

/// The common smoothing methods as a trait/interface.
pub trait SmootherMethods {
    fn new() -> Self;

    fn setup(&mut self);
    fn step(&mut self, sample: &f64) -> f64;
    fn set_sample_rate(&mut self, rate: i32);
    fn set_initial_state(&mut self, state: f64);
    fn set_time_contant(&mut self, constant: f64);
}

impl SmootherMethods for Smoother {
    fn new() -> Smoother {
        Smoother {
            sample_rate: 0,
            initial_state: 0.0,
            time_constant: 0.0,
            previous_value: 0.0,
        }
    }

    fn setup(&mut self) {
        self.previous_value = self.initial_state;
    }

    fn step(&mut self, sample: &f64) -> f64 {
        return 0.0;
    }

    fn set_sample_rate(&mut self, rate: i32) {
        self.sample_rate = rate;
    }

    fn set_initial_state(&mut self, state: f64) {
        self.initial_state = state;
    }

    fn set_time_contant(&mut self, constant: f64) {
        self.time_constant = constant;
    }
}

/// An exponential smoother for single float inputs.
pub struct ExponentialSmoother {
    parent: Smoother,
    alpha: f64,
}

impl SmootherMethods for ExponentialSmoother {
    fn new() -> ExponentialSmoother {
        ExponentialSmoother {
            parent: Smoother::new(),
            alpha: 0.0,
        }
    }

    fn setup(&mut self) {
        self.parent.setup();
        self.alpha = compute_alpha(self.parent.time_constant, self.parent.sample_rate);
    }

    fn step(&mut self, sample: &f64) -> f64 {
        let smoothed_sample = self.alpha * sample + (1.0 - self.alpha) * self.parent.previous_value;
        self.parent.previous_value = smoothed_sample;

        return smoothed_sample;
    }

    fn set_sample_rate(&mut self, rate: i32) {
        self.parent.set_sample_rate(rate);
    }

    fn set_initial_state(&mut self, state: f64) {
        self.parent.set_initial_state(state);
    }

    fn set_time_contant(&mut self, constant: f64) {
        self.parent.set_time_contant(constant);
    }
}

/// Compute alpha value from time constant and sample rate.
fn compute_alpha(time_constant: f64, sample_rate: i32) -> f64 {
    let alpha = 1.0 - (-1.0 / ((sample_rate as f64) * time_constant)).exp();
    return alpha;
}
