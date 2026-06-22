mod ssh;
mod routing;

use ssh::{SshConfig, SshEngine};
use ssh::libssh2_backend::LibSsh2Engine;
use ssh::russh_backend::RusshEngine;
use tokio::signal;
use tracing::{info, error};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Setup logging
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive(tracing::Level::INFO.into()))
        .init();

    info!("⚡ Advanced Blazing-Fast Rust SSH-VPN Core starting...");

    let config = SshConfig {
        username: "root".to_string(),
        password: Some("your_secure_password".to_string()),
        private_key_path: None,
        server_addr: "127.0.0.1:22".to_string(),
        local_proxy_addr: "127.0.0.1:1080".to_string(),
        remote_target_host: "127.0.0.1".to_string(),
        remote_target_port: 8080,
    };

    // Example of using the native Rust engine for better stability
    let use_native = std::env::var("USE_NATIVE_SSH").unwrap_or_default() == "1";

    let engine: Box<dyn SshEngine> = if use_native {
        info!("Using Russh (Native) backend");
        Box::new(RusshEngine::new(config))
    } else {
        info!("Using LibSSH2 (C-binding) backend");
        Box::new(LibSsh2Engine::new(config))
    };

    // Split Tunneling example
    let _ = routing::add_split_route("8.8.8.8", "192.168.1.1");

    tokio::spawn(async move {
        if let Err(e) = engine.start().await {
            error!("Core runtime error: {:?}", e);
        }
    });

    signal::ctrl_c().await?;
    info!("🛑 Shutdown signal received. Cleaning up...");

    let _ = routing::remove_split_route("8.8.8.8");

    info!("👋 Core stopped safely.");
    Ok(())
}
