use cloudflare::endpoints::dns::dns::{CreateDnsRecordParams, DnsContent, DnsRecord};

pub struct DnsRecordDelta<'record> {
    pub deleted: Vec<&'record DnsRecord>,
    pub added: Vec<&'record CreateDnsRecordParams<'record>>,
}

fn is_record_match(want: &CreateDnsRecordParams, have: &DnsRecord) -> bool {
    want.proxied.unwrap_or(false) == have.proxied
        && want.name == have.name
        && (want.ttl.is_none() || (want.ttl == Some(have.ttl)))
        && match &(&want.content, &have.content) {
            (DnsContent::A { content: left }, DnsContent::A { content: right }) => left == right,
            (DnsContent::AAAA { content: left }, DnsContent::AAAA { content: right }) => {
                left == right
            }
            (DnsContent::CNAME { content: left }, DnsContent::CNAME { content: right }) => {
                left == right
            }
            (DnsContent::TXT { content: left }, DnsContent::TXT { content: right }) => {
                left == right
            }
            (DnsContent::SRV { content: left }, DnsContent::SRV { content: right }) => {
                left == right
            }
            (
                DnsContent::MX {
                    content: left,
                    priority: left_priority,
                },
                DnsContent::MX {
                    content: right,
                    priority: right_priority,
                },
            ) => left == right && left_priority == right_priority,
            (DnsContent::NS { content: left }, DnsContent::NS { content: right }) => left == right,
            _ => false,
        }
}

pub fn delta_dns_records<'record>(
    want: &'record [CreateDnsRecordParams],
    have: &'record [DnsRecord],
) -> DnsRecordDelta<'record> {
    let deleted: Vec<&DnsRecord> = have
        .iter()
        .filter(|exists| !want.iter().any(|want| is_record_match(want, exists)))
        .collect();

    let added: Vec<&CreateDnsRecordParams> = want
        .iter()
        .filter(|want| !have.iter().any(|have| is_record_match(want, have)))
        .collect();

    DnsRecordDelta { deleted, added }
}

pub fn describe_content(content: &DnsContent) -> String {
    match content {
        DnsContent::A { content } => format!("A {}", content),
        DnsContent::AAAA { content } => format!("AAAA {}", content),
        DnsContent::CNAME { content } => format!("CNAME {}", content),
        DnsContent::NS { content } => format!("NS {}", content),
        DnsContent::MX { content, priority } => format!("MX {} {}", content, priority),
        DnsContent::TXT { content } => format!("TXT {}", content),
        DnsContent::SRV { content } => format!("SRV {}", content),
    }
}
