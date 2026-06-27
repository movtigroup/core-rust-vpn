use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use anyhow::{Result, anyhow};

pub async fn handle_socks5_handshake(stream: &mut TcpStream) -> Result<(String, u16)> {
    let mut buf = [0u8; 2];
    stream.read_exact(&mut buf).await?;

    if buf[0] != 0x05 {
        return Err(anyhow!("Unsupported SOCKS version: {}", buf[0]));
    }

    let nmethods = buf[1] as usize;
    let mut methods = vec![0u8; nmethods];
    stream.read_exact(&mut methods).await?;

    // No authentication for now
    stream.write_all(&[0x05, 0x00]).await?;

    let mut request = [0u8; 4];
    stream.read_exact(&mut request).await?;

    if request[0] != 0x05 || request[1] != 0x01 {
        return Err(anyhow!("Invalid SOCKS5 request"));
    }

    let addr_type = request[3];
    let host = match addr_type {
        0x01 => { // IPv4
            let mut ipv4 = [0u8; 4];
            stream.read_exact(&mut ipv4).await?;
            format!("{}.{}.{}.{}", ipv4[0], ipv4[1], ipv4[2], ipv4[3])
        }
        0x03 => { // Domain
            let len = stream.read_u8().await? as usize;
            let mut domain = vec![0u8; len];
            stream.read_exact(&mut domain).await?;
            String::from_utf8(domain)?
        }
        0x04 => { // IPv6
            let mut ipv6 = [0u8; 16];
            stream.read_exact(&mut ipv6).await?;
            // Simple IPv6 format
            "ipv6_placeholder".to_string()
        }
        _ => return Err(anyhow!("Unsupported address type")),
    };

    let port = stream.read_u16().await?;

    // Success response
    stream.write_all(&[0x05, 0x00, 0x00, 0x01, 0, 0, 0, 0, 0, 0]).await?;

    Ok((host, port))
}

pub async fn handle_http_handshake(stream: &mut TcpStream) -> Result<(String, u16)> {
    let mut buf = [0u8; 1024];
    let n = stream.read(&mut buf).await?;
    let request = String::from_utf8_lossy(&buf[..n]);

    if !request.starts_with("CONNECT") {
        return Err(anyhow!("Not an HTTP CONNECT request"));
    }

    let line = request.lines().next().ok_or_else(|| anyhow!("Empty request"))?;
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() < 2 {
        return Err(anyhow!("Malformed HTTP CONNECT"));
    }

    let target = parts[1];
    let host_port: Vec<&str> = target.split(':').collect();
    let host = host_port[0].to_string();
    let port = host_port.get(1).unwrap_or(&"80").parse()?;

    stream.write_all(b"HTTP/1.1 200 Connection Established\r\n\r\n").await?;

    Ok((host, port))
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::net::TcpListener;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};

    #[tokio::test]
    async fn test_http_handshake_logic() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();

        tokio::spawn(async move {
            let (mut socket, _) = listener.accept().await.unwrap();
            let res = handle_http_handshake(&mut socket).await;
            assert!(res.is_ok());
            let (host, port) = res.unwrap();
            assert_eq!(host, "google.com");
            assert_eq!(port, 443);
        });

        let mut client = tokio::net::TcpStream::connect(addr).await.unwrap();
        client.write_all(b"CONNECT google.com:443 HTTP/1.1\r\nHost: google.com:443\r\n\r\n").await.unwrap();
        let mut response = [0u8; 39];
        client.read_exact(&mut response).await.unwrap();
        assert!(String::from_utf8_lossy(&response).contains("200 Connection Established"));
    }
}
