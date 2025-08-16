use log::{debug, trace};

const MODRINTH_API_BASE_URL: &str = "https://api.modrinth.com/v2";

pub fn get_text<T: ToString>(api_endpoint: T) -> Result<String, Box<dyn std::error::Error>> {
    let full_url = MODRINTH_API_BASE_URL.to_string() + api_endpoint.to_string().as_str();
    debug!("Making GET request to {}", full_url);
    let text = reqwest::blocking::get(&full_url)?.text()?;
    trace!("Got text {} from {}", text, full_url);
    Ok(text)
}
