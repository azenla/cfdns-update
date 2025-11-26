use cloudflare::endpoints::dns::dns::{
    CreateDnsRecord, CreateDnsRecordParams, DeleteDnsRecord, DnsRecord, ListDnsRecords,
    ListDnsRecordsParams,
};
use cloudflare::framework::Environment;
use cloudflare::framework::auth::Credentials;
use cloudflare::framework::client::ClientConfig;
use cloudflare::framework::client::blocking_api::HttpApiClient;

const DNS_RECORD_PAGE_SIZE: u32 = 100;

pub fn create() -> HttpApiClient {
    let token = std::env::var("CF_TOKEN").expect("CF_TOKEN not set");
    let credentials = Credentials::UserAuthToken { token };
    HttpApiClient::new(
        credentials,
        ClientConfig::default(),
        Environment::Production,
    )
    .expect("failed to create api client")
}

pub fn dns_records(client: &HttpApiClient, zone_identifier: &str) -> Vec<DnsRecord> {
    let mut records = Vec::new();
    let mut page = 1;
    loop {
        let result = client
            .request(&ListDnsRecords {
                zone_identifier,
                params: ListDnsRecordsParams {
                    page: Some(page),
                    per_page: Some(DNS_RECORD_PAGE_SIZE),
                    ..Default::default()
                },
            })
            .expect("failed to fetch dns records")
            .result;
        let end = result.len() < DNS_RECORD_PAGE_SIZE as usize;
        records.extend(result);
        if end {
            break records;
        }
        page += 1;
    }
}

pub fn delete_dns_record(client: &HttpApiClient, zone_identifier: &str, record_identifier: &str) {
    client
        .request(&DeleteDnsRecord {
            zone_identifier,
            identifier: record_identifier,
        })
        .expect("failed to delete dns record");
}

pub fn add_dns_record(
    client: &HttpApiClient,
    zone_identifier: &str,
    params: CreateDnsRecordParams,
) {
    client
        .request(&CreateDnsRecord {
            zone_identifier,
            params,
        })
        .expect("failed to create dns record");
}
