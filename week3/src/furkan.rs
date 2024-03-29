use serde::{Deserialize, Serialize};

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
    use crate::furkan::Person;
    use crate::furkan::Gender;
    
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
