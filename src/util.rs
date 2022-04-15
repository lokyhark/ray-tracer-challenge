use crate::Num;

pub fn float_eq<Float: Num>(left: Float, right: Float) -> bool {
    (left - right).abs() <= Float::EPSILON
}
