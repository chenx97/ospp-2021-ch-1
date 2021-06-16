use crate::SysInfo;
use anyhow::Result;
use blocking::unblock;
use dbus::blocking::{stdintf::org_freedesktop_dbus::Properties, Connection};
use pretty_bytes::converter::convert;
use std::time::Duration;
use sysinfo::SystemExt;

pub(crate) async fn fetch_system_info() -> Result<SysInfo> {
    unblock(|| {
        let mut system = sysinfo::System::new_all();
        system.refresh_all();

        // TODO: Implement getting other system info

        Ok(SysInfo {
            desktop: get_desktop_environment().unwrap_or("unknown".to_string()),
            distro: system.get_name().unwrap_or_default(),
            ram: format!("{}", convert((system.get_total_memory() * 1024) as f64)),
            hostname: system.get_host_name().unwrap_or_default(),
            kernel_version: system.get_kernel_version().unwrap_or_default(),
            systemd_version: get_systemd_version().unwrap_or("not found".to_string()),
        })
    })
    .await
}

/// inspired by https://github.com/AOSC-Dev/ciel-rs.git
/// adapted from test_sd_bus() in diagnose.rs
fn get_systemd_version() -> Result<String> {
    Ok(Connection::new_system()?
        .with_proxy(
            "org.freedesktop.systemd1",
            "/org/freedesktop/systemd1",
            Duration::from_secs(10),
        )
        .get("org.freedesktop.systemd1.Manager", "Version")?)
}

fn get_desktop_environment() -> Result<String> {
    Ok(Connection::new_system()?
        .with_proxy(
            "org.freedesktop.login1",
            "/org/freedesktop/login1/session/auto",
            Duration::from_secs(10),
        )
        .get("org.freedesktop.login1.Session", "Desktop")?)
}
