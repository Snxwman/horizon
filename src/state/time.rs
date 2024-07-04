use std::fs::File;
use std::sync::RwLock;

use once_cell::sync::Lazy;

use chrono::prelude::*;

use crate::widgets::UsesHorizonData;

pub static DATETIME: Lazy<RwLock<LocalDateTime>> = Lazy::new(|| RwLock::new(LocalDateTime::new()));

#[derive(Debug)]
pub struct LocalDateTime(pub DateTime<Local>);

impl LocalDateTime {
    pub fn new() -> Self {
        Self(chrono::Local::now())
    }

    pub fn update(&mut self) {
        self.0 = chrono::Local::now();
    }
}

struct Month {
    name: String,
    short_name: String,
    number: u8,
}

enum Months {
    January(Month),
    February(Month),
    March(Month),
    April(Month),
    May(Month),
    June(Month),
    July(Month),
    August(Month),
    September(Month),
    October(Month),
    November(Month),
    December(Month),
}

enum IntoMonthsError {
    NotAMonthName,
    NotAMonthNumber,
}

impl Month {
    fn new(name: &str, number: u8) -> Self {
        let name = name.to_owned();
        let short_name = match name.as_str() {
            "September" => "Sept".to_owned(),
            _ => name[0..3].to_owned(),
        };

        Month {
            name,
            short_name,
            number
        }
    }
}

impl TryFrom<&str> for Months {
    type Error = IntoMonthsError;

    fn try_from(name: &str) -> Result<Self, Self::Error> {
        match name.to_lowercase().as_str() {
            "january"   | "jan" => Ok(Months::January(Month::new("January", 1))),
            "february"  | "feb" => Ok(Months::February(Month::new("February", 1))),
            "march"     | "mar" => Ok(Months::March(Month::new("March", 1))),
            "april"     | "apr" => Ok(Months::April(Month::new("April", 1))),
            "may"               => Ok(Months::May(Month::new("May", 1))),
            "june"      | "jun" => Ok(Months::June(Month::new("June", 1))),
            "july"      | "jul" => Ok(Months::July(Month::new("July", 1))),
            "august"    | "aug" => Ok(Months::August(Month::new("August", 1))),
            "september" | "sept" => Ok(Months::September(Month::new("September", 1))),
            "october"   | "oct" => Ok(Months::October(Month::new("October", 10))),
            "november"  | "nov" => Ok(Months::November(Month::new("November", 11))),
            "december"  | "dev" => Ok(Months::December(Month::new("December", 12))),
            _ => Err(IntoMonthsError::NotAMonthName),
        }
    }
}

impl TryFrom<u8> for Months {
    type Error = IntoMonthsError;

    fn try_from(number: u8) -> Result<Self, Self::Error> {
        match number {
            1  => Ok(Months::January(Month::new("January", 1))),
            2  => Ok(Months::February(Month::new("February", 1))),
            3  => Ok(Months::March(Month::new("March", 1))),
            4  => Ok(Months::April(Month::new("April", 1))),
            5  => Ok(Months::May(Month::new("May", 1))),
            6  => Ok(Months::June(Month::new("June", 1))),
            7  => Ok(Months::July(Month::new("July", 1))),
            8  => Ok(Months::August(Month::new("August", 1))),
            9  => Ok(Months::September(Month::new("September", 1))),
            10 => Ok(Months::October(Month::new("October", 10))),
            11 => Ok(Months::November(Month::new("November", 11))),
            12 => Ok(Months::December(Month::new("December", 12))),
            _ => Err(IntoMonthsError::NotAMonthNumber),
        }
    }
}

struct Day {
    name: String,
    short_name: String,
    day_type: DayType,
    day_of_year: u16,
    day_of_month: u16,
    week_number: u16,
}

enum Days {
    Monday(Day),
    Tuesday(Day),
    Wednesday(Day),
    Thursday(Day),
    Friday(Day),
    Saturday(Day),
    Sunday(Day),
}

enum DayType {
    Weekday,
    Weekend,
}

enum IntoDaysError {
    NotADayName,
}

impl Day {
    fn new(name: &str, day_of_year: u16, day_of_month: u16, week_number: u16) -> Self {
        let name = name.to_owned();
        let short_name = name[0..3].to_owned();
        let day_type = match short_name.to_lowercase().as_str() {
            "mon" | "tue" | "wed" | "thu" | "fri" => DayType::Weekday,
            "sat" | "sun" => DayType::Weekend,
            _ => unreachable!(),
        };
        let day_of_year = 0;
        let day_of_month = 0;
        let week_number = 0;

        Day {
            name,
            short_name,
            day_type,
            day_of_year,
            day_of_month,
            week_number
        }
    }
}

impl TryFrom<&str> for Days {
    type Error = IntoDaysError;

    fn try_from(name: &str) -> Result<Self, Self::Error> {
        match name.to_lowercase().as_str() {
            "monday"    | "mon" => Ok(Days::Monday(Day::new("Monday", 0, 0, 0))),
            "tuesday"   | "tue" => Ok(Days::Tuesday(Day::new("Tuesday", 0, 0, 0))),
            "wednesday" | "wed" => Ok(Days::Wednesday(Day::new("Wednesday", 0, 0, 0))),
            "thursday"  | "thu" => Ok(Days::Thursday(Day::new("Thursday", 0, 0, 0))),
            "friday"    | "fri" => Ok(Days::Friday(Day::new("Friday", 0, 0, 0))),
            "saturday"  | "sat" => Ok(Days::Saturday(Day::new("Saturday", 0, 0, 0))),
            "sunday"    | "sun" => Ok(Days::Sunday(Day::new("Sunday", 0, 0, 0))),
            _ => Err(IntoDaysError::NotADayName),
        }
    }
}

struct HorizonDate {
    year: u16,
    month: Months,
    day: Days,
}

struct HorizonTime {
    hour: u8,
    minute: u8,
    second: u8,
    timezone: String,
}

// Important things
//      - the actual data source (method call, file, dbus, etc)
//      - the marshalling function
pub struct HorizonDateTime {
    date: HorizonDate,
    time: HorizonTime,
}

impl From<File> for HorizonDateTime {
    fn from(json_file: File) -> Self {
        todo!()
    }
}

pub fn get_now_local() -> chrono::DateTime<Local> {
    chrono::Local::now()
}

pub fn get_now_utc() -> chrono::DateTime<Utc> {
    chrono::Utc::now()
}
