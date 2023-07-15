use std::fs::{self, File};
use std::io::Error;
use std::io::prelude::*;
use std::path::Path;
use crate::utils::to_pascal_case;

pub enum Template {
    Component,
    Entity,
    Model,
    GetUseCase,
    Unknown,
}

impl From<&str> for Template {
    fn from(item: &str) -> Self {
        match item {
            "component" | "c" => Template::Component,
            "entity" | "e" => Template::Entity,
            "model" | "m" => Template::Model,
            "get-use-case" | "g" => Template::GetUseCase,
            _ => Template::Unknown,
        }
    }
}

impl Template {
    fn get_path(&self) -> &str {
        match self {
            Template::Component => if self.home_dir().exists() { "./ui" } else { "." },
            Template::Entity => if self.home_dir().exists() { "./entities" } else { "." },
            Template::Model => if self.home_dir().exists() { "./models" } else { "." },
            Template::GetUseCase => if self.home_dir().exists() { "./use-cases" } else { "." },
            Template::Unknown => ".",
        }
    }

    fn home_dir(&self) -> &Path {
        match self {
            Template::Component => Path::new("./ui"),
            Template::Entity => Path::new("./entities"),
            Template::Model => Path::new("./models"),
            Template::GetUseCase => Path::new("./use-cases"),
            Template::Unknown => Path::new("."),
        }
    }

    pub fn create_files(&self, name: &str) -> Result<(), Error> {
        let pascal_name = to_pascal_case(&name);

        match self {
            Template::Component => {
                let path = self.get_path();

                if Path::exists(Path::new(format!("{path}/{name}").as_str())) {
                    println!("Component {pascal_name} already exists.");
                    return Ok(());
                }

                fs::create_dir(format!("{path}/{name}"))?;

                create_index_file(&path, &name)?;
                create_component_file(&path, &name, &pascal_name)?;
                create_stories_file(&path, &name, &pascal_name)?;

                println!("Component {pascal_name} created.");
            },

            Template::Entity => {
                let path = self.get_path();

                if Path::exists(Path::new(format!("{path}/{name}.entity.ts").as_str())) {
                    println!("Entity {pascal_name} already exists.");
                    return Ok(());
                }

                create_entity_file(&path, &name, &pascal_name)?;

                println!("Entity {pascal_name} created.");
            },

            Template::Model => {
                let path = self.get_path();

                if Path::exists(Path::new(format!("{path}/{name}.model.ts").as_str())) {
                    println!("Model {pascal_name} already exists.");
                    return Ok(());
                }

                create_model_file(&path, &name, &pascal_name)?;

                println!("Model {pascal_name} created.");
            },

            Template::GetUseCase => {
                let path = self.get_path();

                if Path::exists(Path::new(format!("{path}/get-{name}").as_str())) {
                    println!("Get use case {name} already exists.");
                    return Ok(());
                }

                fs::create_dir(format!("{path}/get-{name}"))?;

                create_use_case_index_file(&path, &name)?;
                create_get_use_case_query_file(&path, &name, &pascal_name)?;
                create_get_use_case_query_keys_file(&path, &name, &pascal_name)?;

                println!("Get use case {pascal_name} created.");
            },

            Template::Unknown => {
                println!("Unknown template type.");
            },
        }

        Ok(())
    }
}

fn create_use_case_index_file(path: &str, name: &str) -> Result<(), Error> {
    let template = include_str!("./templates/get-use-case/index_template.ts")
        .replace("$name$", &name);

    let mut file = File::create(format!("{path}/get-{name}/index.ts"))?;
    write!(file, "{template}")?;

    Ok(())
}

fn create_get_use_case_query_file(path: &str, name: &str, pascal_name: &str) -> Result<(), Error> {
    let template = include_str!("./templates/get-use-case/query_template.ts")
        .replace("$name$", &name)
        .replace("$pascal_name$", &pascal_name);

    let mut file = File::create(format!("{path}/get-{name}/use-{name}-query.ts"))?;
    write!(file, "{template}")?;

    Ok(())
}

fn create_get_use_case_query_keys_file(path: &str, name: &str, pascal_name: &str) -> Result<(), Error> {
    let template = include_str!("./templates/get-use-case/query_keys_template.ts")
        .replace("$name$", &name)
        .replace("$pascal_name$", &pascal_name);

    let mut file = File::create(format!("{path}/get-{name}/{name}-query-keys.ts"))?;
    write!(file, "{template}")?;

    Ok(())
}

fn create_component_file(path: &str, name: &str, pascal_name: &str) -> Result<(), Error> {
    let template = include_str!("./templates/component/component_template.tsx")
        .replace("$pascal_name$", &pascal_name);

    let mut file = File::create(format!("{path}/{name}/{name}.component.tsx"))?;
    write!(file, "{template}")?;

    Ok(())
}

fn create_index_file(path: &str, name: &str) -> Result<(), Error> {
    let template = include_str!("./templates/component/index_template.ts")
        .replace("$name$", name);

    let mut file = File::create(format!("{path}/{name}/index.ts"))?;
    write!(file, "{template}")?;

    Ok(())
}

fn create_entity_file(path: &str, name: &str, pascal_name: &str) -> Result<(), Error> {
    let template = include_str!("./templates/entity/entity_template.ts")
        .replace("$pascal_name$", &pascal_name);

    let mut file = File::create(format!("{path}/{name}.entity.ts"))?;
    write!(file, "{template}")?;

    Ok(())
}

fn create_model_file(path: &str, name: &str, pascal_name: &str) -> Result<(), Error> {
    let template = include_str!("./templates/model/model_template.ts")
        .replace("$pascal_name$", &pascal_name)
        .replace("$name$", &name);

    let mut file = File::create(format!("{path}/{name}.model.ts"))?;
    write!(file, "{template}")?;

    Ok(())
}

fn create_stories_file(path: &str, name: &str, pascal_name: &str) -> Result<(), Error> {
    let template = include_str!("./templates/component/stories_template.tsx")
        .replace("$pascal_name$", &pascal_name)
        .replace("$name$", &name);

    let mut file = File::create(format!("{path}/{name}/{name}.stories.tsx"))?;
    write!(file, "{template}")?;

    Ok(())
}