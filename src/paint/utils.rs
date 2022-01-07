pub fn frac_part(x: f64) -> f64 {
    x - x.floor()
}

pub fn rf_part(x: f64) -> f64 {
    1.0 - frac_part(x)
}

pub fn rnd(x: f64) -> f64 {
    (x + 0.5).round()
}