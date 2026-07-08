use crate::core::cl31_hashing::Cl31Multivector;

pub fn calculate_lambda_cost(coord: &Cl31Multivector) -> f64 {
    const W_SCALAR: f64 = 1.0;
    const W_VECTOR: f64 = 2.5;
    const W_BIVECTOR: f64 = 4.0;
    const W_TRIVECTOR: f64 = 6.5;
    const W_PSEUDOSCALAR: f64 = 1.5;

    let friction = (coord.scalar.abs() * W_SCALAR) +
                   (coord.vector.iter().map(|v| v.abs()).sum::<f64>() * W_VECTOR) +
                   (coord.bivector.iter().map(|v| v.abs()).sum::<f64>() * W_BIVECTOR) +
                   (coord.trivector.iter().map(|v| v.abs()).sum::<f64>() * W_TRIVECTOR) +
                   (coord.pseudoscalar.abs() * W_PSEUDOSCALAR);

    friction
}
