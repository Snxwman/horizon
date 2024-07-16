use std::fs::File;
use std::sync::RwLock;

use chrono::prelude::*;
use once_cell::sync::Lazy;
use tokio::sync::watch;
use tokio::sync::watch::Sender;

use super::ChannelMessage;

pub static DATETIME: Lazy<RwLock<HorizonDateTime>> = Lazy::new(|| {
    RwLock::new(HorizonDateTime::new_from_chrono())
});

// Important things
//      - the actual data source (method call, file, dbus, etc)
//      - the marshalling function
#[derive(Debug)]
pub struct HorizonDateTime {
    pub date: HorizonDate,
    pub time: HorizonTime,
    pub sender: Sender<ChannelMessage>,
}

#[derive(Debug)]
pub struct HorizonDate {
    pub year: u32,
    pub month: Month,
    pub day: Day,
}

#[derive(Debug)]
pub struct HorizonTime {
    pub hour: u32,
    pub minute: u32,
    pub second: u32,
    // pub timezone: String,
}

#[derive(Debug)]
pub struct Month {
    pub name: String,
    pub short_name: String,
    pub number: u32,
    pub month: Months,
}

#[derive(Debug)]
pub struct Day {
    pub name: String,
    pub short_name: String,
    pub weekday: Weekday,
    pub day_type: DayType,
    pub day_of_year: u32,
    pub day_of_month: u32,
    pub week_number: u32,
}

#[derive(Debug)]
pub enum Months {
    January,
    February,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December,
}

#[derive(Debug)]
pub enum Weekday {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

#[derive(Debug)]
pub enum DayType {
    Weekday,
    Weekend,
}

#[derive(Debug)]
pub enum IntoMonthsError {
    NotAMonthName,
    NotAMonthNumber,
}

#[derive(Debug)]
pub enum IntoDaysError {
    NotADayName,
}

impl HorizonDateTime {
    pub fn new_from_chrono() -> Self {
        let (sender, reveiver) = watch::channel(ChannelMessage::Init);

        Self {
            date: HorizonDate::new_from_chrono(),
            time: HorizonTime::new_from_chrono(),
            sender,
        }
    }

    pub fn update(&mut self) {
        self.update_time_from_chrono();
    }

    pub fn update_date_from_chrono(&mut self) {
        self.date = HorizonDate::new_from_chrono();
    }

    pub fn update_time_from_chrono(&mut self) {
        self.time = HorizonTime::new_from_chrono();
    }
}

impl HorizonDate {
    pub fn new_from_chrono() -> Self {
        let now = Local::now();

        let year = now.year() as u32;
        let month = Month::new(
            Months::try_from(now.month()).unwrap().to_string().as_str(),
            now.month(),
        );
        let day = Day::new(
            now.weekday().to_string().as_str(),
            now.ordinal(),
            now.day(),
            now.iso_week().week(),
        );

        Self {
            year,
            month,
            day,
        }
    }
}

impl HorizonTime {
    pub fn new_from_chrono() -> Self {
        let now = Local::now();
        let tz = now.timezone();

        Self {
            hour: now.hour(),
            minute: now.minute(),
            second: now.second(),
        }
    }
}

impl From<File> for HorizonDateTime {
    fn from(json_file: File) -> Self {
        todo!()
    }
}

impl From<File> for HorizonDate {
    fn from(json_file: File) -> Self {
        todo!()
    }
}

impl From<File> for HorizonTime {
    fn from(json_file: File) -> Self {
        todo!()
    }
}

impl Month {
    fn new(name: &str, number: u32) -> Self {
        let name = name.to_owned();
        let short_name = match name.as_str() {
            "September" => "Sept".to_owned(),
            _ => name[0..3].to_owned(),
        };
        let month = Months::try_from(name.as_str())
            .unwrap_or(Months::try_from("january").unwrap());

        Month {
            name,
            short_name,
            number,
            month,
        }
    }
}

impl Months {
    fn to_string(&self) -> String {
        match self {
            Self::January => String::from("January"),
            Self::February => String::from("February"),
            Self::March => String::from("March"),
            Self::April => String::from("April"),
            Self::May => String::from("May"),
            Self::June => String::from("June"),
            Self::July => String::from("July"),
            Self::August => String::from("August"),
            Self::September => String::from("September"),
            Self::October => String::from("October"),
            Self::November => String::from("November"),
            Self::December => String::from("December"),
        }
    }
}

impl TryFrom<&str> for Months {
    type Error = IntoMonthsError;

    fn try_from(name: &str) -> Result<Self, Self::Error> {
        match name.to_lowercase().as_str() {
            "january"   | "jan" => Ok(Months::January),
            "february"  | "feb" => Ok(Months::February),
            "march"     | "mar" => Ok(Months::March),
            "april"     | "apr" => Ok(Months::April),
            "may"               => Ok(Months::May),
            "june"      | "jun" => Ok(Months::June),
            "july"      | "jul" => Ok(Months::July),
            "august"    | "aug" => Ok(Months::August),
            "september" | "sept" => Ok(Months::September),
            "october"   | "oct" => Ok(Months::October),
            "november"  | "nov" => Ok(Months::November),
            "december"  | "dev" => Ok(Months::December),
            _ => Err(IntoMonthsError::NotAMonthName),
        }
    }
}

impl TryFrom<u32> for Months {
    type Error = IntoMonthsError;

    fn try_from(number: u32) -> Result<Self, Self::Error> {
        match number {
            1  => Ok(Months::January),
            2  => Ok(Months::February),
            3  => Ok(Months::March),
            4  => Ok(Months::April),
            5  => Ok(Months::May),
            6  => Ok(Months::June),
            7  => Ok(Months::July),
            8  => Ok(Months::August),
            9  => Ok(Months::September),
            10 => Ok(Months::October),
            11 => Ok(Months::November),
            12 => Ok(Months::December),
            _ => Err(IntoMonthsError::NotAMonthNumber),
        }
    }
}

impl Day {
    fn new(name: &str, day_of_year: u32, day_of_month: u32, week_number: u32) -> Self {
        let name = name.to_owned();
        let short_name = name[0..3].to_owned();
        let weekday = Weekday::try_from(name.as_str())
            .unwrap_or(Weekday::try_from("monday").unwrap());
        let day_type = match short_name.to_lowercase().as_str() {
            "mon" | "tue" | "wed" | "thu" | "fri" => DayType::Weekday,
            "sat" | "sun" => DayType::Weekend,
            _ => unreachable!(),
        };

        Self {
            name,
            short_name,
            weekday,
            day_type,
            day_of_year,
            day_of_month,
            week_number
        }
    }
}

impl TryFrom<&str> for Weekday {
    type Error = IntoDaysError;

    fn try_from(name: &str) -> Result<Self, Self::Error> {
        match name.to_lowercase().as_str() {
            "monday"    | "mon" => Ok(Weekday::Monday),
            "tuesday"   | "tue" => Ok(Weekday::Tuesday),
            "wednesday" | "wed" => Ok(Weekday::Wednesday),
            "thursday"  | "thu" => Ok(Weekday::Thursday),
            "friday"    | "fri" => Ok(Weekday::Friday),
            "saturday"  | "sat" => Ok(Weekday::Saturday),
            "sunday"    | "sun" => Ok(Weekday::Sunday),
            _ => Err(IntoDaysError::NotADayName),
        }
    }
}

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
