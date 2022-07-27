use {
    std::fmt::{Display, Formatter},
    colored::Colorize,
    reqwest::{StatusCode, Result, header::HeaderMap, blocking::Response},
    super::request_region::RegionCode,
    NetflixStatus::{NetworkError, NotAvailable, Available},
    AvailableLevel::{Proxy, Custom, SelfMade, All},
};

pub enum NetflixStatus {
    NetworkError(String),
    NotAvailable,
    Available(RegionCode, AvailableLevel),
}

pub enum AvailableLevel {
    Proxy,
    Custom,
    SelfMade,
    All,
}

pub(super) trait ToNetflixStatus {
    fn to_netflix_status(&self) -> NetflixStatus;
}

impl ToNetflixStatus for Result<Response> {
    fn to_netflix_status(&self) -> NetflixStatus {
        match self {
            Ok(res) => match res.status() {
                StatusCode::OK => res.headers().to_netflix_status(),
                _ => NotAvailable
            }
            Err(e) => NetworkError(e.to_string())
        }
    }
}

impl ToNetflixStatus for HeaderMap {
    fn to_netflix_status(&self) -> NetflixStatus {
        // 客户端发送请求时会自动响应 301 码，自动重定向至相应的地区
        // 之前 301 响应中的 Location 字段不再适用
        // 因此取 X-Originating-Url 字段检查
        if let Some(region) = self.get("X-Originating-Url") {
            let str = String::from_utf8_lossy(region.as_bytes());

            // 例如：[http:][][www.netflix.com][hk]
            // 跳过前三个，取地区信息
            let region_str = str.split("/").skip(3).next();

            let region = match region_str {
                Some(code) => RegionCode::from(code),
                None => RegionCode::unknown()
            };
            return Available(region, Proxy);
        }
        NotAvailable
    }
}

impl Display for NetflixStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            NetworkError(msg) => {
                writeln!(f, "{}", "网络错误, 无法访问 Netflix!".red())?;
                write!(f, "Error Message: {}", msg)
            }
            // IpBanned => {
            //     write!(f, "{}", "您的 IP 地址被 Netflix 禁止访问!".red())
            // }
            NotAvailable => {
                write!(f, "{}", "Netflix 在此地区不提供服务!".red())
            }
            Available(region, level) => {
                let hint = match level {
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
