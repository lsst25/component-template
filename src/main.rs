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

    let (template, name) = parse_config(&args);

    let builder = TemplateBuilder::new(template, name);
    builder.create_files()?;

    Ok(())
}

fn parse_config(args: &[String]) -> (&str, &str) {
    let template = &args[1];
    let name = &args[2];

    (template, name)
}
