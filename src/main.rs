use std::process::Command;

use eduhoff_arch_install::cli;

fn main() {
    clearscreen::clear().expect("Failed to clear screen");

    // Aqui em cima vai ter código para validar o hardware e outras coisas

    let valid_layouts = cli::keyboard_layout::get_available_keymap();

    let mut selected_layouts = cli::keyboard_layout::promt_user();

    if !valid_layouts.is_empty() && !valid_layouts.contains(&"br-abnt2".to_string()) {
        eprintln!(
            "Layout '{}' is invalid! Falling back to 'br-abnt2'.",
            selected_layouts
        );
        selected_layouts = "br-abnt2".to_string();
    }

    println!("Applying keyboard layout: {}", selected_layouts);
    let status = Command::new("loadkeys").arg(&selected_layouts).status();

    match status {
        Ok(s) if s.success() => println!("Layout applied successfully!"),
        _ => eprintln!("Failed to apply layout via loadkeys."),
    }
}
