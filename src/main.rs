// This is free and unencumbered software released into the public domain.

#[cfg(feature = "std")]
fn main() -> Result<clientele::SysexitsError, Box<dyn std::error::Error>> {
    use clientele::SysexitsError::*;
    use minijinja::{Environment, UndefinedBehavior};
    use serde_json::Value;
    use std::io::stdin;

    // Load environment variables from `.env`:
    clientele::dotenv().ok();

    // Expand wildcards and @argfiles:
    let args = clientele::args_os()?;
    if args.len() < 2 {
        return Ok(EX_USAGE);
    }

    // Configure tracing & logging:
    #[cfg(feature = "tracing")]
    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .with_max_level(tracing_subscriber::filter::LevelFilter::WARN)
        .init();

    // Define the template environment:
    let mut template_env = Environment::new();
    template_env.set_undefined_behavior(UndefinedBehavior::Strict);

    // Load and compile the template:
    for template_path in args.iter().skip(1) {
        let template_name = template_path.to_string_lossy().into_owned();
        let template_data = std::fs::read_to_string(template_path)?;
        template_env.add_template_owned(template_name, template_data)?;
    }
    let template = template_env.get_template(&(args[1].to_string_lossy()))?;

    // Read the input JSON from stdin:
    let input_json: Value = serde_json::from_reader(stdin())?;

    // Render the template from stdin to stdout:
    println!("{}", template.render(input_json)?);

    Ok(EX_OK)
}

#[cfg(not(feature = "std"))]
fn main() {
    unimplemented!("asimov-jinja-runner requires the 'std' feature")
}
