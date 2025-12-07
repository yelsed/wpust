use color_eyre::eyre::Result;
use hickory_resolver::{
    config::{ResolverConfig, ResolverOpts},
    proto::rr::{Name, RecordType, RData},
    Resolver,
};
use std::str::FromStr;
use crate::utils::{extract_hostname, extract_root_domain};

pub fn dns(url: String) -> Result<()> {
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

fn lookup_dns_records(
    resolver: &Resolver,
    name: &Name,
    record_type: RecordType,
    label: &str,
) -> Result<()> {
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
