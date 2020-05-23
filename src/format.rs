use colored::*;
use crate::submitter::StepResult;
use std::str::FromStr;
use std::io::Error;

#[derive(Debug)]
pub enum Format {
    Quiet,
    Yaml,
    Json,
    Wide,
}

impl Format {
    pub fn on_result(&self, result: &StepResult, colors: &Option<bool>) -> Result<(), Error> {
        match self {
            Format::Quiet => Ok(()),
            Format::Yaml => print_yaml(result, colors),
            Format::Json => print_json(result, colors),
            Format::Wide => print_wide(result, colors),
        }
    }
}

impl FromStr for Format {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "yaml" | "yml" => Ok(Format::Yaml),
            "none" | "quiet" => Ok(Format::Quiet),
            "json" => Ok(Format::Json),
            "wide" => Ok(Format::Wide),
            _ => Err(s.into())
        }
    }
}

fn print_yaml(result: &StepResult, opt_colours: &Option<bool>) -> Result<(), Error> {
    let mut message = format!("- name: {}\n", result.name);
    let colours = opt_colours.unwrap_or_else(|| atty::is(atty::Stream::Stdout));

    if let Some(ref description) = result.description {
        message.push_str(&format!("  description: {}\n", description))
    }

    message.push_str(&format!("  pass: {}\n", result.pass));

    if result.output != "" {
        if result.output.contains("\n") {
            message.push_str(&format!(
                "  output: |\n    {}\n",
                result.output.replace("\n", "\n    ")
            ));
        } else {
            message.push_str(&format!("  output: {}\n", result.output));
        }
    }

    if let Some(ref error) = result.error {
        message.push_str(&format!("  error: {}\n", error));
    }

    message.push_str(&format!("  duration: {}ms\n", result.duration));
    print_message(&message, &colours, result);

    Ok(())
}

fn print_json(result: &StepResult, opt_colours: &Option<bool>) -> Result<(), Error> {
    let message = serde_json::to_string(result)?;
    let colours = opt_colours.unwrap_or(false);
    print_message(&message, &colours, result);

    Ok(())
}

fn print_wide(result: &StepResult, opt_colours: &Option<bool>) -> Result<(), Error> {
    let message = format!(
        "name={} description={:?} pass={} output={:?} error={:?} duration={}ms",
        result.name, result.description, result.pass, result.output, result.error, result.duration
    );

    let colours = opt_colours.unwrap_or_else(|| atty::is(atty::Stream::Stdout));
    print_message(&message, &colours, result);

    Ok(())
}

fn print_message(message: &str, colours: &bool, result: &StepResult) {
    if *colours {
        if result.pass {
            println!("{}", message.green().bold());
        } else {
            println!("{}", message.red().bold());
        }
    } else {
        println!("{}", message);
    }
}
