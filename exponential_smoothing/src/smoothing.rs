struct Smoother {
    initial_state: f64,
    previous_value: f64
}

impl Smoother {
    pub fn new() -> Smoother {
        Self {
            initial_state: 0,
            previous_value: 0
        }
    }

    pub fn set_initial_state(state: f64) {
        Self.initial_state = state;
    }
}
