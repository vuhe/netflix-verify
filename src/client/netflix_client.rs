use {
    reqwest::{blocking::Client, Proxy},
    super::netflix_request::NetflixRequest,
    super::request_result::{
        NetflixStatus, NetflixStatus::Available,
        AvailableLevel::{Custom, SelfMade, All},
        ToNetflixStatus,
    },
};

pub struct NetflixClient {
    client: Client,
}

impl NetflixClient {
    pub(super) fn new(proxy: Option<String>) -> Self {
        let client = Client::builder();
        let client = match proxy {
            None => client,
            Some(proxy) => {
                let proxy = Proxy::all(proxy)
                    .expect("Proxy address error!");
                client.proxy(proxy)
            }
        };
        NetflixClient { client: client.build().unwrap() }
    }

    pub fn verify(&self, id: Option<String>) -> NetflixStatus {
        match id {
            None => self.default_verify(),
            Some(id) => self.custom_verify(id)
        }
    }

    fn default_verify(&self) -> NetflixStatus {
        let proxy_check = self.client.netflix("80018499".to_string());
        let self_made_check = self.client.netflix("80197526".to_string());
        let non_self_made_check = self.client.netflix("70143836".to_string());

        let mut result = proxy_check.join().unwrap().to_netflix_status();
        match result {
            Available(_, _) => {}
            _ => { return result; }
        }

        let self_made = self_made_check.join().unwrap().to_netflix_status();
        if let Available(region, _) = self_made {
            result = Available(region, SelfMade);
        }

        let non_self_made = non_self_made_check.join().unwrap().to_netflix_status();
        if let Available(region, _) = non_self_made {
            result = Available(region, All);
        }

        result
    }

    fn custom_verify(&self, id: String) -> NetflixStatus {
        let proxy_check = self.client.netflix("80018499".to_string());
        let custom_check = self.client.netflix(id);

        let mut result = proxy_check.join().unwrap().to_netflix_status();
        match result {
            Available(_, _) => {}
            _ => { return result; }
        }

        let custom = custom_check.join().unwrap().to_netflix_status();
        if let Available(region, _) = custom {
            result = Available(region, Custom);
        }

        result
    }
}
