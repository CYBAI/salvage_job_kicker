#![deny(warnings)]

#[macro_use]
extern crate log;
extern crate env_logger;
extern crate reqwest;
extern crate rusoto_core;
extern crate rusoto_sqs;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate salvage_job_kicker;

use salvage_job_kicker::config::Config;
use salvage_job_kicker::media_linkage_credential::MediaLinkageCredential;
use salvage_job_kicker::message::Message;
use rusoto_core::Region;
use rusoto_sqs::{SendMessageRequest,Sqs,SqsClient};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Response {
    media_linkage_credentials: Vec<MediaLinkageCredential>
}

fn main() {
    env_logger::init();

    info!("Hello");

    let config = Config::load_from_env();

    let endpoint = format!("{}/api/private/media_linkage_credentials", config.auth_url);

    let client = reqwest::Client::new();
    let mut res = client
        .get(&endpoint)
        .header(reqwest::header::Authorization(format!("Bearer {}", config.auth_master_token)))
        .send()
        .unwrap();

    let status = res.status();
    if !status.is_success() {
        error!("Got unexpected status code from auth: {}", status);
        std::process::exit(1);
    }

    let response: Response = res.json().unwrap();

    let creds = response.media_linkage_credentials;

    let sqs_client = SqsClient::new(Region::ApNortheast1);

    let len = creds.len();

    for cred in creds {
        let msg = Message::from_media_linkage_credential(cred, "Scrape:Candidacies");
        let msg_json = serde_json::to_string(&msg).unwrap();

        let req = SendMessageRequest {
            queue_url: config.sqs_queue_url.to_owned(),
            message_body: msg_json,
            .. SendMessageRequest::default()
        };
        sqs_client.send_message(req).sync().unwrap();
    }

    info!("Successfully sent {} message(s) to SQS", len);
    info!("Goodbye")
}
