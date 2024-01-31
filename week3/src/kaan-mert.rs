use serde::{Deserialize,Serialize};
use serde_json;

#[derive(Serialize, Deserialize)]
pub enum Gender {
    Male,
    Female,
}

#[derive(Serialize, Deserialize)]
pub struct Person {
    pub name: String,
    pub age: u8,
    pub gender: Gender,
}

#[cfg(test)]
mod week3_tests {
    use crate::kaan_mert::Person;

    #[test]
    fn person_to_json() {
        let person1:Person = super::Person {
            name: String::from("Emin"),
            age: 18,
            gender: super::Gender::Male,
        };
        let person2:Person = super::Person {
            name: String::from("Ayşe"),
            age: 25,
            gender: super::Gender::Female,
        };

        let p1_json_str:String = serde_json::to_string(&person1).unwrap();
        let p2_json_str:String = serde_json::to_string(&person2).unwrap();

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