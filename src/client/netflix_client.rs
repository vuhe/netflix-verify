use {
    hyper_tls::HttpsConnector,
    hyper::Client,
    hyper::client::HttpConnector,
    super::netflix_request::NetflixRequest,
    super::request_result::NetflixStatus,
    super::request_result::NetflixStatus::Available,
    super::request_result::AvailableLevel::{Custom, SelfMade, All},
};

pub struct NetflixClient {
    client: Client<HttpsConnector<HttpConnector>>,
}

impl NetflixClient {
    pub(super) fn new() -> Self {
        let connector = HttpsConnector::new();
        NetflixClient { client: Client::builder().build(connector) }
    }

    pub async fn verify(&self, id: Option<String>) -> NetflixStatus {
        match id {
            None => self.default_verify().await,
            Some(id) => self.custom_verify(id).await
        }
    }

    async fn default_verify(&self) -> NetflixStatus {
        let proxy_check = self.client.netflix_request("80018499".to_string());
        let self_made_check = self.client.netflix_request("80197526".to_string());
        let non_self_made_check = self.client.netflix_request("70143836".to_string());

        let mut result = proxy_check.await.unwrap();

        if let Available(region, _) = self_made_check.await.unwrap() {
            result = Available(region, SelfMade);
        }

        if let Available(region, _) = non_self_made_check.await.unwrap() {
            result = Available(region, All);
        }

        result
    }

    async fn custom_verify(&self, id: String) -> NetflixStatus {
        let proxy_check = self.client.netflix_request("80018499".to_string());
        let custom_check = self.client.netflix_request(id);

        let mut result = proxy_check.await.unwrap();

        if let Available(region, _) = custom_check.await.unwrap() {
            result = Available(region, Custom);
        }

        result
    }
}
