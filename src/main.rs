mod client;
mod dns;
mod args;

use colored::Colorize;

fn main() {
    // ---------------------------- args ----------------------------

    let args = args::parse();

    println!();
    println!("** Netflix 解锁检测小工具 v1.0.0 By {} **", "@vuhe".cyan());

    let client = match args.get("proxy") {
        None => client::create(),
        Some(proxy) => client::with_proxy(proxy)
    };

    // ---------------------------- dns ----------------------------

    let dns_status = dns::check();
    println!("{}", dns_status);
    if dns_status.is_not_available() {
        return;
    }

    // ---------------------------- content ----------------------------

    let res = client.verify(args.get("custom"));
    println!("{}", res);
}
