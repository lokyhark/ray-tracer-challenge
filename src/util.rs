use crate::Num;

pub fn float_eq<Float: Num>(left: Float, right: Float) -> bool {
    (left - right).abs() <= Float::EPSILON
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn float_eq_pass() {
        assert!(float_eq(1.0_f32, 1.000_001));
        assert!(float_eq(1.0_f64, 1.000_001));
    }

    #[test]
    fn float_eq_fail() {
        assert!(!float_eq(1.0_f32, 1.000_02));
        assert!(!float_eq(1.0_f64, 1.000_02));
    }
}
