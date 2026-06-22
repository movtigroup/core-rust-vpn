use crate::ssh::{SshConfig, SshEngine};
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
        // SECURITY NOTE: In production, you MUST verify the server's public key
        // to prevent Man-In-The-Middle attacks. For this core, we accept it for ease of use.
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

        info!("🔒 Russh authenticated successfully");

        let listener = TcpListener::bind(&self.config.local_proxy_addr).await?;
        info!("🚀 Core (Russh) listening on {}...", self.config.local_proxy_addr);

        while let Ok((mut local_stream, _)) = listener.accept().await {
            let channel = match session.channel_open_direct_tcpip(
                &self.config.remote_target_host,
                self.config.remote_target_port as u32,
                "127.0.0.1",
                12345
            ).await {
                Ok(ch) => ch,
                Err(e) => {
                    error!("Failed to open channel: {}", e);
                    continue;
                }
            };

            tokio::spawn(async move {
                let mut stream = channel.into_stream();
                if let Err(e) = tokio::io::copy_bidirectional(&mut local_stream, &mut stream).await {
                    error!("Tunnel error: {:?}", e);
                }
            });
        }
        Ok(())
    }
}
