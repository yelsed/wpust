use color_eyre::eyre::Result;
use inquire::{Confirm, Password, Select, Text};

use crate::config::{BasicAuthRule, Config, config_path, load_config, save_config};

const BROWSER_OPTIONS: [&str; 4] = ["System default", "Firefox", "Chrome", "Chromium"];

fn browser_value(selection: &str) -> Option<String> {
    match selection {
        "System default" => None,
        other => Some(other.to_lowercase()),
    }
}

fn browser_display(config: &Config) -> &str {
    match config.browser.as_deref() {
        None | Some("default") => "System default",
        Some("firefox") => "Firefox",
        Some("chrome") => "Chrome",
        Some("chromium") => "Chromium",
        _ => "System default",
    }
}

pub fn setup() -> Result<()> {
    let mut config = load_config()?;

    // Browser selection
    let current_browser = browser_display(&config);
    let default_idx = BROWSER_OPTIONS
        .iter()
        .position(|&o| o == current_browser)
        .unwrap_or(0);

    let browser_choice = Select::new(
        "Which browser should wpust use to open pages?",
        BROWSER_OPTIONS.to_vec(),
    )
    .with_starting_cursor(default_idx)
    .prompt()?;

    config.browser = browser_value(browser_choice);

    // WP admin path
    let default_path = config
        .wp_admin_path
        .as_deref()
        .unwrap_or("wp-admin")
        .to_string();

    let admin_path = Text::new("Custom WordPress admin path:")
        .with_default(&default_path)
        .prompt()?;

    config.wp_admin_path = if admin_path == "wp-admin" {
        None
    } else {
        Some(admin_path)
    };

    // Show existing basic auth rules
    if !config.basic_auth.is_empty() {
        println!("\nExisting basic auth rules:");
        for rule in &config.basic_auth {
            println!("  - {} (user: {})", rule.pattern, rule.username);
        }
        println!();

        let keep = Confirm::new("Keep existing basic auth rules?")
            .with_default(true)
            .prompt()?;

        if !keep {
            config.basic_auth.clear();
        }
    }

    // Add new basic auth rules
    loop {
        let add = Confirm::new("Add a basic auth rule?")
            .with_default(false)
            .prompt()?;

        if !add {
            break;
        }

        let pattern = Text::new("URL pattern (e.g. staging.example.com):")
            .prompt()?;

        let username = Text::new("Username:")
            .prompt()?;

        let password = Password::new("Password:")
            .without_confirmation()
            .prompt()?;

        println!("  Added rule for '{}'.", pattern);

        config.basic_auth.push(BasicAuthRule {
            pattern,
            username,
            password,
        });
    }

    save_config(&config)?;
    println!("\nConfig saved to {}", config_path()?.display());

    Ok(())
}
