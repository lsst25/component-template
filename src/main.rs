use std::env;
use std::fs::{self, File};
use std::io::prelude::*;

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
    if args.len() < 2 {
        println!("Please provide a component name.");
        return Ok(());
    }

    let component_name = &args[1];
    let pascal_component_name = to_pascal_case(component_name);

    fs::create_dir_all(component_name)?;

    let template = include_str!("./templates/index_template.ts")
        .replace("{component}", component_name);
    let mut file = File::create(format!("{}/index.ts", component_name))?;
    write!(file, "{}", template)?;

    let template = include_str!("./templates/component_template.tsx")
        .replace("{component}", &pascal_component_name);
    let mut file = File::create(format!("{}/{}.component.tsx", component_name, component_name))?;
    write!(file, "{}", template)?;

    println!("Component {} created.", pascal_component_name);

    Ok(())
}
