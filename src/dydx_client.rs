use self::types::ApiKeyCredentials;

pub use super::types;
use crate::modules::eth_private::EthPrivate;
use crate::modules::onboarding::Onboarding;
use crate::modules::private::Private;
use crate::modules::public::Public;

#[derive(Debug)]
pub struct ClientOptions {
    pub network_id: Option<usize>,
    pub api_timeout: Option<u64>,
    pub api_key_credentials: Option<ApiKeyCredentials>,
    pub stark_private_key: Option<String>,
    pub eth_private_key: Option<String>,
}

#[readonly::make]
#[derive(Debug, Clone)]
pub struct DydxClient {
    #[readonly]
    pub api_timeout: Option<u64>,
    pub public: Public,
    pub private: Option<Private>,
    pub eth_private: Option<EthPrivate>,
    pub onboarding: Option<Onboarding>,
}

impl DydxClient {
    pub fn new(host: &str, _options: ClientOptions) -> DydxClient {
        let network_id = _options.network_id.unwrap_or(1);
        let api_timeout = _options.api_timeout.unwrap_or(10);
        DydxClient {
            api_timeout: None,

            public: Public::new(host, api_timeout),
            private: match _options.api_key_credentials {
                Some(v) => Some(Private::new(
                    host,
                    network_id,
                    api_timeout,
                    v,
                    _options.stark_private_key.as_deref(),
                )),
                None => None,
            },
            eth_private: match _options.eth_private_key {
                Some(ref v) => Some(EthPrivate::new(host, network_id, api_timeout, v.as_str())),
                None => None,
            },
            onboarding: match _options.eth_private_key {
                Some(ref r) => Some(Onboarding::new(host, network_id, api_timeout, r.as_str())),
                None => None,
            },
        }
    }
}
