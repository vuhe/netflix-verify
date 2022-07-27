extern crate clap;

use clap::{App, arg, ArgMatches};

pub struct AppArgs {
    matches: ArgMatches,
}

impl AppArgs {
    pub(super) fn parse() -> Self {
        const HINT: &str = "A script used to test Netflix unlock level.";

        let matches = App::new("netflix-verify")
            .version("1.0.0")
            .author("vuhe <zhuhe202@qq.com>")
            .about(HINT)
            .args(vec![
                arg!(-c --custom <ID> "Custom Netflix id for test.").required(false),
                arg!(-p --proxy <ADDRESS> "Setting proxy address to use.").required(false),
                // arg!(-a --address <IP> "").required(false).default_value(""),
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
