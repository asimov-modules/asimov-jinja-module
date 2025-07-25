// This is free and unencumbered software released into the public domain.

#[cfg(not(feature = "std"))]
compile_error!("asimov-jinja-runner requires the 'std' feature");

use asimov_module::SysexitsError::{self, *};
use clap::Parser;
use clientele::StandardOptions;
use minijinja::{Environment, UndefinedBehavior};
use serde_json::Value;
use std::{error::Error, io::stdin, path::PathBuf};

/// asimov-jinja-runner
#[derive(Debug, Parser)]
#[command(arg_required_else_help = true)]
struct Options {
    #[clap(flatten)]
    flags: StandardOptions,

    /// The `.j2` template files to render.
    templates: Vec<PathBuf>,
}

pub fn main() -> Result<SysexitsError, Box<dyn Error>> {
    // Load environment variables from `.env`:
    asimov_module::dotenv().ok();

    // Expand wildcards and @argfiles:
    let args = asimov_module::args_os()?;

    // Parse command-line options:
    let options = Options::parse_from(args);

    // Handle the `--version` flag:
    if options.flags.version {
        println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
        return Ok(EX_OK);
    }

    // Handle the `--license` flag:
    if options.flags.license {
        print!("{}", include_str!("../../UNLICENSE"));
        return Ok(EX_OK);
    }

    // Configure logging & tracing:
    #[cfg(feature = "tracing")]
    asimov_module::init_tracing_subscriber(&options.flags).expect("failed to initialize logging");

    // Define the template environment:
    let mut template_env = Environment::new();
    template_env.set_undefined_behavior(UndefinedBehavior::Strict);

    // Load and compile the template:
    for template_path in &options.templates {
        let template_name = template_path.to_string_lossy().into_owned();
        let template_data = std::fs::read_to_string(template_path)?;
        template_env.add_template_owned(template_name, template_data)?;
    }
    let template =
        template_env.get_template(&(options.templates.first().unwrap().to_string_lossy()))?;

    // Read the input JSON from stdin:
    let input_json: Value = serde_json::from_reader(stdin())?;

    // Render the template from stdin to stdout:
    println!("{}", template.render(input_json)?);

    Ok(EX_OK)
}
