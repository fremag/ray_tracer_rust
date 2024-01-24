#[cfg(test)]
mod tests {
    use crate::math::equals;

    #[test]
    pub fn equals_test() {
        assert_eq!(equals(1.0, 2.0), false);
        assert_eq!(equals(1.0, 1.0), true);
        assert_eq!(equals(1.0, 1.00001), true);
        assert_eq!(equals(1.0, 1.0002), false);
    }
}