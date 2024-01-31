// Homework - 1
use std::fmt::{Display, Formatter, Result};

// Homework - 1
pub fn to_letter_grade(num: u8) -> String {
    match num {
        90..=100 => "AA".to_string(),
        85..=89 => "BA".to_string(),
        80..=84 => "BB".to_string(),
        75..=79 => "CB".to_string(),
        70..=74 => "CC".to_string(),
        60..=69 => "DC".to_string(),
        50..=59 => "DD".to_string(),
        40..=49 => "FD".to_string(),
        0..=39 => "FF".to_string(),
        _ => "Error".to_string(),
    }
}

// Homework - 2
pub enum LogLevel {
    Msg,
    Warn,
    Err,
}

pub fn log(level: LogLevel, msg: &str) -> String {
    match level {
        LogLevel::Msg => format!("[MSG]: {}", msg),
        LogLevel::Warn => format!("[WARN]: {}", msg),
        LogLevel::Err => format!("[ERR]: {}", msg),
    }
}

// Homework - 3
pub enum Gender {
    Male,
    Female,
}

pub struct Person {
    pub name: String,
    pub age: u8,
    pub gender: Gender,
}

impl Display for Gender {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Gender::Male => write!(f, "Erkek"),
            Gender::Female => write!(f, "Kadın"),
        }
    }
}

impl Display for Person {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}, {}, {}", self.name, self.age, self.gender)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_letter_grade_test() {
        assert_eq!(to_letter_grade(100), "AA");
        assert_eq!(to_letter_grade(90), "AA");
        assert_eq!(to_letter_grade(88), "BA");
        assert_eq!(to_letter_grade(80), "BB");
        assert_eq!(to_letter_grade(75), "CB");
        assert_eq!(to_letter_grade(70), "CC");
        assert_eq!(to_letter_grade(60), "DC");
        assert_eq!(to_letter_grade(55), "DD");
        assert_eq!(to_letter_grade(45), "FD");
        assert_eq!(to_letter_grade(38), "FF");
        assert_eq!(to_letter_grade(0), "FF");
    }

    #[test]
    fn log_test() {
        assert_eq!(log(LogLevel::Msg, "mesaj"), "[MSG]: mesaj");
        assert_eq!(log(LogLevel::Warn, "uyari"), "[WARN]: uyari");
        assert_eq!(log(LogLevel::Err, "hata"), "[ERR]: hata");
    }

    #[test]
    fn person_display_test() {
        let p1 = Person {
            name: String::from("Ahmet"),
            age: 24,
            gender: Gender::Male,
        };
        assert_eq!(p1.to_string(), "Ahmet, 24, Erkek");

        let p2 = Person {
            name: String::from("Ayşe"),
            age: 18,
            gender: Gender::Female,
        };
        assert_eq!(p2.to_string(), "Ayşe, 18, Kadın");
    }
}

  
