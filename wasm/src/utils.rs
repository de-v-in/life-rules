use rand::Rng;

/// Return random number in range.
pub fn random_range(min: f64, max: f64) -> f64 {
    let seed: f64 = rand::thread_rng().gen();
    min + seed * (max - min)
}

/// Convert a string into f64 number
pub fn str_to_f64(str: &str) -> f64 {
    str::parse::<f64>(str).unwrap_or(0f64)
}
