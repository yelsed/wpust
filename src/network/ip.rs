use color_eyre::eyre::Result;
use dns_lookup::{lookup_addr, lookup_host};
use crate::utils::extract_hostname;

pub fn ip(url: String) -> Result<()> {
    let hostname = extract_hostname(&url)?;

    println!("IP lookup for: {}\n", hostname);

    match lookup_host(&hostname) {
        Ok(ips) => {
            println!("IP addresses:");
            for ip in ips {
                println!("  - {}", ip);

                if let Ok(host) = lookup_addr(&ip) {
                    println!("    (reverse DNS: {})", host);
                }
            }
        }
        Err(e) => {
            return Err(color_eyre::eyre::eyre!("IP lookup failed: {}", e));
        }
    }

    Ok(())
}
