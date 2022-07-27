mod netflix_request;
mod netflix_client;
mod request_result;
mod request_region;

pub fn create() -> netflix_client::NetflixClient {
    netflix_client::NetflixClient::new(None)
}

pub fn with_proxy(address: String) -> netflix_client::NetflixClient {
    netflix_client::NetflixClient::new(Some(address))
}
