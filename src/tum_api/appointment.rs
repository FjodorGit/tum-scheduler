use roxmltree::Document;
use std::env;
use std::str::FromStr;

use chrono::NaiveTime;
use chrono::Weekday;

use super::TumApiError;
use super::TumXmlError;
use super::TumXmlNode;

#[derive(Debug)]
pub struct Appointment {
    pub weekday: Weekday,
    pub from: NaiveTime,
    pub to: NaiveTime,
}

impl TryFrom<TumXmlNode<'_, '_>> for Appointment {
    type Error = TumXmlError;
    fn try_from(appointment_series_node: TumXmlNode<'_, '_>) -> Result<Self, Self::Error> {
        let start_time_text = appointment_series_node.get_text_of_next("seriesBeginTime")?;
        let start_time = NaiveTime::from_str(&start_time_text)?;

        let end_time_text = appointment_series_node.get_text_of_next("seriesEndTime")?;
        let end_time = NaiveTime::from_str(&end_time_text)?;

        let (_, weekday) = appointment_series_node.get_translations()?;
        let app = Appointment {
            weekday: Weekday::from_str(&weekday)?,
            from: start_time,
            to: end_time,
        };

        Ok(app)
    }
}

impl Appointment {
    pub async fn get_recuring_appointments(
        course_id: &str,
    ) -> Result<Vec<Appointment>, TumApiError> {
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
            let appointment = Appointment::try_from(TumXmlNode(appointment_series_element))?;
            println!("{:#?}", appointment);
        }
        Ok(vec![])
    }
}
