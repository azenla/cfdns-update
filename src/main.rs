use cloudflare::endpoints::dns::dns::CreateDnsRecordParams;
use cloudflare::endpoints::zones::zone::ZoneDetails;

pub mod client;
pub mod config;
pub mod delta;

fn main() {
    let config = config::read();
    let client = client::create();
    let zone_details = client
        .request(&ZoneDetails {
            identifier: &config.zone.identifier,
        })
        .expect("failed to get zone details");

    if zone_details.result.name != config.zone.domain {
        panic!("acquired zone details do not match configuration domain");
    }

    let current_records = client::dns_records(&client, &config.zone.identifier);
    let wanted: Vec<CreateDnsRecordParams> = config
        .zone
        .records
        .iter()
        .map(|record| record.create_dns_params())
        .collect();

    let delta = delta::delta_dns_records(&wanted, &current_records);

    for add in &delta.added {
        println!(
            "[{}] will add: {} {}",
            &zone_details.result.name,
            add.name,
            delta::describe_content(&add.content)
        );
    }

    for delete in &delta.deleted {
        println!(
            "[{}] will delete: {} {}",
            &zone_details.result.name,
            delete.name,
            delta::describe_content(&delete.content)
        );
    }

    let commit = std::env::var("CF_CHANGE_CONTROL").ok().as_deref() == Some("commit");
    if commit {
        for delete in delta.deleted {
            client::delete_dns_record(&client, &config.zone.identifier, &delete.id);
            println!(
                "[{}] deleted: {} {}",
                &zone_details.result.name,
                delete.name,
                delta::describe_content(&delete.content)
            );
        }

        for add in delta.added {
            client::add_dns_record(&client, &config.zone.identifier, add.clone());
            println!(
                "[{}] added: {} {}",
                &zone_details.result.name,
                add.name,
                delta::describe_content(&add.content)
            );
        }
    }
}
