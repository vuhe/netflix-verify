mod netflix_client;
mod request_result;
mod request_region;

use {
    tokio::spawn,
    tokio::task::{JoinHandle, JoinError},
    netflix_client::NetflixClient,
    request_result::AvailableLevel::{SelfMade, Custom, All},
    request_result::NetflixStatus,
    request_result::NetflixStatus::Available,
};

pub async fn default_verify() -> Result<NetflixStatus, JoinError> {
    let client = NetflixClient::default();
    let proxy_check = request_str_id(&client, "80018499");
    let self_made_check = request_str_id(&client, "80197526");
    let non_self_made_check = request_str_id(&client, "70143836");

    let mut result = proxy_check.await?;

    if let Available(region, _) = self_made_check.await? {
        result = Available(region, SelfMade);
    }

    if let Available(region, _) = non_self_made_check.await? {
        result = Available(region, All);
    }

    Ok(result)
}

pub async fn custom_verify(id: String) -> Result<NetflixStatus, JoinError> {
    let client = NetflixClient::default();
    let proxy_check = request_str_id(&client, "80018499");
    let custom_check = request_string_id(&client, id);

    let mut result = proxy_check.await?;

    if let Available(region, _) = custom_check.await? {
        result = Available(region, Custom);
    }

    Ok(result)
}

fn request_str_id(client: &NetflixClient, id: &str) -> JoinHandle<NetflixStatus> {
    let client = client.clone();
    let id = String::from(id);
    spawn(async move { client.request_id(id).await })
}

fn request_string_id(client: &NetflixClient, id: String) -> JoinHandle<NetflixStatus> {
    let client = client.clone();
    spawn(async move { client.request_id(id).await })
}
