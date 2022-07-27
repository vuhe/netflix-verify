mod netflix_request;
mod netflix_client;
mod request_result;
mod request_region;

pub fn create() -> netflix_client::NetflixClient {
    netflix_client::NetflixClient::new()
}
