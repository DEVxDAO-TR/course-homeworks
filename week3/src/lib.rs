automod::dir!(pub "src/");

#[cfg(test)]
#[cfg(target_os = "windows")] // burayı kendi kodunuzda silin
mod week3_tests {
    use week3;

    #[test]
    fn person_to_json() {
        let p1 = week3::Person {
            name: String::from("Emin"),
            age: 18,
            gender: week3::Gender::Male,
        };
        let p2 = week3::Person {
            name: String::from("Ayşe"),
            age: 25,
            gender: week3::Gender::Female,
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
