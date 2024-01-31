automod::dir!(pub "src/");

// Test Templates
#[cfg(test)]
#[cfg(target_os = "windows")] // burayı kendi kodunuzda silin
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
        assert_eq!(p2.to_string(), String::from("Ayşe, 18, Kadın"));
    }
}