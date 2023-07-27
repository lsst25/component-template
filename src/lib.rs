mod template_type;
pub mod utils;

use template_type::TemplateBuilder;
use std::error::Error;

pub struct Config {
    pub template: String,
    pub name: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Please provide a template and a name.");
        }

        let template = args[1].clone();
        let name = args[2].clone();

        Ok(Config { template, name })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let builder = TemplateBuilder::new(&config.template, &config.name);
    builder.create_files()?;

    Ok(())
}