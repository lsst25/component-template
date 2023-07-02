pub enum TemplateType {
    Component,
    Entity,
    Model,
    Unknown,
}

impl From<&str> for TemplateType {
    fn from(item: &str) -> Self {
        match item {
            "component" | "c" => TemplateType::Component,
            "entity" | "e" => TemplateType::Entity,
            "model" | "m" => TemplateType::Model,
            _ => TemplateType::Unknown,
        }
    }
}