use std::fs::{self, File};
use std::io::Error;
use std::io::prelude::*;
use std::path::Path;
use crate::utils::to_pascal_case;

pub enum TemplateType {
    Component,
    Entity,
    Model,
    GetUseCase,
    MutationUseCase,
    Unknown,
}

impl From<&str> for TemplateType {
    fn from(item: &str) -> Self {
        match item {
            "component" | "c" => TemplateType::Component,
            "entity" | "e" => TemplateType::Entity,
            "model" | "m" => TemplateType::Model,
            "get-use-case" | "g" => TemplateType::GetUseCase,
            "mutation-use-case" | "mu" => TemplateType::MutationUseCase,
            _ => TemplateType::Unknown,
        }
    }
}

impl TemplateType {
    fn get_path(&self) -> &str {
        match self {
            TemplateType::Component => if self.home_dir().exists() { "./ui" } else { "." },
            TemplateType::Entity => if self.home_dir().exists() { "./entities" } else { "." },
            TemplateType::Model => if self.home_dir().exists() { "./models" } else { "." },
            TemplateType::GetUseCase => if self.home_dir().exists() { "./use-cases" } else { "." },
            TemplateType::MutationUseCase => if self.home_dir().exists() { "./use-cases" } else { "." },
            TemplateType::Unknown => ".",
        }
    }

    fn home_dir(&self) -> &Path {
        match self {
            TemplateType::Component => Path::new("./ui"),
            TemplateType::Entity => Path::new("./entities"),
            TemplateType::Model => Path::new("./models"),
            TemplateType::GetUseCase => Path::new("./use-cases"),
            TemplateType::MutationUseCase => Path::new("./use-cases"),
            TemplateType::Unknown => Path::new("."),
        }
    }

    pub fn create_files(&self, name: &str) -> Result<(), Error> {
        let pascal_name = to_pascal_case(&name);

        match self {
            TemplateType::Component => {
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

            TemplateType::Entity => {
                let path = self.get_path();

                if Path::exists(Path::new(format!("{path}/{name}.entity.ts").as_str())) {
                    println!("Entity {pascal_name} already exists.");
                    return Ok(());
                }

                create_entity_file(&path, &name, &pascal_name)?;

                println!("Entity {pascal_name} created.");
            },

            TemplateType::Model => {
                let path = self.get_path();

                if Path::exists(Path::new(format!("{path}/{name}.model.ts").as_str())) {
                    println!("Model {pascal_name} already exists.");
                    return Ok(());
                }

                create_model_file(&path, &name, &pascal_name)?;

                println!("Model {pascal_name} created.");
            },

            TemplateType::GetUseCase => {
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

            TemplateType::MutationUseCase => {
                let path = self.get_path();

                if Path::exists(Path::new(format!("{path}/{name}").as_str())) {
                    println!("Mutation use case {name} already exists.");
                    return Ok(());
                }

                fs::create_dir(format!("{path}/{name}"))?;

                create_mutation_use_case_index_file(&path, &name)?;
                create_mutation_use_case_mutation_file(&path, &name, &pascal_name)?;

                println!("Mutation use case {pascal_name} created.");
            },

            TemplateType::Unknown => {
                println!("Unknown template type.");
            },
        }

        Ok(())
    }
}

fn create_mutation_use_case_index_file(path: &str, name: &str) -> Result<(), Error> {
    let template = include_str!("./templates/mutation-use-case/index_template.ts")
        .replace("$name$", &name);

    let mut file = File::create(format!("{path}/{name}/index.ts"))?;
    write!(file, "{template}")?;

    Ok(())
}

fn create_mutation_use_case_mutation_file(path: &str, name: &str, pascal_name: &str) -> Result<(), Error> {
    let template = include_str!("./templates/mutation-use-case/mutation_template.ts")
        .replace("$name$", &name)
        .replace("$pascal_name$", &pascal_name);

    let mut file = File::create(format!("{path}/{name}/use-{name}-mutation.ts"))?;
    write!(file, "{template}")?;

    Ok(())
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