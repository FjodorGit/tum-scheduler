use anyhow::Result;
use roxmltree::{Document, Node};
use std::env;
use std::str::FromStr;

use crate::utils::element_has_name;
use chrono::NaiveTime;
use chrono::Weekday;
use thiserror::Error;

#[derive(Debug)]
pub struct Appointment {
    pub weekday: Weekday,
    pub from: NaiveTime,
    pub to: NaiveTime,
}

#[derive(Debug, Error)]
pub enum AppointmentError {
    #[error("Failed to get text of series element")]
    ElementNoTextError,
    #[error("Failed to parse time for {0}")]
    TimeParseError(String),
    #[error("No {0} was specified")]
    NoTimeError(String),
}

impl TryFrom<Node<'_, '_>> for Appointment {
    type Error = AppointmentError;
    fn try_from(appointment_series_node: Node<'_, '_>) -> Result<Self, Self::Error> {
        let start_time = appointment_series_node
            .descendants()
            .filter(|n| element_has_name(n, "seriesBeginTime"))
            .map(|n| n.text().ok_or(AppointmentError::ElementNoTextError))
            .map(|s| {
                NaiveTime::from_str(s?)
                    .map_err(|_| AppointmentError::TimeParseError("Startime".to_string()))
            })
            .next()
            .ok_or(AppointmentError::NoTimeError("Startime".to_string()))??;
        let end_time = appointment_series_node
            .descendants()
            .filter(|n| element_has_name(n, "seriesEndTime"))
            .map(|n| n.text().ok_or(AppointmentError::ElementNoTextError))
            .map(|s| {
                NaiveTime::from_str(s?)
                    .map_err(|_| AppointmentError::TimeParseError("Endtime".to_string()))
            })
            .next()
            .ok_or(AppointmentError::NoTimeError("Endtime".to_string()))??;
        let weekday = appointment_series_node
            .descendants()
            .filter(|n| element_has_name(n, "translation") && n.has_attribute("lang"))
            .filter_map(|n| n.text())
            .find_map(|s| Weekday::from_str(s).ok())
            .ok_or(AppointmentError::NoTimeError("Weekday".to_string()))?;
        println!("{:#?}", start_time);
        println!("{:#?}", end_time);
        println!("{:#?}", weekday);
        let app = Appointment {
            weekday,
            from: start_time,
            to: end_time,
        };

        Err(AppointmentError::NoTimeError("Weekday".to_string()))
    }
}

impl Appointment {
    pub async fn get_recuring_appointments(course_id: &str) -> Result<Vec<Appointment>> {
        println!("Requesting appointement for {}", course_id);
        let appointments: Vec<Appointment> = vec![];
        let mut request_url = env::var("APPOINTMENT_URL")
            .expect("APPOINTMENT_URL should exist in environment variables");
        request_url.push_str(course_id);
        let request_result = reqwest::get(request_url).await?;
        let xml: String = request_result.text().await?;
        let document = Document::parse(&xml).expect("Returned APPOINTEMENT XML should be valid");
        println!("Got valid document");
        let root_element = document.root_element();
        for appointment_series_element in root_element
            .descendants()
            .filter(|n| n.is_element() && n.tag_name().name() == "appointmentSeriesDtos")
        {
            let appointment = Appointment::try_from(appointment_series_element)?;
            println!("{:#?}", appointment);
        }
        Ok(vec![])
    }
}
