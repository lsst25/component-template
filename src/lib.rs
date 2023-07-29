mod template_type;
pub mod utils;

use template_type::TemplateBuilder;
use std::error::Error;

pub struct Config {
    pub template: String,
    pub name: String,
}

impl Config {
    pub fn build(
        mut args: impl Iterator<Item = String>,
    ) -> Result<Config, &'static str> {
        args.next();

        let template = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a template argument"),
        };

        let name = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a name argument"),
        };

        Ok(Config { template, name })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let builder = TemplateBuilder::new(&config.template, &config.name);
    builder.create_files()?;

    Ok(())
}