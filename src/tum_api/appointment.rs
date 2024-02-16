use std::str::FromStr;

use chrono::NaiveTime;
use chrono::ParseError;
use chrono::Weekday;
use roxmltree::Document;
use thiserror::Error;

pub struct Appointments {
    pub weekdays: Vec<Weekday>,
    pub from: Vec<NaiveTime>,
    pub to: Vec<NaiveTime>,
}

#[derive(Debug, Error)]
pub enum AppointmentError {
    #[error("Failed to get text of series element")]
    ElementTextError,
    #[error("Failed to parse time")]
    TimeParseError,
}

impl TryFrom<Document<'_>> for Appointments {
    type Error = AppointmentError;
    fn try_from(xml: Document) -> Result<Self, Self::Error> {
        let root_element = xml.root_element();
        let start_times = root_element
            .descendants()
            .filter(|n| n.is_element() && n.tag_name().name() == "seriesBeginTime")
            .map(|n| n.text().ok_or(AppointmentError::ElementTextError))
            .map(|s| NaiveTime::from_str(s?).map_err(|_| AppointmentError::TimeParseError))
            .collect::<Result<Vec<NaiveTime>, AppointmentError>>()?;
        let end_times = root_element
            .descendants()
            .filter(|n| n.is_element() && n.tag_name().name() == "seriesEndTime")
            .map(|n| n.text().ok_or(AppointmentError::ElementTextError))
            .map(|s| NaiveTime::from_str(s?).map_err(|_| AppointmentError::TimeParseError))
            .collect::<Result<Vec<NaiveTime>, AppointmentError>>()?;
        println!("{:#?}", start_times);
        println!("{:#?}", end_times);
        let app = Appointments {
            weekdays: vec![Weekday::Mon],
            from: start_times,
            to: end_times,
        };
        Ok(app)
    }
}
