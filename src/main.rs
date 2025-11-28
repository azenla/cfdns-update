use cloudflare::endpoints::dns::dns::CreateDnsRecordParams;
use cloudflare::endpoints::zones::zone::ZoneDetails;

pub mod client;
pub mod config;
pub mod delta;

fn main() {
    let change_control = std::env::var("CF_CHANGE_CONTROL").ok();
    let dummy = change_control.as_deref() == Some("dummy");
    let token = if !dummy {
        std::env::var("CF_API_TOKEN").expect("missing CF_API_TOKEN")
    } else {
        "dummy".to_string()
    };
    let config = std::env::var("CF_ZONE_CONFIG").expect("missing CF_ZONE_CONFIG");

    let commit = change_control.as_deref() == Some("commit");

    let config = config::read(config);
    let client = client::create(token);

    let zone_domain = if dummy {
        config.zone.domain.clone()
    } else {
        client
            .request(&ZoneDetails {
                identifier: &config.zone.identifier,
            })
            .expect("failed to get zone details")
            .result
            .name
    };

    if zone_domain != config.zone.domain {
        panic!("acquired zone details do not match configuration domain");
    }

    let current_records = if dummy {
        vec![]
    } else {
        client::dns_records(&client, &config.zone.identifier)
    };

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
            zone_domain,
            add.name,
            delta::describe_content(&add.content)
        );
    }

    for delete in &delta.deleted {
        println!(
            "[{}] will delete: {} {}",
            zone_domain,
            delete.name,
            delta::describe_content(&delete.content)
        );
    }

    if commit {
        for delete in delta.deleted {
            client::delete_dns_record(&client, &config.zone.identifier, &delete.id);
            println!(
                "[{}] deleted: {} {}",
                zone_domain,
                delete.name,
                delta::describe_content(&delete.content)
            );
        }

        for add in delta.added {
            client::add_dns_record(&client, &config.zone.identifier, add.clone());
            println!(
                "[{}] added: {} {}",
                zone_domain,
                add.name,
                delta::describe_content(&add.content)
            );
        }
    }
}
