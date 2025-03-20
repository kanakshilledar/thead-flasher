use log::{error, info};
use nix::unistd::Uid;
use rusb;
use std::path::PathBuf;
use std::process::Command;
use std::{thread, time};

pub fn check_dependencies(dependency: &str) -> bool {
    let output = Command::new("/usr/bin/which")
        .arg(dependency)
        .output()
        .expect("failed to execute process");

    let availability = output.status.success();
    if availability {
        info!("Found: {}", dependency);
    }
    availability
}

pub fn check_privilege() -> bool {
    Uid::effective().is_root()
}

fn add_delay(seconds: u64) {
    thread::sleep(time::Duration::from_secs(seconds as u64));
}

pub fn check_board_connected() -> bool {
    let devices = rusb::devices().unwrap();
    for device in devices.iter() {
        let device_desc = device.device_descriptor().unwrap();
        if device_desc.vendor_id() == 0x2345 && device_desc.product_id() == 0x7654 {
            info!("board found!");
            return true;
        }
    }
    false
}

pub fn flash_uboot(uboot: PathBuf) {
    info!("Flashing UBoot");
    let output = Command::new("/usr/bin/fastboot")
        .args(["flash", "ram", uboot.to_str().unwrap()])
        .output()
        .expect("failed to flash ram");
    if output.status.success() {
        info!("UBoot successfully flashed on RAM");
        add_delay(2);
        let output = Command::new("/usr/bin/fastboot")
            .arg("reboot")
            .output()
            .expect("failed to reboot");
        if output.status.success() {
            info!("Board rebooted successfully");
            add_delay(5);
            let output = Command::new("/usr/bin/fastboot")
                .args(["flash", "uboot", uboot.to_str().unwrap()])
                .output()
                .expect("failed to flash board");
            if output.status.success() {
                info!("UBoot successfully flashed on board");
            } else {
                error!("UBoot flashing failed!");
            }
        }
    } else {
        error!("UBoot flashing to RAM failed!");
    }
}

pub fn flash_boot(boot: PathBuf) {
    info!("Flashing Boot Partition");
    add_delay(1);
    let output = Command::new("/usr/bin/fastboot")
        .args(["flash", "boot", boot.to_str().unwrap()])
        .output()
        .expect("failed to flash board");
    if output.status.success() {
        info!("boot.ext4 successfully flashed on board");
    } else {
        error!("boot.ext4 flashing failed!");
    }
}

pub fn flash_root(root: PathBuf) {
    info!("Flashing Root Partition");
    add_delay(2);
    let output = Command::new("/usr/bin/fastboot")
        .args(["flash", "root", root.to_str().unwrap()])
        .output()
        .expect("failed to flash board");
    if output.status.success() {
        info!("root.ext4 successfully flashed on board");
        let output = Command::new("/usr/bin/fastboot")
            .arg("reboot")
            .output()
            .expect("failed to reboot");
        if output.status.success() {
            info!("Board rebooted successfully");
        }
    } else {
        error!("root.ext4 flashing failed!");
    }
}
