use std::fmt::format;

automod::dir!("src/");

    pub fn to_letter_grade(num:u32) -> String{
            match num {
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

    enum LogLevel {
       Msg,
       Warn,
       Err
    }

    impl Display for LogLevel {
        fn fmt(&self, f: &mut std:fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                LogLevel::Msg => write!(f, "MSG"),
                LogLevel::Warn => write!(f, "WARN"),
                LogLevel::Err => write!(f, "ERR")
            }
        }
    }


    pub fn log(level: LogLevel, msg: &str) -> String {
        format!("{}, {}", level, msg)
    }
  

    enum Gender {
        Male,
        Female,
    }

    struct Person { 
        name: String,
        age: u32,
        gender: Gender,
    }

    impl Gender {   
        fn to_string(&self) -> String {
            match self {
                Gender::Male => String::from("Male"),
                Gender::Female => String::from("Female"),
            }
        }
    }

    impl Person {   
        fn new(name: String, age: u32, gender: Gender) -> Person {
            Person {
                name,
                age,
                gender,
            }
        }

        fn to_string(&self) -> String {     
            format!("{}, {}, {}", self.name, self.age.to_string(), self.gender.to_string())
        }
    }



// Test Templates
#[cfg(test)]
mod week2_tests {
        use super::*;

        #[test]
        fn to_letter_grade_test() {
            assert_eq!(to_letter_grade(95), String::from("AA"));
            assert_eq!(to_letter_grade(85), String::from("BA"));
            assert_eq!(to_letter_grade(80), String::from("BB"));
            assert_eq!(to_letter_grade(75), String::from("CB"));
            assert_eq!(to_letter_grade(150), String::from("Invalid"));
        }

        #[test]
        fn log_test() {
            assert_eq!(log(LogLevel::Msg, "mesaj"), String::from("[MSG]: mesaj"));
            assert_eq!(log(LogLevel::Warn, "uyari"), String::from("[WARN]: uyari"));
            assert_eq!(log(LogLevel::Err, "hata"), String::from("[ERR]: hata"));
        }

        fn person_display_test(){
            let p1 = Person::new(String::from("John"), 20, Gender::Male);
            let p2: Person = Person::new(String::from("Mary"), 30, Gender::Female);
            assert_eq!(p1.to_string(), String::from("John, 20, Male"));
            assert_eq!(p2.to_string(), String::from("Mary, 30, Female"));
        }
} 
