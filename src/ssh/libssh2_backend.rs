use crate::ssh::{SshConfig, SshEngine};
use async_trait::async_trait;
use anyhow::{Result, anyhow};
use ssh2::Session;
use std::sync::Arc;
use std::io::{Read, Write};
use tracing::{info, error, warn};

pub struct LibSsh2Engine {
    config: SshConfig,
}

impl LibSsh2Engine {
    pub fn new(config: SshConfig) -> Self {
        Self { config }
    }
}

#[async_trait]
impl SshEngine for LibSsh2Engine {
    async fn start(&self) -> Result<()> {
        info!("Starting LibSSH2 Engine for {}", self.config.server_addr);
        warn!("LibSSH2 backend is synchronous and may have performance limits under high concurrency.");

        let config = self.config.clone();

        tokio::task::spawn_blocking(move || {
            let tcp = std::net::TcpStream::connect(&config.server_addr)?;
            let mut sess = Session::new().map_err(|e| anyhow!("Failed to create session: {}", e))?;
            sess.set_tcp_stream(tcp);
            sess.handshake().map_err(|e| anyhow!("Handshake failed: {}", e))?;

            if let Some(pwd) = &config.password {
                sess.userauth_password(&config.username, pwd)
                    .map_err(|e| anyhow!("Auth failed: {}", e))?;
            } else {
                return Err(anyhow!("Password required for this backend currently"));
            }

            if !sess.authenticated() {
                return Err(anyhow!("Authentication failed"));
            }

            info!("🔒 LibSSH2 authenticated successfully");
            let sess = Arc::new(sess);

            let listener = std::net::TcpListener::bind(&config.local_proxy_addr)?;
            info!("🚀 Core (LibSSH2) listening on {}...", config.local_proxy_addr);

            for stream in listener.incoming() {
                let local_stream = match stream {
                    Ok(s) => s,
                    Err(e) => {
                        error!("Accept error: {}", e);
                        continue;
                    }
                };

                let sess_clone = Arc::clone(&sess);
                let target_host = config.remote_target_host.clone();
                let target_port = config.remote_target_port;

                std::thread::spawn(move || {
                    let mut channel = match sess_clone.channel_direct_tcpip(&target_host, target_port, None) {
                        Ok(ch) => ch,
                        Err(e) => {
                            error!("Failed to open channel: {}", e);
                            return;
                        }
                    };

                    let mut local_stream_r = local_stream.try_clone().expect("Failed to clone local stream");
                    let mut local_stream_w = local_stream;

                    sess_clone.set_blocking(false);
                    local_stream_r.set_nonblocking(true).expect("Failed to set nonblocking");

                    let mut buf_l = [0; 16384];
                    let mut buf_r = [0; 16384];

                    loop {
                        let mut active = false;

                        // Local -> Remote
                        match local_stream_r.read(&mut buf_l) {
                            Ok(0) => break,
                            Ok(n) => {
                                let _ = channel.write_all(&buf_l[..n]);
                                active = true;
                            }
                            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {}
                            Err(_) => break,
                        }

                        // Remote -> Local
                        match channel.read(&mut buf_r) {
                            Ok(0) => break,
                            Ok(n) => {
                                let _ = local_stream_w.write_all(&buf_r[..n]);
                                active = true;
                            }
                            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {}
                            Err(_) => break,
                        }

                        if !active {
                            // Yield to CPU to prevent 100% usage on idle tunnels
                            std::thread::sleep(std::time::Duration::from_millis(1));
                        }
                    }
                });
            }
            Ok(())
        }).await?
    }
}
