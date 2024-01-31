use std::fmt::{self, Display};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
    gender: Gender,
}

#[derive(Serialize, Deserialize)]
enum Gender {
    Male,
    Female,
}

impl Display for Gender {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Gender::Male => write!(f, "Male"),
            Gender::Female => write!(f, "Female"),
        }
    }
}
// fn person_to_json() -> String {}

#[cfg(test)]
mod week3_tests {
    use super::*;

    #[test]
    fn person_to_json() {
        let p1 = Person {
            name: String::from("Emin"),
            age: 18,
            gender: Gender::Male,
        };
        let p2 = Person {
            name: String::from("Ayşe"),
            age: 25,
            gender: Gender::Female,
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
