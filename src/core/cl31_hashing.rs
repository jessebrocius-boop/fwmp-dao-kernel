pub struct Cl31Multivector {
    pub scalar: f64,
    pub vector: [f64; 4],
    pub bivector: [f64; 6],
    pub trivector: [f64; 4],
    pub pseudoscalar: f64,
}

impl Cl31Multivector {
    pub fn verify_geometric_parity(&self) -> bool {
        let parity_sum = self.scalar + self.pseudoscalar;
        parity_sum.is_finite() && parity_sum.abs() < 1e-9
    }
}
