mod template_type;
pub mod utils;

use std::env;
use template_type::TemplateType;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Please provide a template and a name.");
        return Ok(());
    }

    let name = &args[2];
    let template_type = TemplateType::from(args[1].as_str());

    template_type.create_files(name)?;

    Ok(())
}
