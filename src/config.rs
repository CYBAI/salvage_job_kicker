use std;

#[derive(Debug)]
pub struct Config {
    pub auth_master_token: String,
    pub auth_url: String,
    pub sqs_queue_url: String,
}

impl Config {
    pub fn load_from_env() -> Config {
        let auth_master_token = require_env_var("AUTH_MASTER_TOKEN");
        let auth_url = require_env_var("AUTH_ADDR");
        let sqs_queue_url = require_env_var("SQS_QUEUE_URL");

        Config {
            auth_master_token: auth_master_token,
            auth_url: auth_url,
            sqs_queue_url: sqs_queue_url,
        }
    }
}

fn require_env_var(key: &str) -> String {
    let value = std::env::var(key);

    value.unwrap_or_else(|_| {
        eprintln!("missing required environment variable: {}", key);
        std::process::exit(1)
    })
}
