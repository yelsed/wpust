use color_eyre::eyre::Result;
use url::Url;

pub fn extract_hostname(url_str: &str) -> Result<String> {
    let parsed = Url::parse(url_str)
        .or_else(|_| Url::parse(&format!("https://{}", url_str)))?;

    parsed.host_str()
        .ok_or_else(|| color_eyre::eyre::eyre!("No hostname found"))
        .map(|s| s.to_string())
}

pub fn extract_root_domain(hostname: &str) -> Result<String> {
    let parts: Vec<&str> = hostname.split('.').collect();
    if parts.len() >= 2 {
        Ok(format!("{}.{}", parts[parts.len() - 2], parts[parts.len() - 1]))
    } else {
        Ok(hostname.to_string())
    }
}
