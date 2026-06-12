use std::{
    io::{self, Write},
    process::Command,
};

pub fn get_available_keymap() -> Vec<String> {
    let output = Command::new("localectl").arg("list-keymaps").output();

    match output {
        Ok(out) if !out.status.success() => {
            let text = String::from_utf8_lossy(&out.stdout);
            text.lines().map(|l| l.trim().to_string()).collect()
        }
        _ => Vec::new(),
    }
}

pub fn promt_user() -> String {
    println!("\n=== Keyboard Configuration ===");
    print!("Enter your keyboard layout [Default: br-abnt2]: ");
    io::stdout().flush().expect("Failed to flush");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let choosen = input.trim().to_string();
    if choosen.is_empty() {
        "br-abnt".to_string()
    } else {
        choosen
    }
}
