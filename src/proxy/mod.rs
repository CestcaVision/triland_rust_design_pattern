//! Proxy Pattern
//! Proxy pattern is a structural design pattern that lets you provide a substitute or placeholder for another object. A proxy controls access to the original object, allowing you to perform something either before or after the request gets through to the original object.
pub trait Server {
    fn request(&self, url: &str) -> String;
}
pub struct RealServer;

impl Server for RealServer {
    fn request(&self, url: &str) -> String {
        format!("This content is from real server：{}", url)
    }
}

pub struct ProxyServer {
    real_server: RealServer,
}

impl ProxyServer {
    fn new() -> Self {
        ProxyServer {
            real_server: RealServer,
        }
    }

    fn check_access(&self, url: &str) -> bool {
        // check access
        println!("Check access : {}", url);
        if url == "123" {
            true
        } else {
            false
        }
    }
}

impl Server for ProxyServer {
    fn request(&self, url: &str) -> String {
        if self.check_access(url) {
            self.real_server.request(url)
        } else {
            "访问被拒绝".to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_proxy_pattern_functionality() {
        let proxy_server = ProxyServer::new();
        assert_eq!(
            proxy_server.request("123"),
            "This content is from real server：123"
        );
        assert_eq!(proxy_server.request("456"), "访问被拒绝");
    }
}
