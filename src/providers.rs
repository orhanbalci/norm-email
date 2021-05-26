use enumflags2::{bitflags, make_bitflags, BitFlags};
use std::collections::HashSet;
use std::fmt::Debug;

#[bitflags]
#[repr(u16)]
#[derive(Eq, PartialEq, Hash, Debug, Copy, Clone)]
pub enum Rules {
    DashAddressing,
    PlusAddressing,
    LocalPartAsHostName,
    StripPeriods,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Provider {
    pub rules: BitFlags<Rules>,
    pub mx_domains: HashSet<String>,
    pub name: String,
}

lazy_static! {
    pub static ref APPLE_PROVIDER: Provider = Provider {
        rules: Rules::PlusAddressing.into(),
        mx_domains: vec!["icloud.com".into()].into_iter().collect(),
        name: "APPLE".into()
    };
    pub static ref FAST_MAIL_PROVIDER: Provider = Provider {
        rules: Rules::PlusAddressing | Rules::LocalPartAsHostName,
        mx_domains: vec!["messagingengine.com".into()].into_iter().collect(),
        name: "FAST_MAIL".into()
    };
    pub static ref GOOGLE_PROVIDER: Provider = Provider {
        rules: Rules::PlusAddressing | Rules::StripPeriods,
        mx_domains: vec!["google.com".into()].into_iter().collect(),
        name: "GOOGLE".into()
    };
    pub static ref MICROSOFT_PROVIDER: Provider = Provider {
        rules: Rules::PlusAddressing.into(),
        mx_domains: vec!["outlook.com".into()].into_iter().collect(),
        name: "MICROSOFT".into()
    };
    pub static ref PROTON_MAIL_PROVIDER: Provider = Provider {
        rules: Rules::PlusAddressing.into(),
        mx_domains: vec!["protonmail.ch".into()].into_iter().collect(),
        name: "PROTONMAIL".into()
    };
    pub static ref RACKSPACE_PROVIDER: Provider = Provider {
        rules: Rules::PlusAddressing.into(),
        mx_domains: vec!["emailsrvr.com".into()].into_iter().collect(),
        name: "RACKSPACE".into()
    };
    pub static ref YAHOO_PROVIDER: Provider = Provider {
        rules: Rules::DashAddressing | Rules::StripPeriods,
        mx_domains: vec!["yahoodns.net".into()].into_iter().collect(),
        name: "YAHOO".into()
    };
    pub static ref YANDEX_PROVIDER: Provider = Provider {
        rules: Rules::PlusAddressing.into(),
        mx_domains: vec!["mx.yandex.net".into(), "yandex.ru".into()]
            .into_iter()
            .collect(),
        name: "YANDEX".into()
    };
    pub static ref ZOHOO_PROVIDER: Provider = Provider {
        rules: Rules::PlusAddressing.into(),
        mx_domains: vec!["zoho.com".into()].into_iter().collect(),
        name: "ZOHOO".into()
    };
    pub static ref PROVIDERS: Vec<&'static Provider> = vec![
        &APPLE_PROVIDER,
        &FAST_MAIL_PROVIDER,
        &GOOGLE_PROVIDER,
        &MICROSOFT_PROVIDER,
        &PROTON_MAIL_PROVIDER,
        &RACKSPACE_PROVIDER,
        &YAHOO_PROVIDER,
        &YANDEX_PROVIDER,
        &ZOHOO_PROVIDER
    ];
}
