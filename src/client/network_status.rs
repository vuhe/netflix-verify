use {
    super::netflix_region::RegionCode,
    reqwest::{Result, header::HeaderMap, blocking::Response},
    NetworkStatus::{NetworkError, NotAvailable, Available},
};

pub(super) enum NetworkStatus {
    NetworkError(String),
    NotAvailable,
    Available(RegionCode),
}

impl NetworkStatus {
    pub fn is_available(&self) -> bool {
        match self {
            Available(_) => true,
            _ => false
        }
    }
}

pub(super) trait ToNetworkStatus {
    fn to_network_status(&self) -> NetworkStatus;
}

impl ToNetworkStatus for Result<Response> {
    fn to_network_status(&self) -> NetworkStatus {
        match self {
            Ok(res) => res.headers().to_network_status(),
            Err(e) => NetworkError(e.to_string())
        }
    }
}

impl ToNetworkStatus for HeaderMap {
    fn to_network_status(&self) -> NetworkStatus {
        // X-Robots-Tag 标签仅存在于 HTTP 200 响应的 html 页面
        // 如果之间访问没有地区码的地址出现此标识，说明此页面没有特别的地区码
        // 通常美国的网站会省略地区码，因此返回美国地区
        if let Some(robots) = self.get("X-Robots-Tag") {
            let tag = robots.to_str().unwrap();
            if tag == "index" {
                return Available(RegionCode::default());
            }
        }

        // Location 标签存在于 30x 重定向响应中
        // 直接检查要重定向的地址即可获取区域信息
        if let Some(region) = self.get("Location") {
            let str = String::from_utf8_lossy(region.as_bytes());

            // 例如：[http:][][www.netflix.com][hk]
            // 跳过前三个，取地区信息
            let region_str = str.split("/").skip(3).next();

            let region = match region_str {
                Some(code) => RegionCode::from(code),
                None => RegionCode::unknown()
            };
            return Available(region);
        }

        // 前面两个变量未检查成功的，判定不支持此地区
        NotAvailable
    }
}
