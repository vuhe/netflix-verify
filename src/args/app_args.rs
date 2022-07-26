extern crate clap;

use clap::{App, arg, ArgMatches};

pub struct AppArgs {
    matches: ArgMatches,
}

impl AppArgs {
    pub(super) fn parse() -> Self {
        const HINT: &str = "A script used to determine whether your network \
        can watch native Netflix movies or not.";

        let matches = App::new("netflix-verify")
            .version("1.0.0")
            .author("vuhe <zhuhe202@qq.com>")
            .about(HINT)
            .args(vec![
                arg!(-c --custom <ID> "").required(false).default_value(""),
                // todo!("address and proxy args is not support now.")
                // arg!(-a --address <IP> "").required(false).default_value(""),
                // arg!(-p --proxy <ADDRESS> "").required(false).default_value(""),
            ])
            .get_matches();

        AppArgs { matches }
    }

    pub fn get(&self, key: &str) -> Option<String> {
        match self.matches.get_one::<String>(key) {
            Some(value) if !value.is_empty() => { Some(value.clone()) }
            _ => { None }
        }
    }
}
