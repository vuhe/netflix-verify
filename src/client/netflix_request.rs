use {
    std::thread::{spawn, JoinHandle},
    reqwest::blocking::{Client, Response},
    reqwest::Result,
    super::network_status::{NetworkStatus, ToNetworkStatus},
};

pub(super) trait NetflixRequest {
    fn netflix(&self) -> (NetworkStatus, NetworkStatus, NetworkStatus);
    fn netflix_id(&self, id: String) -> (NetworkStatus, NetworkStatus);
}

impl NetflixRequest for Client {
    fn netflix(&self) -> (NetworkStatus, NetworkStatus, NetworkStatus) {
        let proxy = netflix_str(self, "80018499");
        let self_made = netflix_str(self, "80197526");
        let all = netflix_str(self, "70143836");

        let proxy = proxy.join().unwrap().to_network_status();
        let self_made = self_made.join().unwrap().to_network_status();
        let all = all.join().unwrap().to_network_status();

        (proxy, self_made, all)
    }

    fn netflix_id(&self, id: String) -> (NetworkStatus, NetworkStatus) {
        let proxy = netflix_str(self, "80018499");
        let custom = netflix(self, id);

        let proxy = proxy.join().unwrap().to_network_status();
        let custom = custom.join().unwrap().to_network_status();

        (proxy, custom)
    }
}

fn netflix_str(client: &Client, id: &str) -> JoinHandle<Result<Response>> {
    netflix(client, id.to_string())
}

fn netflix(client: &Client, id: String) -> JoinHandle<Result<Response>> {
    let client = client.clone();
    let uri = format!("https://www.netflix.com/title/{}", id);
    spawn(move || { client.get(uri).send() })
}
