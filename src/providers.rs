use std::collections::HashSet;

#[derive(Eq, PartialEq, Hash)]
pub enum Rules {
    DashAddressing,
    PlusAddressing,
    LocalPartAsHostName,
    StripPeriods,
}

pub struct Provider {
    pub rules: HashSet<Rules>,
    pub mx_domains: HashSet<String>,
}

lazy_static! {
    static ref APPLE_PROVIDER: Provider = Provider {
        rules: vec![Rules::PlusAddressing].into_iter().collect(),
        mx_domains: vec!["icloud.com".into()].into_iter().collect()
    };
    static ref FAST_MAIL_PROVIDER: Provider = Provider {
        rules: vec![Rules::PlusAddressing, Rules::LocalPartAsHostName]
            .into_iter()
            .collect(),
        mx_domains: vec!["messagingengine.com".into()].into_iter().collect(),
    };
    static ref GOOGLE_PROVIDER: Provider = Provider {
        rules: vec![Rules::PlusAddressing, Rules::StripPeriods]
            .into_iter()
            .collect(),
        mx_domains: vec!["google.com".into()].into_iter().collect(),
    };
    static ref MICROSOFT_PROVIDER: Provider = Provider {
        rules: vec![Rules::PlusAddressing].into_iter().collect(),
        mx_domains: vec!["outlook.com".into()].into_iter().collect(),
    };
    static ref PROTON_MAIL_PROVIDER: Provider = Provider {
        rules: vec![Rules::PlusAddressing].into_iter().collect(),
        mx_domains: vec!["protonmail.ch".into()].into_iter().collect(),
    };
    static ref RACKSPACE_PROVIDER: Provider = Provider {
        rules: vec![Rules::PlusAddressing].into_iter().collect(),
        mx_domains: vec!["emailsrvr.com".into()].into_iter().collect(),
    };
    static ref YAHOO_PROVIDER: Provider = Provider {
        rules: vec![Rules::DashAddressing, Rules::StripPeriods]
            .into_iter()
            .collect(),
        mx_domains: vec!["yahoodns.net".into()].into_iter().collect(),
    };
    static ref YANDEX_PROVIDER: Provider = Provider {
        rules: vec![Rules::PlusAddressing].into_iter().collect(),
        mx_domains: vec!["mx.yandex.net".into(), "yandex.ru".into()]
            .into_iter()
            .collect(),
    };
    static ref ZOHOO_PROVIDER: Provider = Provider {
        rules: vec![Rules::PlusAddressing].into_iter().collect(),
        mx_domains: vec!["zoho.com".into()].into_iter().collect(),
    };
    static ref PROVIDERS: Vec<&'static Provider> = vec![
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
