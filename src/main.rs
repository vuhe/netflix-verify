mod client;
mod dns;
mod args;

use {
    colored::Colorize,
    dns::DnsStatus::NotAvailable,
    client::default_verify,
    client::custom_verify,
};

#[tokio::main]
async fn main() {

    // ---------------------------- args ----------------------------

    let args = args::parse();
    println!("** Netflix 解锁检测小工具 v0.1.0 By {} **", "@vuhe".cyan());

    // ---------------------------- dns ----------------------------

    let dns_status = dns::check();
    println!("{}", dns_status);
    if dns_status == NotAvailable {
        return;
    }

    // ---------------------------- content ----------------------------

    let res = match args.get("custom") {
        None => default_verify().await,
        Some(id) => custom_verify(id).await
    };
    println!("{}", res.unwrap());
}
