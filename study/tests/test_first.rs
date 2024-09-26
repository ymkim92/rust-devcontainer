use study::math;

// Test module
#[cfg(test)]
mod tests {
    use super::*;
    
    // Define a test
    #[test]
    fn test_add() {
        assert_eq!(math::add(2, 3), 5);
    }

    #[test]
    fn test_add_negative() {
        assert_eq!(math::add(-2, 3), 1);
    }
}
