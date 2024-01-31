use serde::{Serialize, Deserialize};
use std::fmt::{self, Display};

#[derive(Serialize, Deserialize)]
pub enum Gender{
    Male,
    Female
}

#[derive(Serialize, Deserialize)]
struct Person{
    name: String,
    age: u32,
    gender:Gender
} 

impl Display for Person{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.gender {
            Gender::Female => write!(f,"Female"),
            Gender::Male => write!(f,"Male")
        }
    }
}

#[cfg(test)]
mod week3_tests {
    use super::*;

    #[test]
    fn person_to_json() {
        let p1 = super::Person {
            name: String::from("Emin"),
            age: 18,
            gender: super::Gender::Male,
        };
        let p2 = super::Person {
            name: String::from("Ayşe"),
            age: 25,
            gender: super::Gender::Female,
        };

        let p1_json_str = serde_json::to_string(&p1).unwrap();
        let p2_json_str = serde_json::to_string(&p2).unwrap();

        assert_eq!(
            p1_json_str,
            String::from("{\"name\":\"Emin\",\"age\":18,\"gender\":\"Male\"}")
        );
        assert_eq!(
            p2_json_str,
            String::from(r#"{"name":"Ayşe","age":25,"gender":"Female"}"#)
        );
    }
}