// A simple function that adds two numbers
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

// Test module
#[cfg(test)]
mod tests {
    // Import all functions from the outer module
    use super::*;

    // Define a test
    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn test_add_negative() {
        assert_eq!(add(-2, 3), 1);
    }
}
