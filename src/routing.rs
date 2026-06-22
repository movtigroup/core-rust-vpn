use std::process::Command;
use tracing::{info, warn, error};

pub fn add_split_route(target_ip: &str, gateway_ip: &str) -> Result<(), String> {
    info!("Adding route: {} via {}", target_ip, gateway_ip);
    let output = if cfg!(target_os = "linux") {
        Command::new("ip")
            .args(["route", "add", target_ip, "via", gateway_ip])
            .output()
    } else if cfg!(target_os = "macos") {
        Command::new("route")
            .args(["-n", "add", "-net", target_ip, gateway_ip])
            .output()
    } else if cfg!(target_os = "windows") {
        Command::new("route")
            .args(["add", target_ip, "mask", "255.255.255.255", gateway_ip])
            .output()
    } else {
        return Err("Unsupported Operating System".to_string());
    };

    match output {
        Ok(out) if out.status.success() => {
            info!("Route added successfully");
            Ok(())
        },
        Ok(out) => {
            let err = String::from_utf8_lossy(&out.stderr).to_string();
            error!("Failed to add route: {}", err);
            Err(err)
        },
        Err(e) => {
            error!("Process error: {}", e);
            Err(e.to_string())
        },
    }
}

pub fn remove_split_route(target_ip: &str) -> Result<(), String> {
    info!("Removing route: {}", target_ip);
    let output = if cfg!(target_os = "linux") {
        Command::new("ip").args(["route", "del", target_ip]).output()
    } else if cfg!(target_os = "macos") {
        Command::new("route").args(["-n", "delete", "-net", target_ip]).output()
    } else if cfg!(target_os = "windows") {
        Command::new("route").args(["delete", target_ip]).output()
    } else {
        return Err("Unsupported Operating System".to_string());
    };

    match output {
        Ok(out) if out.status.success() => {
            info!("Route removed successfully");
            Ok(())
        },
        Ok(out) => {
            let err = String::from_utf8_lossy(&out.stderr).to_string();
            warn!("Failed to remove route (might not exist): {}", err);
            Err(err)
        },
        Err(e) => {
            error!("Process error: {}", e);
            Err(e.to_string())
        },
    }
}
