use serde::{Serialize, Deserialize};

// Write Your Code Here
pub enum Gender {
    Male,
    Female
}

// Write Your Code Here
pub struct Person {
    pub name:String,
    pub age:u8,
    pub gender:Gender
}