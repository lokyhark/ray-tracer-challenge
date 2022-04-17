const EPSILON: f64 = 1.0e-5;

pub fn float_eq(left: f64, right: f64) -> bool {
    (left - right).abs() <= EPSILON
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn float_eq_pass() {
        assert!(float_eq(1.0_f64, 1.000_001));
    }

    #[test]
    fn float_eq_fail() {
        assert!(!float_eq(1.0_f64, 1.000_02));
    }
}
