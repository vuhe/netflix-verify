use {
    tokio::spawn,
    tokio::task::JoinHandle,
    hyper::{Client, Body, Method, Request},
    hyper::client::connect::Connect,
    super::request_result::NetflixStatus,
    super::request_result::NetflixStatus::{NetworkError, NotAvailable, Available},
    super::request_result::AvailableLevel::Proxy,
    super::request_region::RegionCode,
};

pub(super) trait NetflixRequest {
    fn netflix_request(&self, id: String) -> JoinHandle<NetflixStatus>;
}

impl<T> NetflixRequest for Client<T> where T: Clone + Send + Sync + Connect + 'static {
    fn netflix_request(&self, id: String) -> JoinHandle<NetflixStatus> {
        let client = self.clone();
        spawn(async move { request_id(client, id).await })
    }
}

async fn request_id<T>(client: Client<T>, id: String) -> NetflixStatus
    where T: Clone + Send + Sync + Connect + 'static {

    let req = build_request(id);
    let res = match client.request(req).await {
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

        // 例如：[http:][][www.netflix.com][hk]
        // 跳过前三个，取地区信息
        let region_str = str.split("/")
            .into_iter().skip(3).next();

        let region = match region_str {
            Some(code) => RegionCode::from(code),
            None => RegionCode::unknown()
        };
        return Available(region, Proxy);
    }

    NotAvailable
}

fn build_request(id: String) -> Request<Body> {
    const USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; WOW64) \
        AppleWebKit/537.36 (KHTML, like Gecko) Chrome/78.0.3904.108 Safari/537.36";
    let uri = format!("https://www.netflix.com/title/{}", id);

    Request::builder()
        .method(Method::GET)
        .uri(uri)
        .header("USER-AGENT", USER_AGENT)
        .body(Body::default()).unwrap()
}
