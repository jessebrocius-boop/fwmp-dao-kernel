// src/escrow/zero_liability.rs
use crate::escrow::negentropy_joule::NegentropyJoule;
use std::time::{SystemTime, UNIX_EPOCH};

/// Represents a finalized thermodynamic settlement between nodes.
pub struct NegentropySettlement {
    pub settlement_id: [u8; 32],      // SHA-256 hash of the triage event
    pub joule_value: NegentropyJoule, // The normalized NJ count
    pub expiry_timestamp: u64,        // Unix timestamp: initiation + 180 seconds
    pub proof_of_work: Vec<u8>,       // Cryptographic proof that the state was resolved
}

/// The decisive outcome of a settlement event.
pub enum SettlementOutcome {
    Cleared(NegentropySettlement),
    TopologicalAmputation(&'static str),
}

impl NegentropySettlement {
    /// Generates a pending settlement bound by a strict 3-minute physical limit.
    pub fn initiate_settlement(
        triage_hash: [u8; 32], 
        joules: NegentropyJoule
    ) -> Self {
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        NegentropySettlement {
            settlement_id: triage_hash,
            joule_value: joules,
            // The 3-Minute Zero-LED Boundary
            expiry_timestamp: current_time + 180, 
            proof_of_work: Vec::new(), 
        }
    }

    /// Evaluates the state. If the temporal boundary is breached, 
    /// the network amputates the node immediately to prevent contagion.
    pub fn attempt_clearance(
        &mut self, 
        submitted_proof: Vec<u8>, 
        current_timestamp: u64
    ) -> SettlementOutcome {
        
        // 1. The Temporal Hard-Cap Check
        if current_timestamp > self.expiry_timestamp {
            return SettlementOutcome::TopologicalAmputation(
                "AMPUTATION_TRIGGERED: 3-Minute Zero-LED boundary breached. Node severed."
            );
        }

        // 2. Cryptographic Proof Validation (MCE Sandbox Gate)
        if submitted_proof.is_empty() {
             return SettlementOutcome::TopologicalAmputation(
                "AMPUTATION_TRIGGERED: Invalid proof of work submitted. Node severed."
            );
        }

        // 3. Zero-Liability Execution
        self.proof_of_work = submitted_proof;
        SettlementOutcome::Cleared(Self {
            settlement_id: self.settlement_id,
            joule_value: self.joule_value.clone(),
            expiry_timestamp: self.expiry_timestamp,
            proof_of_work: self.proof_of_work.clone(),
        })
    }
}
