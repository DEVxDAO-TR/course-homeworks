use std::fmt::{self, Display};

fn to_letter_grade(grade: u32) -> String {
    match grade {
        90..=100 => String::from("AA"),
        85..=89 => String::from("BA"),
        80..=84 => String::from("BB"),
        75..=79 => String::from("CB"),
        70..=74 => String::from("CC"),
        60..=69 => String::from("DC"),
        50..=59 => String::from("DD"),
        40..=49 => String::from("FD"),
        0..=39 => String::from("FF"),
        _ => String::from("Invalid grade"),
    }
}

enum LogLevel {
    Msg,
    Warn,
    Err,
}

impl Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]", self)
    }
}

fn log(loglevel: LogLevel, msg: &str) -> String {
    match loglevel {
        LogLevel::Err => format!("[ERR]: {}", msg),
        LogLevel::Msg => format!("[MSG]: {}", msg),
        LogLevel::Warn => format!("[WARN]: {}", msg),
    }
}

struct Person {
    name: String,
    age: u8,
    gender: Gender,
}

enum Gender {
    Male,
    Female,
}

impl Display for Gender {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Gender::Male => write!(f, "Erkek"),
            Gender::Female => write!(f, "Kadin"),
        }
    }
}

impl Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}, {}", self.name, self.age, self.gender)
    }
}

#[cfg(test)]
mod week2_tests {
    use super::*;

    #[test]
    fn to_letter_grade_test() {
        // AA = 90-100
        // BA = 85-89
        // BB = 80-84
        // CB = 75-79
        // CC = 70-74
        // DC = 60-69
        // DD = 50-59
        // FD = 40-49
        // FF = 0-39
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
        assert_eq!(p2.to_string(), String::from("Ayşe, 18, Kadin"));
    }
}
