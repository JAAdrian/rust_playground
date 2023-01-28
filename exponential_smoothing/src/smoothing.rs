/// The smoothing base class. This holds common variables for all smoothers.
pub struct Smoother {
    sample_rate: i32,
    initial_state: f64,
    time_constant: f64,
    previous_value: f64,
}

/// The common smoothing methods as a trait/interface.
pub trait SmootherMethods {
    fn setup(&mut self);
    fn step(&mut self, sample: &f64) -> f64;
    fn set_sample_rate(&mut self, rate: i32);
    fn set_initial_state(&mut self, state: f64);
    fn set_time_contant(&mut self, constant: f64);
}

impl SmootherMethods for Smoother {
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

/// An alpha-beta smoother for better performance with non-stationary signals.
pub struct AlphaBetaSmoother {
    parent: Smoother,
    sample_period: f64,
    time_constant_velocity: f64,
    previous_velocity: f64,
    alpha: f64,
    beta: f64,
}

pub trait AlphaBetaSmootherMethods: SmootherMethods {
    fn set_time_contant_velocity(&mut self, constant: f64);
}

impl SmootherMethods for AlphaBetaSmoother {
    fn setup(&mut self) {
        self.parent.setup();

        self.alpha = compute_alpha(self.parent.time_constant, self.parent.sample_rate);
        self.beta = compute_alpha(self.time_constant_velocity, self.parent.sample_rate);
        self.sample_period = 1.0 / (self.parent.sample_rate as f64);
    }

    fn step(&mut self, sample: &f64) -> f64 {
        let position = self.parent.previous_value + self.sample_period * self.previous_velocity;
        let velocity = self.previous_velocity;

        let residual = sample - position;

        let new_position = position + self.alpha * residual;
        let new_velocity = velocity + self.beta * residual / self.sample_period;

        self.parent.previous_value = new_position;
        self.previous_velocity = new_velocity;

        return new_position;
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

impl AlphaBetaSmootherMethods for AlphaBetaSmoother {
    fn set_time_contant_velocity(&mut self, constant: f64) {
        self.time_constant_velocity = constant;
    }
}

/// Compute alpha value from time constant and sample rate.
fn compute_alpha(time_constant: f64, sample_rate: i32) -> f64 {
    let alpha = 1.0 - (-1.0 / ((sample_rate as f64) * time_constant)).exp();
    return alpha;
}

fn new_smoother() -> Smoother {
    Smoother {
        sample_rate: 0,
        initial_state: 0.0,
        time_constant: 0.0,
        previous_value: 0.0,
    }
}

pub fn new_exponential_smoother() -> ExponentialSmoother {
    ExponentialSmoother {
        parent: new_smoother(),
        alpha: 0.0,
    }
}

pub fn new_alpha_beta_smoother() -> AlphaBetaSmoother {
    AlphaBetaSmoother {
        parent: new_smoother(),
        sample_period: 0.0,
        time_constant_velocity: 0.0,
        previous_velocity: 0.0,
        alpha: 0.0,
        beta: 0.0,
    }
}
