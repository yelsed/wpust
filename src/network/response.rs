use crate::utils::extract_hostname;
use color_eyre::eyre::Result;
use std::time::Instant;

pub fn response(site: String) -> Result<()> {
    let hostname = extract_hostname(&site)?;

    let start_time_headers = Instant::now();

    let headers_response = reqwest::blocking::get(format!("https://{}", hostname))
        .map_err(|e| color_eyre::eyre::eyre!("HTTP request failed: {}", e))?;

    let status = headers_response.status();

    let duration_headers = start_time_headers.elapsed();

    let start_time_text = Instant::now();

    let _text_response = headers_response.text().map_err(|e| color_eyre::eyre::eyre!("Failed to get text: {}", e))?;

    let duration_text = start_time_text.elapsed();

    println!("Response for: {}\n", hostname);

    println!("Status: {}\n", status);

    println!("Duration for headers response: {:.2?}\n", duration_headers);
    println!("Duration for text response: {:.2?}\n", duration_text);

    Ok(())
}
