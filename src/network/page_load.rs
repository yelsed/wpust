use color_eyre::eyre::Result;
use crate::utils::extract_hostname;

pub fn page_load(site: String) -> Result<()> {
    let hostname = extract_hostname(&site)?;

    println!("Page load time for: {}\n", hostname);

    // TODO: Implement headless browser page load measurement

    println!("Status: Not yet implemented\n");

    Ok(())
}
