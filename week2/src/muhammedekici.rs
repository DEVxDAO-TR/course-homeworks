pub fn to_letter_grade(num:u8) -> String {
    match num {
        90..=100=> "AA".to_string(),
        85..=89=> "BA".to_string(),
        80..=84=> "BB".to_string(),
        75..=79=> "CB".to_string(),
        70..=74=> "CC".to_string(),
        60..=69=> "DC".to_string(),
        50..=59=> "DD".to_string(),
        40..=49=> "FD".to_string(),
        0..=39=> "FF".to_string(),
        _ => "FF".to_string()
    }
}

enum LogLevel {
    Msg,
    Debug,
    Info,
    Warn,
    Err    
}

fn log(level:LogLevel, msg: &str) {
    match level {
        LogLevel::Msg => println!("[MSG]: {}",msg),
        LogLevel::Debug => println!("[DEBUG]: {}",msg),
        LogLevel::Warn=> println!("[WARN]: {}",msg),
        LogLevel::Info => println!("[INFO]: {}",msg),
        LogLevel::Err => println!("[ERR]: {}",msg),
    }

}
enum Gender{
    Male,
    Female
}

struct Person{
    name: String,
    age: u32,
    gender:Gender
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
        assert_eq!(p2.to_string(), String::from("Ayşe, 18, Kadın"));
    }
}