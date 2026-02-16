use std::net::Ipv4Addr;

use anyhow::{bail, Context, Result};

const PRIMARY_URL: &str = "https://api4.ipify.org";
const FALLBACK_URL: &str = "https://ifconfig.me/ip";

fn fetch_from(url: &str) -> Result<Ipv4Addr> {
    let body = ureq::get(url)
        .call()
        .with_context(|| format!("HTTP request to {url} failed"))?
        .body_mut()
        .read_to_string()
        .with_context(|| format!("failed to read response from {url}"))?;

    let trimmed = body.trim();
    trimmed
        .parse::<Ipv4Addr>()
        .with_context(|| format!("response from {url} is not a valid IPv4 address: {trimmed:?}"))
}

pub fn fetch_public_ip() -> Result<Ipv4Addr> {
    match fetch_from(PRIMARY_URL) {
        Ok(ip) => Ok(ip),
        Err(primary_err) => {
            eprintln!("primary IP service failed: {primary_err:#}");
            eprintln!("trying fallback service...");
            match fetch_from(FALLBACK_URL) {
                Ok(ip) => Ok(ip),
                Err(fallback_err) => {
                    bail!(
                        "all IP services failed:\n  primary: {primary_err:#}\n  fallback: {fallback_err:#}"
                    );
                }
            }
        }
    }
}
