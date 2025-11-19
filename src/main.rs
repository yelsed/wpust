use clap::{Parser, Subcommand};
use color_eyre::eyre::Result;
use url::Url;
use dns_lookup::lookup_addr;
use hickory_resolver::{
    config::{ResolverConfig, ResolverOpts},
    proto::rr::{Name, RecordType},
    Resolver,
};
use openssl::ssl::{SslConnector, SslMethod, SslVerifyMode};
use std::net::TcpStream;
// use crossterm::{ExecutableCommand, terminal};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Perma {
        site: String
    },
    Themes {
        site: String
    },
    Plugins {
        site: String
    },
    Siteinfo {
        site: String
    },
    Dns {
        site: String
    },
    Ip {
        site: String
    },
    Ssl {
        site: String
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let args = Args::parse();

    match args.command {
        Commands::Perma {site} => {
            let site_after_check = acccheck(site);
            perma(site_after_check)?;
        },
        Commands::Themes {site} => {
            let site_after_check = acccheck(site);
            themes(site_after_check)?;
        },
        Commands::Plugins {site} => {
            let site_after_check = acccheck(site);
            plugins(site_after_check)?;
        }
        Commands::Siteinfo {site} => {
            let site_after_check = acccheck(site);
            site_health(site_after_check)?;
        }
        Commands::Dns {site} => {
            dns(site)?;
        }
        Commands::Ip {site} => {
            ip(site)?;
        }
        Commands::Ssl {site} => {
            ssl(site)?;
        }
    }

    Ok(())
}

fn acccheck(site: String) -> String {
    let acceptation_basic_auth = "https://REDACTED:REDACTED@";
    let acceptation_string = "acc.";

    let url = if site.contains(acceptation_string) {
        acceptation_basic_auth.to_owned() + &site.to_string()
    } else {
        format!("https://{}", site)
    };

    url
}

fn perma(site: String) -> Result<()> {
    if site.is_empty() {
        return Err(color_eyre::eyre::eyre!("Site name may not be empty"));
    }

    let url = site + "/wp-admin/options-permalink.php";

    println!("opening: {}", url);

    open::that(url)?;
    Ok(())
}


fn themes(site: String) -> Result<()> {
    if site.is_empty() {
        return Err(color_eyre::eyre::eyre!("Site name may not be empty"));
    }
    let url = site + "/wp-admin/themes.php";

    println!("opening: {}", url);

    open::that(url)?;
    Ok(())
}

fn plugins(site: String) -> Result<()> {
    if site.is_empty() {
        return Err(color_eyre::eyre::eyre!("Site name may not be empty"));
    }
    let url = site + "/wp-admin/plugins.php";

    println!("opening: {}", url);

    open::that(url)?;
    Ok(())
}


fn site_health(site: String) -> Result<()> {
    if site.is_empty() {
        return Err(color_eyre::eyre::eyre!("Site name may not be empty"));
    }

    let url = site + "/wp-admin/site-health.php?tab=debug";

    println!("opening: {}", url);

    open::that(url)?;
    Ok(())
}

fn dns(url: String) -> Result<()> {
    use std::str::FromStr;

    let hostname = extract_hostname(&url)?;
    let name = Name::from_str(&hostname)
        .map_err(|e| color_eyre::eyre::eyre!("Invalid hostname: {}", e))?;

    println!("DNS lookup for: {}\n", hostname);

    let resolver = Resolver::new(ResolverConfig::default(), ResolverOpts::default())
        .map_err(|e| color_eyre::eyre::eyre!("Could not create resolver: {}", e))?;

    let record_types = vec![
        (RecordType::A, "A (IPv4)"),
        (RecordType::AAAA, "AAAA (IPv6)"),
        (RecordType::MX, "MX (Mail)"),
        (RecordType::TXT, "TXT"),
        (RecordType::NS, "NS (Name Servers)"),
        (RecordType::CNAME, "CNAME"),
        (RecordType::SOA, "SOA"),
    ];

    for (record_type, label) in record_types {
        lookup_dns_records(&resolver, &name, record_type, label)?;
    }

    let root_domain = extract_root_domain(&hostname)?;
    if root_domain != hostname {
        println!("--- Records for root domain: {} ---\n", root_domain);
        if let Ok(root_name) = Name::from_str(&root_domain) {
            let root_record_types = vec![
                (RecordType::MX, "MX (Mail)"),
                (RecordType::TXT, "TXT"),
                (RecordType::NS, "NS (Name Servers)"),
                (RecordType::SOA, "SOA"),
            ];

            for (record_type, label) in root_record_types {
                lookup_dns_records(&resolver, &root_name, record_type, label)?;
            }
        }
    }

    Ok(())
}

fn extract_root_domain(hostname: &str) -> Result<String> {
    let parts: Vec<&str> = hostname.split('.').collect();
    if parts.len() >= 2 {
        Ok(format!("{}.{}", parts[parts.len() - 2], parts[parts.len() - 1]))
    } else {
        Ok(hostname.to_string())
    }
}

fn ip(url: String) -> Result<()> {
    use dns_lookup::lookup_host;

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

fn lookup_dns_records(
    resolver: &Resolver,
    name: &Name,
    record_type: RecordType,
    label: &str,
) -> Result<()> {
    use hickory_resolver::proto::rr::RData;

    let response = match resolver.lookup(name.clone(), record_type) {
        Ok(response) => response,
        Err(_) => {
            return Ok(());
        }
    };

    let records = response.record_iter();
    let mut has_records = false;
    for record in records {
        if !has_records {
            println!("{} records:", label);
            has_records = true;
        }
        if let Some(rdata) = record.data() {
                match rdata {
                    RData::A(ipv4) => println!("  - {}", ipv4),
                    RData::AAAA(ipv6) => println!("  - {}", ipv6),
                    RData::MX(mx) => println!("  - {} (priority: {})", mx.exchange(), mx.preference()),
                    RData::TXT(txt) => {
                        let text: String = txt.iter()
                            .filter_map(|bytes| std::str::from_utf8(bytes).ok())
                            .collect();
                        println!("  - {}", text);
                    }
                    RData::NS(ns) => println!("  - {}", ns),
                    RData::CNAME(cname) => println!("  - {}", cname),
                    RData::SOA(soa) => {
                        println!("  - MNAME: {}", soa.mname());
                        println!("    RNAME: {}", soa.rname());
                        println!("    Serial: {}", soa.serial());
                        println!("    Refresh: {}", soa.refresh());
                        println!("    Retry: {}", soa.retry());
                        println!("    Expire: {}", soa.expire());
                        println!("    Minimum: {}", soa.minimum());
                    }
                    _ => println!("  - {:?}", rdata),
                }
        }
    }

    if has_records {
        println!();
    }

    Ok(())
}

fn extract_hostname(url_str: &str) -> Result<String> {
    let parsed = Url::parse(url_str)
        .or_else(|_| Url::parse(&format!("https://{}", url_str)))?;

    parsed.host_str()
        .ok_or_else(|| color_eyre::eyre::eyre!("No hostname found"))
        .map(|s| s.to_string())
}

fn ssl(site: String) -> Result<()> {
    let hostname = extract_hostname(&site)?;
    
    println!("SSL Certificate information for: {}\n", hostname);

    let mut connector = SslConnector::builder(SslMethod::tls())?;
    connector.set_verify(SslVerifyMode::NONE);
    let connector = connector.build();

    let stream = TcpStream::connect(format!("{}:443", hostname))
        .map_err(|e| color_eyre::eyre::eyre!("Failed to connect to {}:443: {}", hostname, e))?;

    let mut ssl_stream = connector.connect(&hostname, stream)
        .map_err(|e| color_eyre::eyre::eyre!("SSL handshake failed: {}", e))?;

    let certificate = ssl_stream.ssl()
        .peer_certificate()
        .ok_or_else(|| color_eyre::eyre::eyre!("No certificate found"))?;

    let subject = certificate.subject_name();
    let issuer = certificate.issuer_name();

    println!("Subject:");
    for entry in subject.entries() {
        let object_name = entry.object().nid().short_name().unwrap_or("UNKNOWN");
        let value = entry.data().as_utf8()
            .map_err(|e| color_eyre::eyre::eyre!("Failed to decode subject entry: {}", e))?
            .to_string();
        println!("  {}: {}", object_name, value);
    }

    println!("\nIssuer:");
    for entry in issuer.entries() {
        let object_name = entry.object().nid().short_name().unwrap_or("UNKNOWN");
        let value = entry.data().as_utf8()
            .map_err(|e| color_eyre::eyre::eyre!("Failed to decode issuer entry: {}", e))?
            .to_string();
        println!("  {}: {}", object_name, value);
    }

    let not_before = certificate.not_before();
    let not_after = certificate.not_after();
    
    println!("\nValidity:");
    println!("  Not Before: {}", not_before);
    println!("  Not After:  {}", not_after);

    let now = openssl::asn1::Asn1Time::days_from_now(0)
        .map_err(|e| color_eyre::eyre::eyre!("Failed to get current time: {}", e))?;
    
    let days_until_expiry = (not_after.diff(&now)?
        .days as i64)
        .max(0);
    
    if days_until_expiry == 0 {
        println!("  ⚠️  Certificate has expired!");
    } else if days_until_expiry <= 30 {
        println!("  ⚠️  Certificate expires in {} days", days_until_expiry);
    } else {
        println!("  ✓ Certificate is valid for {} more days", days_until_expiry);
    }

    let serial_number = certificate.serial_number();
    let serial_hex = serial_number.to_hex_str()
        .map_err(|e| color_eyre::eyre::eyre!("Failed to convert serial number: {}", e))?;
    
    println!("\nSerial Number:");
    println!("  {}", serial_hex);

    let fingerprint_sha256 = certificate.digest(openssl::hash::MessageDigest::sha256())
        .map_err(|e| color_eyre::eyre::eyre!("Failed to calculate fingerprint: {}", e))?;
    
    println!("\nFingerprint (SHA-256):");
    println!("  {}", hex::encode(fingerprint_sha256.as_ref()));

    Ok(())
}
