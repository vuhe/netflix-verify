use {
    std::fmt::{Display, Formatter},
    colored::Colorize,
    super::request_region::RegionCode,
};

pub enum NetflixStatus {
    NetworkError(String),
    // IpBanned,
    NotAvailable,
    Available(RegionCode, AvailableLevel),
}

pub enum AvailableLevel {
    Proxy,
    Custom,
    SelfMade,
    All,
}

impl Display for NetflixStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            NetflixStatus::NetworkError(msg) => {
                writeln!(f, "{}", "网络错误, 无法访问 Netflix!".red())?;
                write!(f, "Error Message: {}", msg)
            }
            // NetflixStatus::IpBanned => {
            //     write!(f, "{}", "您的 IP 地址被 Netflix 禁止访问!".red())
            // }
            NetflixStatus::NotAvailable => {
                write!(f, "{}", "Netflix 在此地区不提供服务!".red())
            }
            NetflixStatus::Available(region, level) => {
                let hint = match level {
                    AvailableLevel::Proxy => "您似乎通过代理访问 Netflix, 请调整网络后重试!".red(),
                    AvailableLevel::Custom => "您可以通过 Netflix 观看此影片!".green(),
                    AvailableLevel::SelfMade => "您仅可以访问 Netflix 的自制剧内容!".yellow(),
                    AvailableLevel::All => "您可以访问 Netflix 的全部内容!".green(),
                };
                writeln!(f, "{}", hint)?;
                write!(f, "{}{}", "Netflix 地区: ".cyan(), region)
            }
        }
    }
}
