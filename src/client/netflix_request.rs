use {
    std::time::Duration,
    std::thread::{spawn, JoinHandle},
    reqwest::blocking::{Client, Response},
    reqwest::Result,
};

pub(super) trait NetflixRequest {
    fn netflix(&self, id: String) -> JoinHandle<Result<Response>>;
}

impl NetflixRequest for Client {
    fn netflix(&self, id: String) -> JoinHandle<Result<Response>> {
        const USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; WOW64) AppleWebKit/537.36 \
        (KHTML, like Gecko) Chrome/78.0.3904.108 Safari/537.36";
        let client = self.clone();
        let uri = format!("https://www.netflix.com/title/{}", id);
        spawn(move || {
            client.get(uri)
                .header("USER-AGENT", USER_AGENT)
                .timeout(Duration::from_secs(5))
                .send()
        })
    }
}
