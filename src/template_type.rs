use std::fs::{self, File};
use std::io::{ Error, prelude::* };
use std::path::Path;
use crate::utils::to_pascal_case;

pub struct TemplateBuilder {
    template_type: TemplateType,
    name: String,
    pub pascal_name: String,
    path: String,
}

impl TemplateBuilder {
    pub fn new(template_type: &str, name: &str) -> Self {
        let template_type = TemplateType::from(template_type);
        let pascal_name = to_pascal_case(&name);
        let path = template_type.get_path();

        TemplateBuilder {
            name: name.to_string(),
            pascal_name,
            path: path.to_string(),
            template_type,
        }
    }

    pub fn create_files(&self) -> Result<(), Error> {
        let name = &self.name;
        let pascal_name = &self.pascal_name;
        let path = &self.path;

        match self.template_type {
            TemplateType::Component => {
                if Path::exists(Path::new(format!("{path}/{name}").as_str())) {
                    println!("Component {pascal_name} already exists.");
                    return Ok(());
                }

                fs::create_dir(format!("{path}/{name}"))?;

                self.create_index_file()?;
                self.create_component_file()?;
                self.create_stories_file()?;

                println!("Component {pascal_name} created.");
                Ok(())
            },

            TemplateType::Entity => {
                if Path::exists(Path::new(format!("{path}/{name}.entity.ts").as_str())) {
                    println!("Entity {pascal_name} already exists.");
                    return Ok(());
                }

                self.create_entity_file()?;

                println!("Entity {pascal_name} created.");
                Ok(())
            },

            TemplateType::Model => {
                if Path::exists(Path::new(format!("{path}/{name}.model.ts").as_str())) {
                    println!("Model {pascal_name} already exists.");
                    return Ok(());
                }

                self.create_model_file()?;

                println!("Model {pascal_name} created.");
                Ok(())
            },

            TemplateType::GetUseCase => {
                if Path::exists(Path::new(format!("{path}/get-{name}").as_str())) {
                    println!("Get use case {name} already exists.");
                    return Ok(());
                }

                fs::create_dir(format!("{path}/get-{name}"))?;

                self.create_use_case_index_file()?;
                self.create_get_use_case_query_file()?;
                self.create_get_use_case_query_keys_file()?;

                println!("Get use case {pascal_name} created.");
                Ok(())
            },

            TemplateType::MutationUseCase => {
                if Path::exists(Path::new(format!("{path}/mutation-{name}").as_str())) {
                    println!("Mutation use case {name} already exists.");
                    return Ok(());
                }

                fs::create_dir(format!("{path}/mutation-{name}"))?;

                self.create_mutation_use_case_index_file()?;
                self.create_mutation_use_case_mutation_file()?;
                Ok(())
            },

            TemplateType::Unknown => {
                println!("Unknown template type.");
                Ok(())
            },
        }
    }


    fn create_mutation_use_case_index_file(&self) -> Result<(), Error> {
        let template = include_str!("./templates/mutation-use-case/index_template.ts")
            .replace("$name$", &self.name);

        let mut file = File::create(format!("{}/{}/index.ts", &self.path, &self.name))?;
        write!(file, "{template}")?;

        Ok(())
    }

    fn create_mutation_use_case_mutation_file(&self) -> Result<(), Error> {
        let template = include_str!("./templates/mutation-use-case/mutation_template.ts")
            .replace("$name$", &self.name)
            .replace("$pascal_name$", &self.pascal_name);

        let mut file = File::create(format!("{}/{}/use-{}-mutation.ts", &self.path, &self.name, &self.name))?;
        write!(file, "{template}")?;

        Ok(())
    }

    fn create_use_case_index_file(&self) -> Result<(), Error> {
        let template = include_str!("./templates/get-use-case/index_template.ts")
            .replace("$name$", &self.name);

        let mut file = File::create(format!("{}/get-{}/index.ts", &self.path, &self.name))?;
        write!(file, "{template}")?;

        Ok(())
    }

    fn create_get_use_case_query_file(&self) -> Result<(), Error> {
        let template = include_str!("./templates/get-use-case/query_template.ts")
            .replace("$name$", &self.name)
            .replace("$pascal_name$", &self.pascal_name);

        let mut file = File::create(format!("{}/get-{}/use-{}-query.ts", &self.path, &self.name, &self.name))?;
        write!(file, "{template}")?;

        Ok(())
    }

    fn create_get_use_case_query_keys_file(&self) -> Result<(), Error> {
        let template = include_str!("./templates/get-use-case/query_keys_template.ts")
            .replace("$name$", &self.name)
            .replace("$pascal_name$", &self.pascal_name);

        let mut file = File::create(format!("{}/get-{}/{}-query-keys.ts", &self.path, &self.name, &self.name))?;
        write!(file, "{template}")?;

        Ok(())
    }

    fn create_component_file(&self) -> Result<(), Error> {
        let template = include_str!("./templates/component/component_template.tsx")
            .replace("$pascal_name$", &self.pascal_name);

        let mut file = File::create(format!("{}/{}/{}.component.tsx", &self.path, &self.name, &self.name))?;
        write!(file, "{template}")?;

        Ok(())
    }

    fn create_index_file(&self) -> Result<(), Error> {
        let template = include_str!("./templates/component/index_template.ts")
            .replace("$name$", &self.name);

        let mut file = File::create(format!("{}/{}/index.ts", &self.path, &self.name))?;
        write!(file, "{template}")?;

        Ok(())
    }

    fn create_entity_file(&self) -> Result<(), Error> {
        let template = include_str!("./templates/entity/entity_template.ts")
            .replace("$pascal_name$", &self.pascal_name);

        let mut file = File::create(format!("{}/{}.entity.ts", &self.path, &self.name))?;
        write!(file, "{template}")?;

        Ok(())
    }

    fn create_model_file(&self) -> Result<(), Error> {
        let template = include_str!("./templates/model/model_template.ts")
            .replace("$pascal_name$", &self.pascal_name)
            .replace("$name$", &self.name);

        let mut file = File::create(format!("{}/{}.model.ts", &self.path, &self.name))?;
        write!(file, "{template}")?;

        Ok(())
    }

    fn create_stories_file(&self) -> Result<(), Error> {
        let template = include_str!("./templates/component/stories_template.tsx")
            .replace("$pascal_name$", &self.pascal_name)
            .replace("$name$", &self.name);

        let mut file = File::create(format!("{}/{}/{}.stories.tsx", &self.path, &self.name, &self.name))?;
        write!(file, "{template}")?;

        Ok(())
    }
}


enum TemplateType {
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
}
