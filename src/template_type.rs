use std::path::Path;

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

impl Template {
    pub fn get_path(&self) -> &str {
        match self {
            Template::Component => if self.home_dir().exists() { "./ui" } else { "." },
            Template::Entity => if self.home_dir().exists() { "./entities" } else { "." },
            Template::Model => if self.home_dir().exists() { "./models" } else { "." },
            Template::Unknown => ".",
        }
    }

    fn home_dir(&self) -> &Path {
        match self {
            Template::Component => Path::new("./ui"),
            Template::Entity => Path::new("./entities"),
            Template::Model => Path::new("./models"),
            Template::Unknown => Path::new("."),
        }
    }
}