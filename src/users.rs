// Define the User struct
pub struct User {
    pub name: String,
    pub age: i32,
}

// Implement methods for the User struct
impl User {
    // Constructor method for creating a new User
    pub fn new(name: String, age: i32) -> Self {
        User { name, age }
    }
}
