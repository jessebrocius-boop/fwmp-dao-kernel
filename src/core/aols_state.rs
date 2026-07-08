use crate::core::cl31_hashing::Cl31Multivector;
use crate::imperative::mce_validator::MceValidator;
use std::collections::VecDeque;

pub struct AolsState {
    pub current_coordinate: Cl31Multivector,
    pub lookback_history: VecDeque<f64>,
    pub window_length: usize,
    pub threshold: f64,
    pub is_dark_path: bool,
    pub sandbox: MceValidator, 
}

impl AolsState {
    pub fn new(initial_coord: Cl31Multivector, window_length: usize, threshold: f64) -> Self {
        let mut history = VecDeque::with_capacity(window_length);
        history.push_back(threshold);

        AolsState {
            current_coordinate: initial_coord,
            lookback_history: history,
            window_length,
            threshold,
            is_dark_path: true,
            sandbox: MceValidator::new(),
        }
    }

    pub fn execute_transition(
        &mut self, 
        next_coord: Cl31Multivector, 
        systemic_payoff: f64
    ) -> Result<&str, &'static str> {
        if !next_coord.verify_geometric_parity() {
            return Err("TRANSITION_DENIED: Geometric parity violation. Anomalous multivector detected.");
        }

        match self.sandbox.authorize_payload(&next_coord, systemic_payoff) {
            Ok(_lambda_friction) => { },
            Err(e) => return Err(e), 
        }

        if self.lookback_history.len() == self.window_length {
            self.lookback_history.pop_front();
        }
        self.lookback_history.push_back(systemic_payoff);

        let history_sum: f64 = self.lookback_history.iter().sum();
        let mean_payoff = history_sum / (self.lookback_history.len() as f64);

        if mean_payoff >= self.threshold {
            self.current_coordinate = next_coord;
            self.is_dark_path = true;
            Ok("DARK_PATH: Transition authorized. Node state locked.")
        } else {
            self.current_coordinate = next_coord;
            self.is_dark_path = false;
            Err("INTERRUPT: Window mean-payoff dropped below target threshold. Relaying to arbitrator.")
        }
    }
}
