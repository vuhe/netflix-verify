mod dns_lookup_host;

pub fn check() -> dns_lookup_host::DnsStatus {
    dns_lookup_host::DnsStatus::parse()
}
