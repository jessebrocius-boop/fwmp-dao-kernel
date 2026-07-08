// src/engine/consumer.rs
use crate::interfaces::aura_telemetry::IonicFlux;
use crate::interfaces::dispatcher_ao::DispatcherRingBuffer;
use crate::interfaces::crypto::ZkpAssertion;

/// The Volatile Processing Engine.
/// Operates strictly in high-speed, lock-free memory to drain the ephemeral commons
/// and emit cryptographic truth without leaking telemetry data.
pub struct ConsumerEngine {
    /// The current cryptographically verified state tip.
    pub current_chain_tip: [u8; 32],
    /// Epoch sequence for continuous network alignment.
    pub current_epoch: u64,
}

impl ConsumerEngine {
    /// Initializes the engine with a network-verified genesis state.
    pub fn new(genesis_tip: [u8; 32], genesis_epoch: u64) -> Self {
        Self {
            current_chain_tip: genesis_tip,
            current_epoch: genesis_epoch,
        }
    }

    /// Executes a single non-blocking atomic cycle:
    /// 1. Poll the lock-free buffer.
    /// 2. Verify physical/cryptographic integrity.
    /// 3. Emit ZKP Assertion.
    pub fn execute_cycle(
        &mut self, 
        ring_buffer: &mut DispatcherRingBuffer
    ) -> Option<ZkpAssertion> {
        
        // 1. Poll the Ring Buffer (O(1) execution bounds)
        let payload = match ring_buffer.pop() {
            Some(flux) => flux,
            None => return None, // Buffer empty, yield cycle to preserve MCE
        };

        // 2. Hardware Gate: Validate mesh impedance and dual-node signatures
        if !payload.is_valid() {
            // In a live Dispatcher environment, this would route to the kill switch
            // to initiate Topological Amputation. Here, we drop the corrupted flux.
            return None; 
        }

        // 3. ZKP Generation: Emit truth, blind the data.
        // The raw [f64; 16] matrix is passed directly to the firewall logic.
        match ZkpAssertion::generate(&payload.matrix_data, self.current_chain_tip, self.current_epoch) {
            Ok(assertion) => {
                // Update the state tip for the next cycle, solidifying the temporal chain
                self.current_chain_tip = assertion.chain_tip;
                Some(assertion)
            },
            Err(_) => {
                // MCE constraint failed during proof generation
                None
            }
        }
    }
}
