mod template_type;
mod utils;

use std::env;
use std::fs::{self, File};
use std::io::Error;
use std::io::prelude::*;

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

            fs::create_dir_all(name)?;

            create_index_file(name)?;
            create_component_file(name, &pascal_name)?;
            create_stories_file(name, &pascal_name)?;

            println!("Component {} created.", pascal_name);
        },

        TemplateType::Entity => {
            let name = &args[2];
            let pascal_name = to_pascal_case(name);

            create_entity_file(name, &pascal_name)?;

            println!("Entity {} created.", pascal_name);
        },

        TemplateType::Model => {
            let name = &args[2];
            let pascal_name = to_pascal_case(name);

            create_model_file(name, &pascal_name)?;

            println!("Model {} created.", pascal_name);
        },

        TemplateType::Unknown => {
            println!("Unknown template: {}", args[1]);
            return Ok(());
        }
    }

    Ok(())
}

fn create_component_file(name: &String, pascal_name: &String) -> Result<(), Error> {
    let template = include_str!("./templates/component/component_template.tsx")
        .replace("{component}", &pascal_name);

    let mut file = File::create(format!("{}/{}.component.tsx", name, name))?;
    write!(file, "{}", template)?;

    Ok(())
}

fn create_index_file(name: &String) -> Result<(), Error> {
    let template = include_str!("./templates/component/index_template.ts")
        .replace("{component}", name);

    let mut file = File::create(format!("{}/index.ts", name))?;
    write!(file, "{}", template)?;

    Ok(())
}

fn create_entity_file(name: &String, pascal_name: &String) -> Result<(), Error> {
    let template = include_str!("./templates/entity/entity_template.ts")
        .replace("{pascal-name}", &pascal_name);

    let mut file = File::create(format!("./{}.entity.ts", name))?;
    write!(file, "{}", template)?;

    Ok(())
}

fn create_model_file(name: &String, pascal_name: &String) -> Result<(), Error> {
    let template = include_str!("./templates/model/model_template.ts")
        .replace("{pascal-name}", &pascal_name)
        .replace("{name}", &name);

    let mut file = File::create(format!("./{}.model.ts", name))?;
    write!(file, "{}", template)?;

    Ok(())
}

fn create_stories_file(name: &String, pascal_name: &String) -> Result<(), Error> {
    let template = include_str!("./templates/component/stories_template.tsx")
        .replace("{pascal-name}", &pascal_name)
        .replace("{name}", &name);

    let mut file = File::create(format!("{}/{}.stories.tsx", name, name))?;
    write!(file, "{}", template)?;

    Ok(())
}
