pub enum TemplateType {
    Component,
    Entity,
    Unknown,
}

impl From<&str> for TemplateType {
    fn from(item: &str) -> Self {
        match item {
            "component" | "c" => TemplateType::Component,
            "entity" | "e" => TemplateType::Entity,
            _ => TemplateType::Unknown,
        }
    }
}