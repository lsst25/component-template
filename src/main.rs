use std::{ env, process };
use std::error::Error;
use component_template::Config;

fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = component_template::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }

    Ok(())
}
