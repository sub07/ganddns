mod config;
mod gandi;
mod ip;

use std::net::Ipv4Addr;
use std::path::Path;
use std::thread;

use anyhow::Result;

fn main() -> Result<()> {
    let config = config::load(Path::new("config.yaml"))?;
    let interval = config.fetch_rate;

    println!(
        "ganddns started — updating {} record(s) every {:?}",
        config.records.len(),
        interval
    );

    let mut cached_ip: Option<Ipv4Addr> = None;

    loop {
        match ip::fetch_public_ip() {
            Ok(current_ip) => {
                if cached_ip == Some(current_ip) {
                    println!("public IP unchanged ({current_ip}), skipping DNS checks");
                    println!("sleeping for {interval:?}...");
                    thread::sleep(interval);
                    continue;
                }

                println!("current public IP: {current_ip}");

                for record in &config.records {
                    let label = format!("{}.{}", record.name, record.domain);

                    match gandi::get_current_ip(&config.api_key, &record.domain, &record.name) {
                        Ok(Some(dns_ip)) if dns_ip == current_ip => {
                            println!("  {label} — already up to date ({dns_ip})");
                        }
                        Ok(dns_ip) => {
                            let old =
                                dns_ip.map_or_else(|| "none".to_string(), |ip| ip.to_string());
                            println!("  {label} — updating {old} -> {current_ip}");

                            if let Err(e) = gandi::update_record(
                                &config.api_key,
                                &record.domain,
                                &record.name,
                                current_ip,
                            ) {
                                eprintln!("  {label} — update failed: {e:#}");
                            } else {
                                println!("  {label} — updated successfully");
                            }
                        }
                        Err(e) => {
                            eprintln!("  {label} — failed to fetch current record: {e:#}");
                        }
                    }
                }

                cached_ip = Some(current_ip);
            }
            Err(e) => {
                eprintln!("failed to fetch public IP: {e:#}");
            }
        }

        println!("sleeping for {interval:?}...");
        thread::sleep(interval);
    }
}
