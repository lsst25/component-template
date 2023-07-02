pub enum TemplateType {
    Component,
    Unknown,
}

impl From<&str> for TemplateType {
    fn from(item: &str) -> Self {
        match item {
            "component" | "c" => TemplateType::Component,
            _ => TemplateType::Unknown,
        }
    }
}