mod template_type;
mod utils;

use std::env;
use std::fs::{self, File};
use std::io::Error;
use std::io::prelude::*;
use std::path::Path;

use template_type::TemplateType;
use utils::to_pascal_case;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Please provide a template and a name.");
        return Ok(());
    }

    let template_type = TemplateType::from(args[1].as_str());

    match template_type {
        TemplateType::Component => {
            let name = &args[2];
            let pascal_name = to_pascal_case(name);
            let path = if Path::new("./ui").exists() { "./ui" } else { "." };

            if Path::exists(Path::new(format!("{}/{}", path, name).as_str())) {
                println!("Component {} already exists.", pascal_name);
                return Ok(());
            }

            fs::create_dir(format!("{}/{}", path, name))?;

            create_index_file(path, name)?;
            create_component_file(path, name, &pascal_name)?;
            create_stories_file(path, name, &pascal_name)?;

            println!("Component {} created.", pascal_name);
        },

        TemplateType::Entity => {
            let name = &args[2];
            let pascal_name = to_pascal_case(name);
            let path = if Path::new("./entities").exists() { "./entities" } else { "." };

            if Path::exists(Path::new(format!("{}/{}.entity.ts", path, name).as_str())) {
                println!("Entity {} already exists.", pascal_name);
                return Ok(());
            }

            create_entity_file(path, name, &pascal_name)?;

            println!("Entity {} created.", pascal_name);
        },

        TemplateType::Model => {
            let name = &args[2];
            let pascal_name = to_pascal_case(name);
            let path = if Path::new("./models").exists() { "./models" } else { "." };

            if Path::exists(Path::new(format!("{}/{}.model.ts", path, name).as_str())) {
                println!("Model {} already exists.", pascal_name);
                return Ok(());
            }

            create_model_file(path, name, &pascal_name)?;

            println!("Model {} created.", pascal_name);
        },

        TemplateType::Unknown => {
            println!("Unknown template: {}", args[1]);
            return Ok(());
        }
    }

    Ok(())
}

fn create_component_file(path: &str, name: &String, pascal_name: &String) -> Result<(), Error> {
    let template = include_str!("./templates/component/component_template.tsx")
        .replace("{component}", &pascal_name);

    let mut file = File::create(format!("{}/{}/{}.component.tsx", path, name, name))?;
    write!(file, "{}", template)?;

    Ok(())
}

fn create_index_file(path: &str, name: &String) -> Result<(), Error> {
    let template = include_str!("./templates/component/index_template.ts")
        .replace("{component}", name);

    let mut file = File::create(format!("{}/{}/index.ts", path, name))?;
    write!(file, "{}", template)?;

    Ok(())
}

fn create_entity_file(path: &str, name: &String, pascal_name: &String) -> Result<(), Error> {
    let template = include_str!("./templates/entity/entity_template.ts")
        .replace("{pascal-name}", &pascal_name);

    let mut file = File::create(format!("{}/{}.entity.ts", path, name))?;
    write!(file, "{}", template)?;

    Ok(())
}

fn create_model_file(path: &str, name: &String, pascal_name: &String) -> Result<(), Error> {
    let template = include_str!("./templates/model/model_template.ts")
        .replace("{pascal-name}", &pascal_name)
        .replace("{name}", &name);

    let mut file = File::create(format!("{}/{}.model.ts", path, name))?;
    write!(file, "{}", template)?;

    Ok(())
}

fn create_stories_file(path: &str, name: &String, pascal_name: &String) -> Result<(), Error> {
    let template = include_str!("./templates/component/stories_template.tsx")
        .replace("{pascal-name}", &pascal_name)
        .replace("{name}", &name);

    let mut file = File::create(format!("{}/{}/{}.stories.tsx", path, name, name))?;
    write!(file, "{}", template)?;

    Ok(())
}
