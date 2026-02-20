use color_eyre::eyre::{Result, eyre};
use inquire::Confirm;

pub fn uninstall() -> Result<()> {
    let exe_path = std::env::current_exe()
        .map_err(|e| eyre!("Could not determine binary path: {}", e))?;

    println!("This will remove wpust from your system.");
    println!("Binary: {}", exe_path.display());

    let confirm = Confirm::new("Are you sure you want to uninstall wpust?")
        .with_default(false)
        .prompt()?;

    if !confirm {
        println!("Uninstall cancelled.");
        return Ok(());
    }

    // Remove the binary
    std::fs::remove_file(&exe_path)
        .map_err(|e| eyre!("Failed to remove binary at {}: {}", exe_path.display(), e))?;
    println!("Removed binary: {}", exe_path.display());

    // Offer to remove config directory
    if let Some(config_dir) = dirs::config_dir() {
        let wpust_config = config_dir.join("wpust");
        if wpust_config.exists() {
            let remove_config = Confirm::new(&format!(
                "Also remove config directory ({})?",
                wpust_config.display()
            ))
            .with_default(false)
            .prompt()?;

            if remove_config {
                std::fs::remove_dir_all(&wpust_config)
                    .map_err(|e| eyre!("Failed to remove config directory: {}", e))?;
                println!("Removed config: {}", wpust_config.display());
            }
        }
    }

    println!("wpust has been uninstalled.");
    Ok(())
}
