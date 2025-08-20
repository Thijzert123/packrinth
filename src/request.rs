use log::{debug, trace};
use reqwest::blocking::Client;
use std::sync::OnceLock;

const MODRINTH_API_BASE_URL: &str = "https://api.modrinth.com/v2";

static CLIENT: OnceLock<Client> = OnceLock::new();
const USER_AGENT: &str = concat!(
    "Thijzert123",
    "/",
    "packrinth",
    "/",
    env!("CARGO_PKG_VERSION")
);

pub fn get_text<T: ToString>(api_endpoint: T) -> Result<String, Box<dyn std::error::Error>> {
    let client = CLIENT.get_or_init(|| {
        Client::builder()
            .user_agent(USER_AGENT)
            .build()
            .expect("Failed to build client")
    });

    let full_url = MODRINTH_API_BASE_URL.to_string() + api_endpoint.to_string().as_str();
    debug!("Making GET request to {}", full_url);
    let text = client.get(&full_url).send()?.text()?;
    trace!("Got text {} from {}", text, full_url);
    Ok(text)
}
