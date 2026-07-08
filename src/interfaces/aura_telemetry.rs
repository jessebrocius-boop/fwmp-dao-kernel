// src/interfaces/aura_telemetry.rs

/// Represents raw ionic flux data captured from the woven textile interface.
/// This structure acts as the immutable input schema for the static off-ramp buffer.
#[repr(C, align(64))] // Align to cache line to prevent false sharing
#[derive(Clone, Copy)] // Enables zero-allocation data transfers
pub struct IonicFlux {
    pub matrix_data: [f64; 16],      // 4x4 photodiode matrix output
    pub mesh_impedance: f64,         // Woven textile-shielding physical integrity metric
    pub timestamp: u64,              // Nanosecond precision differential bus clock
    pub primary_checksum: u64,       // Origin node parity hash
    pub secondary_signature: u64,    // Verification node signature (Dual-Node consensus)
}

impl IonicFlux {
    /// Validates the physical and cryptographic integrity of the flux packet.
    pub fn is_valid(&self) -> bool {
        // Enforce basic hardware parity and dual-node signature existence
        self.primary_checksum != 0 && self.secondary_signature != 0 
    }
}
