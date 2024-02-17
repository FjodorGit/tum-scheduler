use roxmltree::Document;

use super::{TumApiError, TumXmlError, TumXmlNode};

#[derive(Debug)]
pub struct CourseVariant {
    pub curriculum: String,
    pub facultiy: String,
    pub abbreviation: String,
}

impl TryFrom<TumXmlNode<'_, '_>> for CourseVariant {
    type Error = TumXmlError;
    fn try_from(resource_node: TumXmlNode<'_, '_>) -> Result<Self, Self::Error> {
        let curriculum = resource_node.get_text_of_next("curriculumVersionId")?;
        let first_back_node = resource_node.get_next("back")?;
        let abbreviation = first_back_node.get_text_of_next("designation")?;
        let facultiy: String = abbreviation
            .chars()
            .filter(|c| c.is_ascii_alphabetic())
            .collect();
        let variant = CourseVariant {
            curriculum,
            facultiy,
            abbreviation,
        };
        Ok(variant)
    }
}

impl CourseVariant {
    fn read_all_from_page(xml: String) -> Result<Vec<CourseVariant>, TumApiError> {
        let document = Document::parse(&xml)?;
        let root_element = TumXmlNode(document.root_element());

        let mut variants = vec![];
        for resource_elem in root_element.resource_elements() {
            let variant = Self::try_from(resource_elem)?;
            variants.push(variant);
        }
        Ok(variants)
    }
}

#[cfg(test)]
mod test {
    use std::fs;

    use crate::tum_api::course_variant::CourseVariant;

    #[test]
    fn test_reading_variants() {
        let test_xml: String = fs::read_to_string("test_xmls/course_variants.xml")
            .expect("Should be able to read course variant test file");
        let variants =
            CourseVariant::read_all_from_page(test_xml).expect("should be able to read variants");
        assert_eq!(variants.len(), 26);
    }
}
