mod ssh;
mod routing;
mod proxy;

use ssh::{SshConfig, SshEngine, ProxyMode};
use ssh::libssh2_backend::LibSsh2Engine;
use ssh::russh_backend::RusshEngine;
use tokio::signal;
use tracing::{info, error};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive(tracing::Level::INFO.into()))
        .init();

    info!("⚡ Advanced Blazing-Fast Rust SSH-VPN Core starting with SOCKS5/HTTP support...");

    let mode = match std::env::var("CORE_MODE").unwrap_or_default().as_str() {
        "socks5" => ProxyMode::Socks5,
        "http" => ProxyMode::Http,
        _ => ProxyMode::Direct,
    };

    let config = SshConfig {
        username: "root".to_string(),
        password: Some("your_secure_password".to_string()),
        private_key_path: None,
        server_addr: "127.0.0.1:22".to_string(),
        local_proxy_addr: "127.0.0.1:1080".to_string(),
        mode,
        remote_target_host: "127.0.0.1".to_string(),
        remote_target_port: 8080,
    };

    config.validate()?;

    let use_native = std::env::var("USE_NATIVE_SSH").unwrap_or_default() == "1" || mode != ProxyMode::Direct;

    let engine: Box<dyn SshEngine> = if use_native {
        info!("Using Russh (Native) backend for mode {:?}", mode);
        Box::new(RusshEngine::new(config))
    } else {
        info!("Using LibSSH2 (C-binding) backend for mode {:?}", mode);
        Box::new(LibSsh2Engine::new(config))
    };

    tokio::spawn(async move {
        if let Err(e) = engine.start().await {
            error!("Core runtime error: {:?}", e);
        }
    });

    signal::ctrl_c().await?;
    info!("🛑 Shutdown signal received. Cleaning up...");
    Ok(())
}
