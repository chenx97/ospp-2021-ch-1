use crate::SysInfo;
use anyhow::Result;
use blocking::unblock;
use sysinfo::SystemExt;

pub(crate) async fn fetch_system_info() -> Result<SysInfo> {
    unblock(|| {
        let mut system = sysinfo::System::new_all();
        system.refresh_all();

        // TODO: Implement getting other system info

        Ok(SysInfo {
            desktop: todo!(),
            distro: todo!(),
            ram: todo!(),
            hostname: system.get_host_name().unwrap_or_default(),
            kernel_version: system.get_kernel_version().unwrap_or_default(),
            systemd_version: todo!(),
        })
    })
    .await
}
