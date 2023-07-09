mod template_type;
pub mod utils;

use std::env;
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
            template_type.create_files(name)?;

            println!("Component {} created.", pascal_name);
        },

        Template::Entity => {
            template_type.create_files(name)?;

            println!("Entity {} created.", pascal_name);
        },

        Template::Model => {
            template_type.create_files(name)?;

            println!("Model {} created.", pascal_name);
        },

        Template::Unknown => {
            println!("Unknown template: {}", args[1]);
            return Ok(());
        }
    }

    Ok(())
}
