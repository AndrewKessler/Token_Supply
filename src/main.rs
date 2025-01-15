fn chebyshev_eval(coeffs: &[f32], num_coeffs: usize, x: f32, x_min: f32, x_max: f32) -> f32 {
    let x_rel_2 = -2.0 + 4.0 * (x - x_min) / (x_max - x_min);
    let mut d = 0.0;
    let mut dd = 0.0;
    let mut temp = 0.0;
    for i in (1..num_coeffs).rev() {
        temp = d;
        d = x_rel_2 * d - dd + coeffs[i];
        dd = temp;
    }
    0.5 * x_rel_2 * d - dd + 0.5 * coeffs[0]
}

const NUM_COEFFS: usize = 8;
const COEFFS: [f32; NUM_COEFFS] = [
    25.03174918802757,
    1.8258010702529597,
    -0.8231072598238414,
    0.4851246043711708,
    -0.31159662100481755,
    0.202116912222996,
    -0.12273401582298715,
    0.058225096947450095,
];
const X_MIN: f32 = 1.0;
const X_MAX: f32 = 1000000.0;

fn main() {
    let x_mid = 0.5 * (X_MIN + X_MAX);
    //let x_mid = 100000.0;
    let value_at_x_mid = chebyshev_eval(&COEFFS, NUM_COEFFS, x_mid, X_MIN, X_MAX);
    println!("Approximated value at x={} is {} (single precision)", x_mid, value_at_x_mid);
    println!("Should be 13.150119248655797 (double precision)");
}

