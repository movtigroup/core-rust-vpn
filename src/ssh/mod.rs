use async_trait::async_trait;
use anyhow::Result;
use serde::{Deserialize, Serialize};

pub mod libssh2_backend;
pub mod russh_backend;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SshConfig {
    pub username: String,
    pub password: Option<String>,
    pub private_key_path: Option<String>,
    pub server_addr: String,
    pub local_proxy_addr: String,
    pub remote_target_host: String,
    pub remote_target_port: u16,
}

#[async_trait]
pub trait SshEngine: Send + Sync {
    async fn start(&self) -> Result<()>;
}
