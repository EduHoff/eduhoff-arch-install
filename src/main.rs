use eduhoff_arch_install::{cli, hardware};

fn main() {
    clearscreen::clear().expect("Fail to clear screen");

    let total_ram = hardware::ram::total_ram();
    let swap_size = hardware::ram::swap_size(total_ram, false);
    println!("MiB {}", total_ram);
    println!("Swap size: {}", swap_size);

    println!("=== Testing eduhoff-arch-install UI ===");

    let selected_layout = cli::keyboard_layout::prompt();

    println!("\nResult saved in memory: {}", selected_layout);
}
