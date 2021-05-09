use std::collections::HashMap;

#[macro_use]
extern crate lazy_static;

mod providers;

type MxRecord = (u32, String);

pub struct LookupResult {
    address: String,
    normalized_address: String,
    mailbox_provider: Option<String>,
    mx_records: Vec<MxRecord>,
}

pub struct CachedItem {
    cached_at: i64,
    hits: u32,
    last_access: f64,
    ttl: u32,
    mx_records: Vec<MxRecord>,
}

type Cache = HashMap<String, CachedItem>;
pub struct Resolver {
    cache: Cache,
    cache_failures: bool,
    cache_limit: u32,
    failure_ttl: u32,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
