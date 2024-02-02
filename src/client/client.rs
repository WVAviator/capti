use std::fmt;

use lazy_static::lazy_static;

use crate::Args;

static USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

lazy_static! {
    static ref CLIENT: reqwest::Client = {
        reqwest::Client::builder()
            .user_agent(USER_AGENT)
            .build()
            .expect("Unable to establish request client.")
    };
}

pub struct ResponseOutput {
    pub http_status: u16,
    pub response_body: serde_json::Value,
    display_options: DisplayOptions,
}

impl ResponseOutput {
    pub async fn new(args: Args) -> Result<Self, reqwest::Error> {
        let response = CLIENT.get(&args.url).send().await?;

        let http_status = response.status().as_u16();
        let response_body = response.json().await.unwrap();
        let display_options = DisplayOptions::new(&args);

        Ok(ResponseOutput {
            http_status,
            response_body,
            display_options,
        })
    }
}

impl fmt::Display for ResponseOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.display_options.show_status {
            writeln!(f, "Status: {}", self.http_status)?;
        }

        writeln!(
            f,
            "{}",
            serde_json::to_string_pretty(&self.response_body).unwrap()
        )
    }
}

struct DisplayOptions {
    show_status: bool,
}

impl DisplayOptions {
    pub fn new(args: &Args) -> Self {
        DisplayOptions {
            show_status: args.status,
        }
    }
}
