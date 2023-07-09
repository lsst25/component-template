mod template_type;
mod utils;

use std::env;
use std::fs::{self, File};
use std::io::Error;
use std::io::prelude::*;
use std::path::Path;

use template_type::Template;
use utils::to_pascal_case;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Please provide a template and a name.");
        return Ok(());
    }

    let name = &args[2];
    let pascal_name = to_pascal_case(name);

    let template_type = Template::from(args[1].as_str());

    match template_type {
        Template::Component => {
            let path = template_type.get_path();

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

        Template::Entity => {
            let path = template_type.get_path();

            if Path::exists(Path::new(format!("{}/{}.entity.ts", path, name).as_str())) {
                println!("Entity {} already exists.", pascal_name);
                return Ok(());
            }

            create_entity_file(path, name, &pascal_name)?;

            println!("Entity {} created.", pascal_name);
        },

        Template::Model => {
            let path = template_type.get_path();

            if Path::exists(Path::new(format!("{}/{}.model.ts", path, name).as_str())) {
                println!("Model {} already exists.", pascal_name);
                return Ok(());
            }

            create_model_file(path, name, &pascal_name)?;

            println!("Model {} created.", pascal_name);
        },

        Template::Unknown => {
            println!("Unknown template: {}", args[1]);
            return Ok(());
        }
    }

    Ok(())
}

fn create_component_file(path: &str, name: &String, pascal_name: &String) -> Result<(), Error> {
    let template = include_str!("./templates/component/component_template.tsx")
        .replace("{component}", &pascal_name);

    let mut file = File::create(format!("{path}/{name}/{name}.component.tsx"))?;
    write!(file, "{}", template)?;

    Ok(())
}

fn create_index_file(path: &str, name: &String) -> Result<(), Error> {
    let template = include_str!("./templates/component/index_template.ts")
        .replace("{component}", name);

    let mut file = File::create(format!("{path}/{name}/index.ts"))?;
    write!(file, "{}", template)?;

    Ok(())
}

fn create_entity_file(path: &str, name: &String, pascal_name: &String) -> Result<(), Error> {
    let template = include_str!("./templates/entity/entity_template.ts")
        .replace("{pascal-name}", &pascal_name);

    let mut file = File::create(format!("{path}/{name}.entity.ts"))?;
    write!(file, "{}", template)?;

    Ok(())
}

fn create_model_file(path: &str, name: &String, pascal_name: &String) -> Result<(), Error> {
    let template = include_str!("./templates/model/model_template.ts")
        .replace("{pascal-name}", &pascal_name)
        .replace("{name}", &name);

    let mut file = File::create(format!("{path}/{name}.model.ts"))?;
    write!(file, "{}", template)?;

    Ok(())
}

fn create_stories_file(path: &str, name: &String, pascal_name: &String) -> Result<(), Error> {
    let template = include_str!("./templates/component/stories_template.tsx")
        .replace("{pascal-name}", &pascal_name)
        .replace("{name}", &name);

    let mut file = File::create(format!("{path}/{name}/{name}.stories.tsx"))?;
    write!(file, "{}", template)?;

    Ok(())
}
