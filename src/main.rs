mod template_type;
pub mod utils;

use std::{ env, process };
use std::error::Error;
use template_type::TemplateBuilder;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }

    Ok(())
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let builder = TemplateBuilder::new(&config.template, &config.name);
    builder.create_files()?;

    Ok(())
}

struct Config {
    template: String,
    name: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Please provide a template and a name.");
        }

        let template = args[1].clone();
        let name = args[2].clone();

        Ok(Config { template, name })
    }
}
