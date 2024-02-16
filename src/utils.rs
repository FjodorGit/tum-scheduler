use roxmltree::Node;

pub fn element_has_name(node: &Node<'_, '_>, name: &str) -> bool {
    node.is_element() && node.tag_name().name() == name
}
