#[macro_use]
extern crate lazy_static;

use trust_dns_resolver::config::*;
use trust_dns_resolver::Resolver as MxResolver;

use anyhow::Result as AResult;

mod providers;

use providers::Provider;
use providers::PROVIDERS;

type MxRecord = (u16, String);

pub struct LookupResult {
    pub address: String,
    pub normalized_address: String,
    pub mailbox_provider: Option<String>,
    pub mx_records: Vec<MxRecord>,
}

pub struct Normalizer {
    resolver: trust_dns_resolver::Resolver,
}

use thiserror::Error;

#[derive(Error, Debug)]
pub enum NormalizerError {
    #[error("Invalid {email:?} contains more than one @")]
    InvalidEmailAt { email: String },
}

impl Default for Normalizer {
    fn default() -> Self {
        Self::new()
    }
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
        for (_priority, host) in mx_records {
            for &p in PROVIDERS.iter() {
                for domain in p.mx_domains.iter() {
                    let doted_domain = format!("{}{}", domain, '.');
                    if host.ends_with(&doted_domain) {
                        return Some(p);
                    }
                }
            }
        }
        None
    }

    pub fn normalize(&self, email_address: &str) -> AResult<LookupResult> {
        let (mut local, mut domain) = Normalizer::get_local_and_domain(email_address)?;
        let mx_records = self.mx_records(&domain)?;
        if let Some(provider) = Normalizer::lookup_provider(&mx_records[..]) {
            let normalized_address;
            if provider
                .rules
                .contains(providers::Rules::LocalPartAsHostName)
            {
                let (new_local_part, new_domain_part) =
                    Normalizer::local_part_as_hostname(&local, &domain);
                local = new_local_part;
                domain = new_domain_part;
                // normalized_address = format!("{}@{}", new_local_part, new_domain_part);
            }

            if provider.rules.contains(providers::Rules::DashAddressing) {
                let local_parts = local.split('-').collect::<Vec<_>>();
                if let Some(lp) = local_parts.first() {
                    local = lp.to_string();
                }
            }

            if provider.rules.contains(providers::Rules::PlusAddressing) {
                let local_parts = local.split('+').collect::<Vec<_>>();
                if let Some(lp) = local_parts.first() {
                    local = lp.to_string();
                }
            }

            if provider.rules.contains(providers::Rules::StripPeriods) {
                let new_local = local.replace(".", "");
                local = new_local;
            }

            normalized_address = format!("{}@{}", local, domain);
            Result::Ok(LookupResult {
                mailbox_provider: Some(provider.name.clone()),
                mx_records,
                address: email_address.into(),
                normalized_address,
            })
        } else {
            Result::Ok(LookupResult {
                mailbox_provider: None,
                mx_records,
                address: email_address.into(),
                normalized_address: email_address.into(),
            })
        }
    }

    pub fn get_local_and_domain(email_address: &str) -> Result<(String, String), NormalizerError> {
        let parts = email_address.split('@').collect::<Vec<_>>();
        if parts.len() != 2 {
            Err(NormalizerError::InvalidEmailAt {
                email: email_address.to_string(),
            })
        } else {
            Result::Ok((
                parts[0].to_string().to_lowercase(),
                parts[1].to_string().to_lowercase(),
            ))
        }
    }

    pub fn local_part_as_hostname(local_part: &str, domain_part: &str) -> (String, String) {
        let mut local_part_inner = local_part.to_string();
        let mut domain_part_inner = domain_part.to_string();
        let domain_splits = domain_part.split('.').collect::<Vec<_>>();
        if domain_splits.len() > 2 {
            local_part_inner = domain_splits[0].to_string();
            domain_part_inner = domain_splits[1..].join(".");
        }

        (local_part_inner, domain_part_inner)
    }
}

#[cfg(test)]
mod tests {
    use super::Normalizer;
    use uuid::Uuid;

    #[test]
    fn mx_records() {
        use super::Normalizer;
        let n = Normalizer::new();
        let records = n.mx_records("gmail.com").unwrap();
        // for (a, b) in records {
        //     println!("{} {}", a, b)
        // }
        assert!(true)
    }

    #[test]
    fn lookup_provider() {
        use super::Normalizer;
        let n = Normalizer::new();
        let provider =
            Normalizer::lookup_provider(&n.mx_records("gmail.com").unwrap()[..]).unwrap();
        // println!("{:?}", provider);
        assert!(true)
    }

    #[test]
    fn apple() {
        let local = Uuid::new_v4();
        let domain = "icloud.com";

        let normalizer = Normalizer::new();
        let result = normalizer
            .normalize(&format!("{}+test@{}", local, domain))
            .unwrap();
        assert_eq!(result.normalized_address, format!("{}@{}", local, domain));
    }

    #[test]
    fn gmail() {
        let local = Uuid::new_v4();
        let domain = "gmail.com";
        let normalizer = Normalizer::new();
        let result = normalizer
            .normalize(&format!("{}+test@{}", local, domain))
            .unwrap();
        println!("{:?}", result.mx_records);
        println!("{:?}", result.mailbox_provider);
        assert_eq!(result.normalized_address, format!("{}@{}", local, domain));
    }

    #[test]
    fn fastmail() {
        let local = Uuid::new_v4();
        let domain = "fastmail.com";
        let normalizer = Normalizer::new();
        let result = normalizer
            .normalize(&format!("{}+test@{}", local, domain))
            .unwrap();
        println!("{:?}", result.mx_records);
        println!("{:?}", result.mailbox_provider);
        assert_eq!(result.normalized_address, format!("{}@{}", local, domain));
    }

    #[test]
    fn fastmail_second() {
        let local = Uuid::new_v4();
        let domain_local = Uuid::new_v4();
        let domain = "fastmail.com";
        let normalizer = Normalizer::new();
        let result = normalizer
            .normalize(&format!("{}@{}.{}", local, domain_local, domain))
            .unwrap();
        println!("{:?}", result.mx_records);
        println!("{:?}", result.mailbox_provider);
        assert_eq!(
            result.normalized_address,
            format!("{}@{}", domain_local, domain)
        );
    }

    #[test]
    fn yahoo() {
        let normalizer = Normalizer::new();
        let result = normalizer.normalize("a.b.c.d+tag@yahoo.com").unwrap();
        assert_eq!("a.b.c.d+tag@yahoo.com", result.normalized_address);
    }

    #[test]
    fn yahoo_second() {
        let normalizer = Normalizer::new();
        let result = normalizer.normalize("a-b.c-tag@yahoo.ro").unwrap();
        assert_eq!("a@yahoo.ro", result.normalized_address);
    }

    #[test]
    fn microsoft_first() {
        let normalizer = Normalizer::new();
        let result = normalizer.normalize("a.b.c+tag@outlook.com").unwrap();
        assert_eq!("a.b.c@outlook.com", result.normalized_address);
    }

    #[test]
    fn microsoft_second() {
        let normalizer = Normalizer::new();
        let result = normalizer.normalize("a.b.c-tag@outlook.com").unwrap();
        assert_eq!("a.b.c-tag@outlook.com", result.normalized_address);
    }
}
