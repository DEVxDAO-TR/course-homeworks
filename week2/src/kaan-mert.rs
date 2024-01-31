pub fn to_letter_grade(grade: i32) -> &'static str {
    match grade {
        90..=100 => "AA",
        85..=89 => "BA",
        80..=84 => "BB",
        75..=79 => "CB",
        70..=74 => "CC",
        60..=69 => "DC",
        50..=59 => "DD",
        40..=49 => "FD",
        0..=39 => "FF",
        _ => "Invalid Grade"
    }
}

pub enum LogLevel {
    Msg,
    Warn,
    Err
}


pub fn log(log: LogLevel,text:&str ) -> String {
    match log {
        LogLevel::Msg   => format!("[MSG] : {}",text),
        LogLevel::Warn  => format!("[WARN]: {}",text),
        LogLevel::Err   => format!("[ERR] : {}",text)
    }
}

pub enum Gender {
    Female,
    Male
}

pub struct Person {
    name:String,
    age :i32,
    gender:Gender
}

impl std::fmt::Display for Gender {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       match self {
           Gender::Female => write!(f,"Kadın"),
           Gender::Male   => write!(f,"Erkek") 
       }
    }
}

impl std::fmt::Display for Person {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}, {}", self.name, self.age, self.gender)
    }
}


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
    assert_eq!(to_letter_grade(90),  String::from("AA"));
    assert_eq!(to_letter_grade(88),  String::from("BA"));
    assert_eq!(to_letter_grade(80),  String::from("BB"));
    assert_eq!(to_letter_grade(75),  String::from("CB"));
    assert_eq!(to_letter_grade(70),  String::from("CC"));
    assert_eq!(to_letter_grade(60),  String::from("DC"));
    assert_eq!(to_letter_grade(55),  String::from("DD"));
    assert_eq!(to_letter_grade(45),  String::from("FD"));
    assert_eq!(to_letter_grade(38),  String::from("FF"));
    assert_eq!(to_letter_grade(0),   String::from("FF"));
}

#[test]
fn log_test() {
    assert_eq!(log(LogLevel::Msg, "mesaj"), "[MSG] : mesaj");
    assert_eq!(log(LogLevel::Warn, "uyari"), "[WARN]: uyari");
    assert_eq!(log(LogLevel::Err, "hata"), "[ERR] : hata");
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
