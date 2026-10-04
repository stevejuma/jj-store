//! Protected HTTP readiness; redirects and non-local origins are never followed.
use crate::Result;
use std::{
    io::{Read, Write},
    net::{SocketAddr, TcpStream},
    time::Duration,
};

pub fn listening(line: &str, expected: SocketAddr) -> Result<()> {
    let event: serde_json::Value =
        serde_json::from_str(line).map_err(|e| format!("Invalid listening event: {e}"))?;
    if event["version"] != 1
        || event["event"] != "listening"
        || event["address"] != expected.to_string()
        || event["transport"] != "explicit-loopback"
    {
        return Err("Bundled service reported unexpected readiness".into());
    }
    Ok(())
}
pub fn bootstrap(address: SocketAddr) -> Result<()> {
    if !address.ip().is_loopback() || address.port() == 0 {
        return Err("Readiness requires concrete loopback".into());
    }
    let timeout = Duration::from_secs(5);
    let mut socket = TcpStream::connect_timeout(&address, timeout).map_err(|e| e.to_string())?;
    socket
        .set_read_timeout(Some(timeout))
        .map_err(|e| e.to_string())?;
    socket
        .set_write_timeout(Some(timeout))
        .map_err(|e| e.to_string())?;
    write!(socket, "GET /v1/bootstrap HTTP/1.1\r\nHost: {address}\r\nOrigin: http://{address}\r\nx-jj-store-protocol: 1\r\nSec-Fetch-Site: same-origin\r\nSec-Fetch-Mode: cors\r\nSec-Fetch-Dest: empty\r\nConnection: close\r\n\r\n").map_err(|e| e.to_string())?;
    let mut response = Vec::new();
    socket
        .take(1024 * 1024 + 1)
        .read_to_end(&mut response)
        .map_err(|e| e.to_string())?;
    validate_response(&response)
}
pub fn validate_response(response: &[u8]) -> Result<()> {
    if response.len() > 1024 * 1024 {
        return Err("Bootstrap response exceeds limit".into());
    }
    let end = response
        .windows(4)
        .position(|v| v == b"\r\n\r\n")
        .ok_or("Invalid bootstrap response")?;
    if end > 16 * 1024 {
        return Err("Bootstrap headers exceed limit".into());
    }
    let headers = std::str::from_utf8(&response[..end]).map_err(|_| "Invalid bootstrap headers")?;
    if !headers.starts_with("HTTP/1.1 200 ")
        || !headers
            .to_ascii_lowercase()
            .contains("\r\ncontent-type: application/json")
    {
        return Err("Local service bootstrap was denied or redirected".into());
    }
    let body: serde_json::Value = serde_json::from_slice(&response[end + 4..])
        .map_err(|e| format!("Invalid bootstrap JSON: {e}"))?;
    if body["version"] != 1
        || body["access_mode"] != "local_trusted"
        || body["storage"]["ready"] != true
    {
        return Err("Local embedded storage is not ready".into());
    }
    Ok(())
}

pub fn navigation_allowed(origin: &url::Url, target: &url::Url) -> bool {
    target.origin() == origin.origin()
        && target.username().is_empty()
        && target.password().is_none()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn readiness_rejects_redirects_foreign_modes_and_unready_storage() {
        let valid = b"HTTP/1.1 200 OK\r\ncontent-type: application/json\r\n\r\n{\"version\":1,\"access_mode\":\"local_trusted\",\"storage\":{\"ready\":true}}";
        assert!(validate_response(valid).is_ok());
        assert!(
            validate_response(
                &String::from_utf8_lossy(valid)
                    .replace("200 OK", "302 Found")
                    .into_bytes()
            )
            .is_err()
        );
        assert!(
            validate_response(
                &String::from_utf8_lossy(valid)
                    .replace("local_trusted", "authenticated")
                    .into_bytes()
            )
            .is_err()
        );
        assert!(
            validate_response(
                &String::from_utf8_lossy(valid)
                    .replace("true", "false")
                    .into_bytes()
            )
            .is_err()
        );
        assert!(validate_response(&vec![b'0'; 1024 * 1024 + 1]).is_err());
    }
    #[test]
    fn navigation_preserves_only_exact_managed_origin() {
        let origin = url::Url::parse("http://127.0.0.1:17440/").unwrap();
        for target in [
            "http://127.0.0.1:17440/#/repo",
            "http://127.0.0.1:17440/v1/bootstrap",
        ] {
            assert!(navigation_allowed(
                &origin,
                &url::Url::parse(target).unwrap()
            ));
        }
        for target in [
            "http://localhost:17440/",
            "http://127.0.0.1:17441/",
            "https://example.com/",
            "http://user@127.0.0.1:17440/",
        ] {
            assert!(!navigation_allowed(
                &origin,
                &url::Url::parse(target).unwrap()
            ));
        }
    }
}
