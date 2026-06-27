use crate::ssh::{SshConfig, SshEngine, ProxyMode};
use crate::proxy;
use async_trait::async_trait;
use anyhow::{Result, Context, anyhow};
use russh::*;
use russh_keys::*;
use std::sync::Arc;
use tokio::net::TcpListener;
use tracing::{info, error};

pub struct RusshEngine {
    config: SshConfig,
}

impl RusshEngine {
    pub fn new(config: SshConfig) -> Self {
        Self { config }
    }
}

struct ClientHandler {}

#[async_trait]
impl client::Handler for ClientHandler {
    type Error = anyhow::Error;
    async fn check_server_key(self, _server_public_key: &key::PublicKey) -> Result<(Self, bool), Self::Error> {
        Ok((self, true))
    }
}

#[async_trait]
impl SshEngine for RusshEngine {
    async fn start(&self) -> Result<()> {
        info!("Starting Russh (Native) Engine for {}", self.config.server_addr);

        let config = Arc::new(client::Config::default());
        let sh = ClientHandler {};

        let mut session = client::connect(config, &self.config.server_addr, sh).await
            .context("Failed to connect to SSH server")?;

        let auth_res = if let Some(ref password) = self.config.password {
            session.authenticate_password(&self.config.username, password).await?
        } else {
            return Err(anyhow!("Password auth required for now"));
        };

        if !auth_res {
            return Err(anyhow!("Authentication failed"));
        }

        info!("🔒 Russh authenticated successfully (Mode: {:?})", self.config.mode);

        let listener = TcpListener::bind(&self.config.local_proxy_addr).await?;
        info!("🚀 Core (Russh) listening on {}...", self.config.local_proxy_addr);

        let session_handle = Arc::new(tokio::sync::Mutex::new(session));

        while let Ok((mut local_stream, _)) = listener.accept().await {
            let mode = self.config.mode;
            let target_host_config = self.config.remote_target_host.clone();
            let target_port_config = self.config.remote_target_port;
            let session_clone = Arc::clone(&session_handle);

            tokio::spawn(async move {
                let (target_host, target_port) = match mode {
                    ProxyMode::Direct => (target_host_config, target_port_config),
                    ProxyMode::Socks5 => {
                        match proxy::handle_socks5_handshake(&mut local_stream).await {
                            Ok(res) => res,
                            Err(e) => {
                                error!("SOCKS5 handshake failed: {}", e);
                                return;
                            }
                        }
                    }
                    ProxyMode::Http => {
                        match proxy::handle_http_handshake(&mut local_stream).await {
                            Ok(res) => res,
                            Err(e) => {
                                error!("HTTP handshake failed: {}", e);
                                return;
                            }
                        }
                    }
                };

                info!("Opening SSH channel to {}:{}", target_host, target_port);
                let channel = {
                    let session_locked = session_clone.lock().await;
                    match session_locked.channel_open_direct_tcpip(
                        &target_host,
                        target_port as u32,
                        "127.0.0.1",
                        12345
                    ).await {
                        Ok(ch) => ch,
                        Err(e) => {
                            error!("Failed to open channel to {}:{}: {}", target_host, target_port, e);
                            return;
                        }
                    }
                };

                let mut stream = channel.into_stream();
                if let Err(e) = tokio::io::copy_bidirectional(&mut local_stream, &mut stream).await {
                    error!("Tunnel error ({}:{}): {:?}", target_host, target_port, e);
                }
            });
        }
        Ok(())
    }
}
