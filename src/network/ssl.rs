use color_eyre::eyre::Result;
use openssl::ssl::{SslConnector, SslMethod, SslVerifyMode};
use std::net::TcpStream;
use crate::utils::extract_hostname;

pub fn ssl(site: String) -> Result<()> {
    let hostname = extract_hostname(&site)?;

    println!("SSL Certificate information for: {}\n", hostname);

    let mut connector = SslConnector::builder(SslMethod::tls())?;
    connector.set_verify(SslVerifyMode::NONE);
    let connector = connector.build();

    let stream = TcpStream::connect(format!("{}:443", hostname))
        .map_err(|e| color_eyre::eyre::eyre!("Failed to connect to {}:443: {}", hostname, e))?;

    let ssl_stream = connector.connect(&hostname, stream)
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

    let diff_result = now.diff(&not_after)?;
    let days_until_expiry = (diff_result.days as i64).max(0);

    if days_until_expiry == 0 {
        println!("  ⚠️  Certificate has expired!");
    } else if days_until_expiry <= 30 {
        println!("  ⚠️  Certificate expires in {} days", days_until_expiry);
    } else {
        println!("  ✓ Certificate is valid for {} more days", days_until_expiry);
    }

    let serial_number = certificate.serial_number();
    let serial_bytes = serial_number.to_bn()
        .map_err(|e| color_eyre::eyre::eyre!("Failed to convert serial number: {}", e))?
        .to_vec();

    // Format as hex with colons between bytes (standard SSL certificate format)
    let serial_hex: String = serial_bytes.iter()
        .map(|b| format!("{:02X}", b))
        .collect::<Vec<_>>()
        .join(":");

    println!("\nSerial Number:");
    println!("  {}", serial_hex);

    let fingerprint_sha256 = certificate.digest(openssl::hash::MessageDigest::sha256())
        .map_err(|e| color_eyre::eyre::eyre!("Failed to calculate fingerprint: {}", e))?;

    println!("\nFingerprint (SHA-256):");
    println!("  {}", hex::encode(fingerprint_sha256.as_ref()));

    Ok(())
}
