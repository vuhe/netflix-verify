use std::fmt::{Display, Formatter};
use {
    std::net::IpAddr::{V4, V6},
    colored::Colorize,
    dns_lookup::lookup_host,
    DnsStatus::{NotAvailable, OnlyIpv4, OnlyIpv6, All},
};

pub enum DnsStatus {
    NotAvailable,
    OnlyIpv4,
    OnlyIpv6,
    All,
}

impl DnsStatus {
    pub(super) fn parse() -> Self {
        const ADDRESS: &str = "netflix.com";
        let mut status = NotAvailable;
        if let Ok(it) = lookup_host(ADDRESS) {
            for i in it {
                match i {
                    V4(_) if status == NotAvailable => { status = OnlyIpv4 }
                    V4(_) if status == OnlyIpv6 => { status = All }
                    V6(_) if status == NotAvailable => { status = OnlyIpv6 }
                    V6(_) if status == OnlyIpv4 => { status = All }
                    _ => {}
                }
            }
        }
        status
    }

    pub fn is_not_available(&self) -> bool {
        match self {
            NotAvailable => true,
            _ => false
        }
    }
}

impl PartialEq for DnsStatus {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (NotAvailable, NotAvailable) => true,
            (OnlyIpv6, OnlyIpv6) => true,
            (OnlyIpv4, OnlyIpv4) => true,
            (All, All) => true,
            _ => false
        }
    }
}

impl Display for DnsStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            NotAvailable => write!(f, "{}", "网络地址解析错误, 找不到 IP 地址!".red()),
            OnlyIpv4 => write!(f, "{}", "您的网络支持 IPv4 访问 Netflix!".cyan()),
            OnlyIpv6 => write!(f, "{}", "您的网络支持 IPv6 访问 Netflix!".cyan()),
            All => write!(f, "{}", "您的网络同时支持 IPv4 和 IPv6 访问 Netflix!".green())
        }
    }
}
