mod app_args;

use app_args::AppArgs;

pub fn parse() -> AppArgs {
    AppArgs::parse()
}
