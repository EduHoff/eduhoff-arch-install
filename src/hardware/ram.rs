use sysinfo::System;

const BYTES_TO_MIB: u64 = 1_048_576;

pub fn total_ram() -> u64 {
    let mut sys = System::new_all();

    sys.refresh_memory();

    let bytes = sys.total_memory();
    bytes / BYTES_TO_MIB
}

pub fn swap_size(ram: u64, hibernation: bool) -> u64 {
    if ram < 2048 {
        if hibernation { ram * 3 } else { ram * 2 }
    } else if (2048..8192).contains(&ram) {
        if hibernation { ram * 2 } else { ram }
    } else if (8192..65536).contains(&ram) {
        if hibernation {
            ((ram as f64) * 1.5) as u64
        } else {
            8192
        }
    } else {
        4096
    }
}
