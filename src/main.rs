mod template_type;

use std::env;
use std::fs::{self, File};
use std::io::prelude::*;

use template_type::TemplateType;

fn to_pascal_case(name: &str) -> String {
    name.split('-')
        .map(|s| {
            let mut c = s.chars();
            match c.next() {
                None => String::new(),
                Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
            }
        })
        .collect::<String>()
}

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

            let template = include_str!("./templates/index_template.ts")
                .replace("{component}", name);
            let mut file = File::create(format!("{}/index.ts", name))?;
            write!(file, "{}", template)?;

            let template = include_str!("./templates/component_template.tsx")
                .replace("{component}", &pascal_name);
            let mut file = File::create(format!("{}/{}.component.tsx", name, name))?;
            write!(file, "{}", template)?;

            println!("Component {} created.", pascal_name);
        },

        TemplateType::Unknown => {
            println!("Unknown template: {}", args[1]);
            return Ok(());
        }
    }

    Ok(())
}
