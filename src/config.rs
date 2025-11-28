use cloudflare::endpoints::dns::dns::{CreateDnsRecordParams, DnsContent};
use serde::{Deserialize, Serialize};
use std::fs;

/// Root of the configuration file.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Configuration {
    pub zone: ZoneConfiguration,
}

/// Configuration for a zone.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZoneConfiguration {
    pub domain: String,
    pub identifier: String,
    pub records: Vec<ZoneRecordConfiguration>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ZoneRecordKind {
    #[serde(rename = "A")]
    A,
    #[serde(rename = "AAAA")]
    AAAA,
    #[serde(rename = "CNAME")]
    CNAME,
    #[serde(rename = "TXT")]
    TXT,
    #[serde(rename = "MX")]
    MX,
    #[serde(rename = "NS")]
    NS,
    #[serde(rename = "SRV")]
    SRV,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZoneRecordConfiguration {
    pub name: String,
    pub kind: ZoneRecordKind,
    pub value: String,
    pub proxied: Option<bool>,
    pub ttl: Option<u32>,
    pub priority: Option<u16>,
}

impl ZoneRecordConfiguration {
    pub fn create_dns_content(&self) -> DnsContent {
        match self.kind {
            ZoneRecordKind::A => DnsContent::A {
                content: self.value.parse().unwrap(),
            },

            ZoneRecordKind::AAAA => DnsContent::AAAA {
                content: self.value.parse().unwrap(),
            },

            ZoneRecordKind::CNAME => DnsContent::CNAME {
                content: self.value.clone(),
            },

            ZoneRecordKind::TXT => DnsContent::TXT {
                content: self.value.clone(),
            },

            ZoneRecordKind::MX => DnsContent::MX {
                content: self.value.parse().unwrap(),
                priority: self.priority.unwrap(),
            },

            ZoneRecordKind::NS => DnsContent::NS {
                content: self.value.clone(),
            },

            ZoneRecordKind::SRV => DnsContent::SRV {
                content: self.value.parse().unwrap(),
            },
        }
    }

    pub fn create_dns_params(&self) -> CreateDnsRecordParams<'_> {
        CreateDnsRecordParams {
            ttl: self.ttl,
            priority: self.priority,
            proxied: self.proxied,
            name: &self.name,
            content: self.create_dns_content(),
        }
    }
}

pub fn read(path: String) -> Configuration {
    let config = fs::read(&path).expect("failed to read config file");
    toml::from_slice(&config).expect("failed to parse config file")
}
