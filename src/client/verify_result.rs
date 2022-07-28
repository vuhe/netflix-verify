use {
    std::fmt::{Display, Formatter},
    colored::Colorize,
    super::{
        network_status::{
            NetworkStatus,
            NetworkStatus::{Available, NotAvailable, NetworkError},
        },
        unlock_status::{
            UnlockStatus,
            UnlockStatus::{Proxy, Custom, SelfMade, All},
        },
    },
};

pub struct VerifyResult {
    network: NetworkStatus,
    unlock: Option<UnlockStatus>,
}

impl VerifyResult {
    pub(super) fn new(network: NetworkStatus) -> Self {
        VerifyResult { network, unlock: None }
    }

    pub(super) fn with(network: NetworkStatus, unlock: UnlockStatus) -> Self {
        VerifyResult { network, unlock: Some(unlock) }
    }
}

impl Display for VerifyResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.network {
            NetworkError(msg) => {
                writeln!(f, "{}", "网络错误, 无法访问 Netflix!".red())?;
                write!(f, "Error Message: {}", msg)
            }
            NotAvailable => {
                write!(f, "{}", "Netflix 在此地区不提供服务!".red())
            }
            Available(region) => {
                let hint = match &self.unlock.as_ref().unwrap() {
                    Proxy => "您似乎通过代理访问 Netflix, 请调整网络后重试!".red(),
                    Custom => "您可以通过 Netflix 观看此影片!".green(),
                    SelfMade => "您仅可以访问 Netflix 的自制剧内容!".yellow(),
                    All => "您可以访问 Netflix 的全部内容!".green(),
                };
                writeln!(f, "{}", hint)?;
                write!(f, "{}{}", "Netflix 地区: ".cyan(), region)
            }
        }
    }
}
