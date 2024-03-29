use roxmltree::Node;
use thiserror::Error;

pub struct TumXmlNode<'a, 'input>(Node<'a, 'input>);

#[derive(Debug, Error)]
pub enum TumXmlError {
    #[error("Failed to parse resource node: {0}")]
    TumNodeParseError(String),
    #[error("Failed to parse weekday `{0}`")]
    TumWeekdayParseError(#[from] chrono::ParseWeekdayError),
    #[error("Failed to find organization")]
    MissingOrganization,
    #[error("Failed to parse time `{0}`")]
    TumTimeParseError(#[from] chrono::ParseError),
}

impl<'a, 'input> TumXmlNode<'a, 'input> {
    pub fn new(node: Node<'a, 'input>) -> Self {
        Self(node)
    }
    pub fn resource_elements(&self) -> impl std::iter::Iterator<Item = Self> + '_ {
        self.get_all_nodes("resource")
    }

    pub fn get_all_nodes<'b>(
        &'b self,
        nodes_name: &'b str,
    ) -> impl std::iter::Iterator<Item = Self> + '_ {
        self.0
            .descendants()
            .filter(move |n| element_has_name(n, nodes_name))
            .map(TumXmlNode)
    }

    pub fn get_text_of_next(&self, node_name: &str) -> Result<String, TumXmlError> {
        let node_text = self
            .0
            .descendants()
            .filter(|n| element_has_name(n, node_name))
            .find_map(|n| n.text())
            .ok_or(TumXmlError::TumNodeParseError(format!(
                "No node with name `{}` found",
                node_name
            )))?;
        Ok(node_text.to_owned())
    }

    pub fn get_text_of_last(&self, node_name: &str) -> Result<String, TumXmlError> {
        let node_text = self
            .0
            .descendants()
            .filter(|n| element_has_name(n, node_name))
            .filter_map(|n| n.text())
            .last()
            .ok_or(TumXmlError::TumNodeParseError(format!(
                "No with name `{}` node found",
                node_name
            )))?;
        Ok(node_text.to_owned())
    }

    pub fn get_next(&self, node_name: &str) -> Result<Self, TumXmlError> {
        let node = self
            .0
            .descendants()
            .find(|n| element_has_name(n, node_name))
            .ok_or(TumXmlError::TumNodeParseError(format!(
                "No with name `{}` node found",
                node_name
            )))?;
        Ok(Self(node))
    }

    pub fn get_next_sibling(&self) -> Result<Self, TumXmlError> {
        let current_node_name = self.0.tag_name().name();
        let node = self
            .0
            .next_sibling_element()
            .ok_or(TumXmlError::TumNodeParseError(format!(
                "No sibling found for node `{}`",
                current_node_name
            )))?;
        Ok(Self(node))
    }

    pub fn get_translations(&self) -> Result<(String, String), TumXmlError> {
        let current_node_name = self.0.tag_name().name();
        let mut node_with_translations = self
            .0
            .descendants()
            .filter(|n| element_has_name(n, "translation"))
            .filter_map(|n| n.text());
        let name_de: &str = node_with_translations
            .next()
            .ok_or(TumXmlError::TumNodeParseError(format!(
                "No german translation found for node `{}`",
                current_node_name
            )))?;
        let name_en: &str = node_with_translations
            .next()
            .ok_or(TumXmlError::TumNodeParseError(format!(
                "No english translation found for node `{}`",
                current_node_name
            )))?;
        Ok((name_de.to_owned(), name_en.to_owned()))
    }
}

pub fn element_has_name(node: &Node<'_, '_>, name: &str) -> bool {
    node.is_element() && node.tag_name().name() == name
}
