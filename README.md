# TH1520 Board Flashing Utility

Simple tool to flash TH1520 based RISC-V board using fastboot.

```console
$ thead-flasher --help
Usage: thead-flasher [OPTIONS] --uboot <uboot> --boot <boot>

Options:
  -u, --uboot <uboot>  Path of u-boot-with-spl.bin
  -b, --boot <boot>    Path of boot.ext4
  -r, --root <root>    Path of root.ext4
  -h, --help           Print help
  -V, --version        Print version

```

Only UBoot and Boot partitions are required, Root partition is optional.