// src/governance/kill_switch.rs

use crate::escrow::zero_liability::SettlementOutcome;
use crate::core::cl31_hashing::Cl31Multivector;
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct Executioner {
    pub active_topology: HashMap<[u8; 32], f64>, 
}

impl Executioner {
    pub fn enforce_settlement(
        &mut self,
        target_node_id: [u8; 32],
        outcome: SettlementOutcome,
        final_state: &Cl31Multivector,
    ) -> Result<&'static str, FieldDiagnosticCaseFile> {
        match outcome {
            SettlementOutcome::Cleared(_) => {
                Ok("DARK_PATH: Liability resolved. Topological weights secured.")
            }
            SettlementOutcome::TopologicalAmputation(reason) => {
                self.sever_connection(target_node_id);
                
                let autopsy_record = FieldDiagnosticCaseFile::generate_autopsy(
                    target_node_id,
                    reason,
                    final_state,
                );
                
                Err(autopsy_record)
            }
        }
    }

    fn sever_connection(&mut self, node_id: [u8; 32]) {
        self.active_topology.remove(&node_id);
    }
}

pub struct FieldDiagnosticCaseFile {
    pub node_id: [u8; 32],
    pub breach_reason: &'static str,
    pub terminal_multivector: Cl31Multivector,
    pub timestamp: u64,
}

impl FieldDiagnosticCaseFile {
    pub fn generate_autopsy(
        node_id: [u8; 32],
        reason: &'static str,
        terminal_state: &Cl31Multivector,
    ) -> Self {
        Self {
            node_id,
            breach_reason: reason,
            terminal_multivector: terminal_state.clone(),
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }
}
