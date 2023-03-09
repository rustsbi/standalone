# RustSBI Service

This is the module which runs on backend to provide SBI or other interface features.
It should be loaded by `rom-rt` project and run on supervisor S-mode.

## LinuxBoot

This module should prepare initramfs and DTB for LinuxBoot.
It scans all bootable media if target board supports to do so.

## UEFI

If RISC-V UEFI feature is enabled, this service provides runtime UEFI service for kernel.
It would provide DXE driver environment, and do bootable device scan (sometimes referred
as BDS) before the kernel is up. 
