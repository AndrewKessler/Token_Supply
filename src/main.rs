fn ln_chebyshev_eval(coeffs: &[f32], num_coeffs: usize, x: f32, x_min: f32, x_max: f32) -> f32 {
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

const NUM_COEFFS: usize = 9;
const COEFFS: [f32; NUM_COEFFS] = [
    25.012499098559037,
    1.8453148886962574,
    -0.843432580749474,
    0.5068733325071559,
    -0.3354990572347687,
    0.2290967251732503,
    -0.15402070771901166,
    0.09553417086511973,
    -0.045839403109401544,
];
const X_MIN: f32 = 1.0;
const X_MAX: f32 = 1000000.0;

/// Simple Exponential Moving Average (EMA) filter in Rust
/// 
/// Calculates the EMA for a series of time-series data.

struct EMA {
    smoothing_factor: f64,
    last_ema: Option<f64>,
}

impl EMA {
    /// Creates a new EMA filter with a given smoothing factor (alpha).
    /// `alpha` should be a value between 0 and 1.
    fn new(alpha: f64) -> Self {
        assert!((0.0..=1.0).contains(&alpha), "Alpha must be between 0 and 1.");
        EMA {
            smoothing_factor: alpha,
            last_ema: None,
        }
    }

    /// Updates the EMA with a new data point and returns the updated value.
    fn update(&mut self, value: f64) -> f64 {
        let new_ema = match self.last_ema {
            Some(last) => self.smoothing_factor * value + (1.0 - self.smoothing_factor) * last,
            None => value, // If no previous EMA, initialize with the first value
        };

        self.last_ema = Some(new_ema);
        new_ema
    }
}

fn main() {
    let x_mid = 0.5 * (X_MIN + X_MAX);
    //let x_mid = 100000.0;
    let value_at_x_mid = ln_chebyshev_eval(&COEFFS, NUM_COEFFS, x_mid, X_MIN, X_MAX);
    println!("Approximated value at x={} is {} (single precision)", x_mid, value_at_x_mid);
    println!("Should be 13.122364377403834 (double precision)");
    println!("Non approximated value is 13.122364377403828795024049689982");

    let data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0]; // Example time-series data
    let alpha = 0.1; // Smoothing factor

    let mut ema_filter = EMA::new(alpha);

    println!("Time-series data: {:?}", data);
    println!("EMA with alpha = {}:", alpha);

    for (i, value) in data.iter().enumerate() {
        let ema = ema_filter.update(*value);
        println!("Step {}: Value = {:.2}, EMA = {:.2}", i + 1, value, ema);
    }
}

