// Write Your Code Here
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum Gender {
    Male,
    Female,
}

// Write Your Code Here
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Person {
    pub name: String,
    pub age: u8,
    pub gender: Gender,
}

#[cfg(test)]
mod week3_tests {
    use super::*;
    use serde_json;

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

        let p1_json_str: String = serde_json::to_string(&p1).unwrap();
        let p2_json_str = serde_json::to_string(&p2).unwrap();

        assert_eq!(
            p1_json_str,
            String::from("{\"name\":\"Emin\",\"age\":18,\"gender\":\"Male\"}")
        );
        assert_eq!(
            p2_json_str,
            String::from("{\"name\":\"Ayşe\",\"age\":25,\"gender\":\"Female\"}")
        );

        // Deserialize back to Person and check equality
        let p1_deserialized: Person = serde_json::from_str(&p1_json_str).unwrap();
        let p2_deserialized: Person = serde_json::from_str(&p2_json_str).unwrap();

        assert_eq!(p1, p1_deserialized);
        assert_eq!(p2, p2_deserialized);
    }
}
