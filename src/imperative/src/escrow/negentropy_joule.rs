// src/escrow/negentropy_joule.rs

/// Represents a standardized unit of computational/thermodynamic work.
#[derive(Debug, Clone, Copy)]
pub struct NegentropyJoule {
    pub absolute_value: f64,
}

impl NegentropyJoule {
    /// Mints a new NJ based on geometric friction, hardware efficiency, and Phase Lock.
    pub fn mint(
        lambda_friction: f64,
        substrate_efficiency: f64,
        execution_time_ms: f64,
        is_phase_locked: bool,
    ) -> Result<Self, &'static str> {
        
        // 1. The Phase Lock Gate: No resonance, no settlement.
        if !is_phase_locked {
            return Err("MINT_DENIED: Node has lost Phase Lock resonance. Thermodynamic work is invalid.");
        }

        // 2. Prevent division by zero and enforce a minimal execution floor
        let safe_execution_time = if execution_time_ms <= 0.0 { 1.0 } else { execution_time_ms };

        // 3. The Target Baseline: What an optimal node *should* take to clear 1.0 Lambda
        const TARGET_MS_PER_LAMBDA: f64 = 15.0; 
        
        // 4. Calculate the temporal drift (how far off the node is from the ideal baseline)
        let temporal_variance = (TARGET_MS_PER_LAMBDA * lambda_friction) / safe_execution_time;

        // 5. Derive the final NJ value
        // If the node is slow, temporal_variance drops, reducing the NJ yield.
        let final_nj_value = lambda_friction * substrate_efficiency * temporal_variance;

        // 6. MCE Dust Filter: Prevent the minting of microscopic, network-clogging decimals
        if final_nj_value < 0.001 {
            return Err("MINT_DENIED: NJ yield falls below the minimum viable threshold (Dust).");
        }

        Ok(NegentropyJoule {
            absolute_value: final_nj_value,
        })
    }
}
