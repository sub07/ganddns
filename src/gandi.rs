use std::net::Ipv4Addr;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

const API_BASE: &str = "https://api.gandi.net/v5/livedns/domains";

#[derive(Debug, Deserialize)]
struct RecordResponse {
    rrset_values: Vec<String>,
}

#[derive(Debug, Serialize)]
struct RecordUpdate {
    rrset_values: Vec<String>,
    rrset_ttl: u32,
}

fn record_url(domain: &str, name: &str) -> String {
    format!("{API_BASE}/{domain}/records/{name}/A")
}

pub fn get_current_ip(api_key: &str, domain: &str, name: &str) -> Result<Option<Ipv4Addr>> {
    let url = record_url(domain, name);
    let mut response = ureq::get(&url)
        .header("Authorization", &format!("Bearer {api_key}"))
        .call()
        .with_context(|| format!("failed to fetch DNS record for {name}.{domain}"))?;

    let record: RecordResponse = response
        .body_mut()
        .read_json()
        .with_context(|| format!("failed to parse DNS record response for {name}.{domain}"))?;

    let ip = record
        .rrset_values
        .first()
        .and_then(|v| v.parse::<Ipv4Addr>().ok());

    Ok(ip)
}

pub fn update_record(api_key: &str, domain: &str, name: &str, ip: Ipv4Addr) -> Result<()> {
    let url = record_url(domain, name);
    let body = RecordUpdate {
        rrset_values: vec![ip.to_string()],
        rrset_ttl: 300,
    };

    ureq::put(&url)
        .header("Authorization", &format!("Bearer {api_key}"))
        .send_json(&body)
        .with_context(|| format!("failed to update DNS record for {name}.{domain}"))?;

    Ok(())
}
