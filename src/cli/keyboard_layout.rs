use std::process::Command;

use dialoguer::{Select, theme::ColorfulTheme};

pub fn get_available_keymaps() -> Vec<String> {
    let output = Command::new("localectl").arg("list-keymaps").output();

    match output {
        Ok(out) if out.status.success() => {
            let text = String::from_utf8_lossy(&out.stdout);
            text.lines().map(|l| l.trim().to_string()).collect()
        }
        _ => {
            vec![
                "br-abnt2".to_string(),
                "us".to_string(),
                "us-intl".to_string(),
                "cn".to_string(),
                "es".to_string(),
                "fr".to_string(),
                "de".to_string(),
                "it".to_string(),
                "jp".to_string(),
            ]
        }
    }
}

pub fn prompt() -> String {
    let mut keymaps = get_available_keymaps();

    if let Some(pos) = keymaps.iter().position(|x| x == "br-abnt2") {
        let default_item = keymaps.remove(pos);
        keymaps.insert(0, default_item);
    }

    println!("\n === keyboard Configuration ===");

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select your keyboard layout (Use arrow keys, press ENTER)")
        .items(&keymaps)
        .default(0)
        .max_length(7)
        .interact_opt()
        .expect("Fail to selection");

    match selection {
        Some(index) => keymaps[index].clone(),
        None => "br-abnt2".to_string(),
    }
}
