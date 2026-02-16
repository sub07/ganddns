use std::fs;
use std::path::Path;
use std::time::Duration;

use anyhow::{Context, Result};
use duration_string::DurationString;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub api_key: String,
    #[serde(deserialize_with = "deserialize_duration")]
    pub fetch_rate: Duration,
    pub records: Vec<DnsRecord>,
}

fn deserialize_duration<'de, D>(deserializer: D) -> Result<Duration, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let ds: DurationString = s.parse().map_err(serde::de::Error::custom)?;
    Ok(ds.into())
}

#[derive(Debug, Deserialize)]
pub struct DnsRecord {
    pub domain: String,
    pub name: String,
}

pub fn load(path: &Path) -> Result<Config> {
    let contents = fs::read_to_string(path)
        .with_context(|| format!("failed to read config file: {}", path.display()))?;
    let config: Config =
        serde_yaml::from_str(&contents).context("failed to parse config YAML")?;
    Ok(config)
}
