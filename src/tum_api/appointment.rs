use roxmltree::Document;
use std::env;
use std::str::FromStr;

use chrono::NaiveTime;

use super::tum_xml_node::TumXmlNode;
use super::ScraperError;
use super::TumXmlError;

#[derive(Debug)]
pub struct AppointmentFromXml {
    pub weekdays: Vec<String>,
    pub start_time: NaiveTime,
    pub end_time: NaiveTime,
}

#[derive(Debug)]
pub struct AppointmentsEndpoint {
    base_request_url: String,
}

impl TryFrom<TumXmlNode<'_, '_>> for AppointmentFromXml {
    type Error = TumXmlError;
    fn try_from(appointment_series_node: TumXmlNode<'_, '_>) -> Result<Self, Self::Error> {
        let start_time_text = appointment_series_node.get_text_of_next("seriesBeginTime")?;
        let start_time = NaiveTime::from_str(&start_time_text)?;

        let end_time_text = appointment_series_node.get_text_of_next("seriesEndTime")?;
        let end_time = NaiveTime::from_str(&end_time_text)?;

        let weekdays = appointment_series_node
            .get_all_nodes("weekday")
            .filter_map(|n| n.get_translations().ok())
            .map(|(_, w)| w)
            .collect();

        let app = AppointmentFromXml {
            weekdays,
            start_time,
            end_time,
        };

        Ok(app)
    }
}

impl AppointmentFromXml {
    fn read_all_from_page(xml: String) -> Result<Vec<AppointmentFromXml>, ScraperError> {
        let mut appointments: Vec<AppointmentFromXml> = vec![];
        let document = Document::parse(&xml)?;
        let root_element = TumXmlNode::new(document.root_element());
        for appointment_series_element in root_element.get_all_nodes("appointmentSeriesDtos") {
            let appointment = AppointmentFromXml::try_from(appointment_series_element)?;
            appointments.push(appointment);
        }
        Ok(appointments)
    }
}

impl AppointmentsEndpoint {
    pub fn new() -> Self {
        let base_request_url = env::var("APPOINTMENT_URL")
            .expect("APPOINTMENT_URL should exist in environment variables");
        AppointmentsEndpoint { base_request_url }
    }

    pub async fn get_recurring_by_id(
        &self,
        course_id: &str,
    ) -> Result<Vec<AppointmentFromXml>, ScraperError> {
        let request_url = format!("{}{}", self.base_request_url, course_id);
        let request_result = reqwest::get(request_url).await?;
        let xml: String = request_result.text().await?;
        AppointmentFromXml::read_all_from_page(xml)
    }
}

#[cfg(test)]
mod test {
    use std::fs;

    use crate::tum_api::appointment::AppointmentFromXml;

    #[test]
    fn test_reading_appointments() {
        let test_xml: String = fs::read_to_string("test_xmls/appointments.xml")
            .expect("Should be able to read appointment test file");
        let appointments = AppointmentFromXml::read_all_from_page(test_xml)
            .expect("should be able to read appointments");
        println!("{:#?}", appointments);
        assert_eq!(appointments.len(), 2);
    }

    #[test]
    fn test_reading_appointment_multiweekday() {
        let test_xml: String = fs::read_to_string("test_xmls/appointment_multi_weekday.xml")
            .expect("Should be able to read course variant test file");
        let appointments = AppointmentFromXml::read_all_from_page(test_xml)
            .expect("should be able to read variants");
        assert_eq!(appointments.len(), 1);
        assert_eq!(appointments.get(0).unwrap().weekdays.len(), 5);
    }
}
