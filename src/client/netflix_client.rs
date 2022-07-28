use {
    std::time::Duration,
    reqwest::{blocking::Client, redirect::Policy, Proxy as HttpProxy},
    super::netflix_request::NetflixRequest,
    super::verify_result::VerifyResult,
    super::unlock_status::UnlockStatus::{Proxy, Custom, SelfMade, All},
};

static USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; WOW64) AppleWebKit/537.36 \
(KHTML, like Gecko) Chrome/78.0.3904.108 Safari/537.36";

pub struct NetflixClient {
    client: Client,
}

impl NetflixClient {
    pub(super) fn new(proxy: Option<String>) -> Self {
        // 默认 USER_AGENT; 限制重定向; 默认5秒超时
        let client = Client::builder()
            .user_agent(USER_AGENT)
            .redirect(Policy::custom(|a| a.stop()))
            .timeout(Duration::from_secs(5));

        // 代理设置
        let client = match proxy {
            None => client,
            Some(proxy) => {
                let proxy = HttpProxy::all(proxy)
                    .expect("Proxy address error!");
                client.proxy(proxy)
            }
        };
        NetflixClient { client: client.build().unwrap() }
    }

    pub fn verify(&self, id: Option<String>) -> VerifyResult {
        match id {
            None => {
                let (proxy, self_made, all) = self.client.netflix();
                if all.is_available() {
                    VerifyResult::with(all, All)
                } else if self_made.is_available() {
                    VerifyResult::with(self_made, SelfMade)
                } else if proxy.is_available() {
                    VerifyResult::with(proxy, Proxy)
                } else {
                    VerifyResult::new(proxy)
                }
            }
            Some(id) => {
                let (proxy, custom) = self.client.netflix_id(id);
                if custom.is_available() {
                    VerifyResult::with(custom, Custom)
                } else if proxy.is_available() {
                    VerifyResult::with(proxy, Proxy)
                } else {
                    VerifyResult::new(proxy)
                }
            }
        }
    }
}
