pub enum Template {
    Component,
    Entity,
    Model,
    Unknown,
}

impl From<&str> for Template {
    fn from(item: &str) -> Self {
        match item {
            "component" | "c" => Template::Component,
            "entity" | "e" => Template::Entity,
            "model" | "m" => Template::Model,
            _ => Template::Unknown,
        }
    }
}