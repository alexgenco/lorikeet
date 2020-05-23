use reqwest::IntoUrl;
use serde::{Deserialize, Serialize};

use std::convert::From;

use crate::step::Step;

use reqwest;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StepResult {
    pub name: String,
    pub description: Option<String>,
    pub pass: bool,
    pub output: String,
    pub error: Option<String>,
    pub duration: f32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebHook {
    hostname: String,
    has_errors: bool,
    tests: Vec<StepResult>,
}

pub async fn submit_webhook<U: IntoUrl, I: Into<String>>(
    results: &Vec<StepResult>,
    url: U,
    hostname: I,
) -> Result<(), reqwest::Error> {
    let has_errors = results.iter().any(|result| result.pass == false);

    let payload = WebHook {
        hostname: hostname.into(),
        has_errors: has_errors,
        tests: results.clone(),
    };

    let client = reqwest::Client::new();

    let builder = client.post(url);

    let builder = builder.json(&payload);

    builder.send().await?;

    Ok(())
}

impl From<Step> for StepResult {
    fn from(step: Step) -> Self {
        let duration = step.get_duration_ms();
        let name = step.name;
        let description = step.description;

        let (pass, output, error) = match step.outcome {
            Some(outcome) => {
                let output = match step.do_output {
                    true => outcome.output.unwrap_or_default(),
                    false => String::new(),
                };

                (outcome.error.is_none(), output, outcome.error)
            }
            None => (false, String::new(), Some(String::from("Not finished"))),
        };

        StepResult {
            name,
            duration,
            description,
            pass,
            output,
            error,
        }
    }
}
