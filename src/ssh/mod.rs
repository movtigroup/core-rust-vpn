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

impl SshConfig {
    pub fn validate(&self) -> Result<()> {
        if self.username.is_empty() {
            return Err(anyhow::anyhow!("Username cannot be empty"));
        }
        if self.server_addr.is_empty() {
            return Err(anyhow::anyhow!("Server address cannot be empty"));
        }
        if !self.server_addr.contains(':') {
            return Err(anyhow::anyhow!("Server address must include port (e.g., host:22)"));
        }
        Ok(())
    }
}

#[async_trait]
pub trait SshEngine: Send + Sync {
    async fn start(&self) -> Result<()>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_validation() {
        let mut config = SshConfig {
            username: "".to_string(),
            password: None,
            private_key_path: None,
            server_addr: "127.0.0.1:22".to_string(),
            local_proxy_addr: "127.0.0.1:1080".to_string(),
            remote_target_host: "127.0.0.1".to_string(),
            remote_target_port: 80,
        };
        assert!(config.validate().is_err());

        config.username = "root".to_string();
        assert!(config.validate().is_ok());

        config.server_addr = "127.0.0.1".to_string();
        assert!(config.validate().is_err());
    }
}
