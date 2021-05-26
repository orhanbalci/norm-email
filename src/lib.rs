use std::collections::HashMap;

#[macro_use]
extern crate lazy_static;

use trust_dns_resolver::config::*;
use trust_dns_resolver::Resolver as MxResolver;

use anyhow::Result as AResult;

mod providers;

use providers::Provider;
use providers::GOOGLE_PROVIDER;
use providers::PROVIDERS;

type MxRecord = (u16, String);

pub struct LookupResult {
    address: String,
    normalized_address: String,
    mailbox_provider: Option<String>,
    mx_records: Vec<MxRecord>,
}

struct Normalizer {
    resolver: trust_dns_resolver::Resolver,
}

impl Normalizer {
    pub fn new() -> Normalizer {
        let opts = ResolverOpts {
            ndots: 0,
            ..ResolverOpts::default()
        };
        Normalizer {
            resolver: MxResolver::new(ResolverConfig::google(), opts).unwrap(),
        }
    }

    pub fn mx_records(&self, domain_name: &str) -> AResult<Vec<MxRecord>> {
        let mx_records = self.resolver.mx_lookup(domain_name)?;
        Ok(mx_records
            .iter()
            .map(|mx| (mx.preference(), mx.exchange().to_string()))
            .collect::<Vec<MxRecord>>())
    }

    pub fn lookup_provider(mx_records: &[MxRecord]) -> Option<&'static Provider> {
        for (priority, host) in mx_records {
            for &p in PROVIDERS.iter() {
                for domain in p.mx_domains.iter() {
                    let doted_domain = format!("{}{}", domain, '.');
                    if host.ends_with(&doted_domain) {
                        return Some(p);
                    } else {
                        println!("{} {}", host, domain)
                    }
                }
            }
        }
        None
    }

    pub fn normalize(&self, email_address: &str) -> LookupResult {
        let domain = Normalizer::get_domain(email_address);
        let mx_records = self.mx_records(domain.unwrap()).unwrap();
        if let Some(provider) = Normalizer::lookup_provider(&mx_records[..]) {
            let mut normalized_address = String::new();
            if provider
                .rules
                .contains(providers::Rules::LocalPartAsHostName)
            {
                normalized_address = email_address.into();
            }

            if provider.rules.contains(providers::Rules::DashAddressing) {
                normalized_address = email_address.into();
            }

            if provider.rules.contains(providers::Rules::PlusAddressing) {
                normalized_address = email_address.into();
            }

            if provider.rules.contains(providers::Rules::StripPeriods) {
                normalized_address = email_address.into();
            }

            LookupResult {
                mailbox_provider: Some(provider.name.clone()),
                mx_records: mx_records,
                address: email_address.into(),
                normalized_address: normalized_address,
            }
        } else {
            LookupResult {
                mailbox_provider: None,
                mx_records: mx_records,
                address: email_address.into(),
                normalized_address: email_address.into(),
            }
        }
    }

    pub fn get_domain(email_address: &str) -> Option<&str> {
        email_address.split('@').next()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn mx_records() {
        use super::Normalizer;
        let n = Normalizer::new();
        let records = n.mx_records("gmail.com").unwrap();
        for (a, b) in records {
            println!("{} {}", a, b)
        }
        assert!(true)
    }

    #[test]
    fn lookup_provider() {
        use super::Normalizer;
        let n = Normalizer::new();
        let provider =
            Normalizer::lookup_provider(&n.mx_records("gmail.com").unwrap()[..]).unwrap();
        println!("{:?}", provider);
        assert!(true)
    }
}
