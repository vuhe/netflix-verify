mod netflix_request;
mod netflix_client;
mod netflix_region;
mod network_status;
mod unlock_status;
mod verify_result;

pub fn create() -> netflix_client::NetflixClient {
    netflix_client::NetflixClient::new(None)
}

pub fn with_proxy(address: String) -> netflix_client::NetflixClient {
    netflix_client::NetflixClient::new(Some(address))
}
