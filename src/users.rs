impl User {
    pub fn new(name: String, age: u32) -> Self {
        User { name, age }
    }
}

pub struct User{
    pub name:String,
    pub age:u32
}