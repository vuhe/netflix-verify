mod app_args;

pub fn parse() -> app_args::AppArgs {
    app_args::AppArgs::parse()
}
