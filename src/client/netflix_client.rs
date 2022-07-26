use {
    hyper_tls::HttpsConnector,
    hyper::{Client, Body, Method, Request},
    hyper::client::HttpConnector,
    super::request_result::NetflixStatus,
    super::request_result::NetflixStatus::{NetworkError, NotAvailable, Available},
    super::request_result::AvailableLevel::Proxy,
    super::request_region::RegionCode,
};

pub struct NetflixClient {
    client: Client<HttpsConnector<HttpConnector>>,
}

impl Default for NetflixClient {
    fn default() -> Self {
        let client = Client::builder()
            .build::<_, Body>(HttpsConnector::new());
        NetflixClient { client }
    }
}

impl Clone for NetflixClient {
    fn clone(&self) -> Self {
        NetflixClient { client: self.client.clone() }
    }
}

impl NetflixClient {
    pub(super) async fn request_id(&self, id: String) -> NetflixStatus {
        const USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; WOW64) \
        AppleWebKit/537.36 (KHTML, like Gecko) Chrome/78.0.3904.108 Safari/537.36";
        let uri = format!("https://www.netflix.com/title/{}", id);

        let req = Request::builder()
            .method(Method::GET)
            .uri(uri)
            .header("USER-AGENT", USER_AGENT)
            .body(Body::default()).unwrap();

        let res = match self.client.request(req).await {
            Ok(res) => { res }
            Err(e) => { return NetworkError(e.to_string()); }
        };

        let headers = res.headers();

        if let Some(tag) = headers.get("X-Robots-Tag") {
            if let Ok(str) = tag.to_str() {
                if str == "index" {
                    return Available(RegionCode::default(), Proxy);
                }
            }
        }

        if let Some(region) = headers.get("Location") {
            let str = String::from_utf8_lossy(region.as_bytes());
            let region_str = str.split("/").into_iter()
                .filter(|it| !it.is_empty())
                .skip(2).next();
            let region = match region_str {
                Some(code) => RegionCode::from(code.to_string()),
                None => RegionCode::from(str.to_string())
            };
            return Available(region, Proxy);
        }

        NotAvailable
    }
}
