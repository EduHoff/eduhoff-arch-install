use eduhoff_arch_install::cli;

fn main() {
    clearscreen::clear().expect("Fail to clear screen");

    println!("=== Testing eduhoff-arch-install UI ===");

    let selected_layout = cli::keyboard_layout::prompt();

    println!("\nResult saved in memory: {}", selected_layout);
}
