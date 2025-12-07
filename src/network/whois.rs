use color_eyre::eyre::Result;
use crate::utils::extract_hostname;

pub fn whois(site: String) -> Result<()> {
    let hostname = extract_hostname(&site)?;

    println!("Whois lookup for: {}\n", hostname);

    // TODO: Implement whois lookup functionality
    // This is a placeholder - you'll need to implement the actual whois lookup
    println!("Whois functionality not yet implemented");

    Ok(())
}
