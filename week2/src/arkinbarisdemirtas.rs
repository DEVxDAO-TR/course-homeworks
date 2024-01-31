mod week2_tests {


    #[derive(Debug, PartialEq)]
    enum LogLevel {
        Msg,
        Warn,
        Err,
    }

    fn log(level: LogLevel, message: &str) -> String {
        match level {
            LogLevel::Msg => format!("[MSG]: {}", message),
            LogLevel::Warn => format!("[WARN]: {}", message),
            LogLevel::Err => format!("[ERR]: {}", message),
        }
    }

    #[derive(Debug, PartialEq)]
    enum Gender {
        Male,
        Female,
    }

    struct Person {
        name: String,
        age: u32,
        gender: Gender,
    }

    impl ToString for Person {
        fn to_string(&self) -> String {
            let gender_str = match self.gender {
                Gender::Male => "Erkek",
                Gender::Female => "Kadın",
            };
            format!("{}, {}, {}", self.name, self.age, gender_str)
        }
    }

    fn to_letter_grade(score: u32) -> String {
        match score {
            90..=100 => String::from("AA"),
            85..=89 => String::from("BA"),
            80..=84 => String::from("BB"),
            75..=79 => String::from("CB"),
            70..=74 => String::from("CC"),
            60..=69 => String::from("DC"),
            50..=59 => String::from("DD"),
            40..=49 => String::from("FD"),
            0..=39 => String::from("FF"),
            _ => String::from("Invalid"),
        }
    }

    #[test]
    fn to_letter_grade_test() {
        assert_eq!(to_letter_grade(100), String::from("AA"));
        assert_eq!(to_letter_grade(90), String::from("AA"));
        assert_eq!(to_letter_grade(88), String::from("BA"));
        assert_eq!(to_letter_grade(80), String::from("BB"));
        assert_eq!(to_letter_grade(75), String::from("CB"));
        assert_eq!(to_letter_grade(70), String::from("CC"));
        assert_eq!(to_letter_grade(60), String::from("DC"));
        assert_eq!(to_letter_grade(55), String::from("DD"));
        assert_eq!(to_letter_grade(45), String::from("FD"));
        assert_eq!(to_letter_grade(38), String::from("FF"));
        assert_eq!(to_letter_grade(0), String::from("FF"));
    }

    #[test]
    fn log_test() {
        assert_eq!(log(LogLevel::Msg, "mesaj"), String::from("[MSG]: mesaj"));
        assert_eq!(log(LogLevel::Warn, "uyari"), String::from("[WARN]: uyari"));
        assert_eq!(log(LogLevel::Err, "hata"), String::from("[ERR]: hata"));
    }

    #[test]
    fn person_display_test() {
        let p1 = Person {
            name: String::from("Ahmet"),
            age: 24,
            gender: Gender::Male,
        };
        assert_eq!(p1.to_string(), String::from("Ahmet, 24, Erkek"));

        let p2 = Person {
            name: String::from("Ayşe"),
            age: 18,
            gender: Gender::Female,
        };
        assert_eq!(p2.to_string(), String::from("Ayşe, 18, Kadın"));
    }
}
