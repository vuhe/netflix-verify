mod client;
mod dns;
mod args;

use colored::Colorize;

#[tokio::main]
async fn main() {

    // ---------------------------- args ----------------------------

    let args = args::parse();
    println!("** Netflix 解锁检测小工具 v0.1.0 By {} **", "@vuhe".cyan());

    // ---------------------------- dns ----------------------------

    let dns_status = dns::check();
    println!("{}", dns_status);
    if dns_status.is_not_available() {
        return;
    }

    // ---------------------------- content ----------------------------

    let res = client::create()
        .verify(args.get("custom")).await;
    println!("{}", res);
}
