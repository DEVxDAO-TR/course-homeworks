use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum Gender {
    Male,
    Female,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Person {
    pub name: String,
    pub age: u8,
    pub gender: Gender,
}

#[cfg(test)]

mod week3_tests {

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
