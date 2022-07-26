mod dns_lookup_host;

pub use dns_lookup_host::DnsStatus;

pub fn check() -> DnsStatus {
    DnsStatus::parse()
}
