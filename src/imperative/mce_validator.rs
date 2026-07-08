use crate::core::cl31_hashing::Cl31Multivector;
use crate::imperative::aurora_penalty::calculate_lambda_cost;

pub struct MceValidator {
    pub minimum_cmce_threshold: f64,
}

impl MceValidator {
    pub fn new() -> Self {
        MceValidator {
            minimum_cmce_threshold: 1.0,
        }
    }

    pub fn authorize_payload(
        &self,
        proposed_coord: &Cl31Multivector,
        systemic_payoff: f64,
    ) -> Result<f64, &'static str> {
        let lambda_friction = calculate_lambda_cost(proposed_coord);
        let absolute_friction = if lambda_friction <= 0.0 { 1e-9 } else { lambda_friction };
        let c_mce = systemic_payoff / absolute_friction;

        if c_mce >= self.minimum_cmce_threshold {
            Ok(lambda_friction)
        } else {
            Err("MCE_VIOLATION: Transition aborted. C_MCE drops below 1.0; computational friction exceeds systemic value.")
        }
    }
}
