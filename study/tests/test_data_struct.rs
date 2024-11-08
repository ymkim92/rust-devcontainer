use std::collections::HashMap;

// A struct representing a simple user database
pub struct UserDatabase {
    users: HashMap<String, User>,
}

// A struct representing a user
pub struct User {
    m_age: u32,
}

impl UserDatabase {
    // Create a new, empty UserDatabase
    pub fn new() -> Self {
        UserDatabase { users: HashMap::new() }
    }

    // Add a user to the database
    pub fn add_user(&mut self, name: String, age: u32) -> Result<(), String> {
        if self.users.contains_key(&name) {
            Err(format!("User {} already exists", name))
        } else {
            self.users.insert(name.clone(), User { m_age: age });
            Ok(())
        }
    }

    // Get a user's age, if the user exists
    pub fn get_user_age(&self, name: &str) -> Option<u32> {
        self.users.get(name).map(|user| user.m_age)
    }
}

#[cfg(test)]
mod db_tests {
    use super::*;

    // This function will be run before each test
    fn setup() -> UserDatabase {
        UserDatabase::new()
    }

    #[test]
    fn test_add_user() {
        let mut db = setup();
        assert!(db.add_user("Alice".to_string(), 30).is_ok());
        assert!(db.add_user("Alice".to_string(), 30).is_err());
        let result = db.add_user("Alice".to_string(), 25);
        assert!(result.is_err());

        match result {
            Err(error_string) => {
                assert_eq!(error_string, "User Alice already exists");
            }
            Ok(_) => panic!("Expected an error, but got Ok"),
        }
    }

    #[test]
    fn test_add_duplicate_user() {
        let mut db = setup();
        assert!(db.add_user("Bob".to_string(), 25).is_ok());
        assert!(db.add_user("Bob".to_string(), 30).is_err());
    }

    #[test]
    fn test_get_nonexistent_user() {
        let db = setup();
        assert_eq!(db.get_user_age("Charlie"), None);
    }

    #[test]
    fn test_multiple_users() {
        let mut db = setup();
        assert!(db.add_user("Eve".to_string(), 28).is_ok());
        assert!(db.add_user("Frank".to_string(), 35).is_ok());
        assert_eq!(db.get_user_age("Eve"), Some(28));
        assert_eq!(db.get_user_age("Frank"), Some(35));
    }
}
