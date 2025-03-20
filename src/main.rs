use clap::{Arg, Command, builder::PathBufValueParser};
use log::error;
use std::path::PathBuf;
use thead_flasher::{
    check_board_connected, check_dependencies, check_privilege, flash_boot, flash_root, flash_uboot,
};

fn main() {
    let matches = Command::new("TH1520 Board Flashing Utility")
        .version("0.1.0")
        .author("Kanak Shilledar <kanakshilledar111@protonmail.com>")
        .arg(
            Arg::new("uboot")
                .short('u')
                .long("uboot")
                .help("Path of u-boot-with-spl.bin")
                .value_parser(PathBufValueParser::default())
                .required(true),
        )
        .arg(
            Arg::new("boot")
                .short('b')
                .long("boot")
                .help("Path of boot.ext4")
                .value_parser(PathBufValueParser::default())
                .required(true),
        )
        .arg(
            Arg::new("root")
                .short('r')
                .long("root")
                .help("Path of root.ext4")
                .value_parser(PathBufValueParser::default()),
        )
        .get_matches();

    colog::init();

    if !check_dependencies("fastboot") {
        error!("Please install fastboot first!");
        std::process::exit(1);
    }

    if !check_privilege() {
        error!("Please run with sudo privileges")
    }

    if !check_board_connected() {
        error!("Board not connected!");
        std::process::exit(1);
    }

    let uboot_path: &PathBuf = matches.get_one("uboot").unwrap();
    let bootfs_path: &PathBuf = matches.get_one("boot").unwrap();
    let root_path: Option<&PathBuf> = matches.get_one("root");

    flash_uboot(uboot_path.clone());
    flash_boot(bootfs_path.clone());

    if let Some(root_path) = root_path {
        flash_root(root_path.clone());
    }
}
