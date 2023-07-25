mod template_type;
pub mod utils;

use std::env;
use template_type::TemplateBuilder;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Please provide a template and a name.");
        return Ok(());
    }

    let config = parse_config(&args);

    let builder = TemplateBuilder::new(&config.template, &config.name);
    builder.create_files()?;

    Ok(())
}

struct Config {
    template: String,
    name: String,
}

fn parse_config(args: &[String]) -> Config {
    let template = args[1].clone();
    let name = args[2].clone();

    Config { template, name }
}
