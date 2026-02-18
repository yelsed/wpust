use color_eyre::eyre::Result;
use self_update::cargo_crate_version;

pub fn update() -> Result<()> {
    println!("Checking for updates...\n");

    let status = self_update::backends::github::Update::configure()
        .repo_owner("yelsed")
        .repo_name("wpust")
        .bin_name("wpust")
        .show_download_progress(true)
        .current_version(cargo_crate_version!())
        .build()
        .map_err(|e| color_eyre::eyre::eyre!("Failed to configure updater: {}", e))?
        .update()
        .map_err(|e| color_eyre::eyre::eyre!("Update failed: {}", e))?;

    if status.updated() {
        println!("\nUpdated wpust to version {}.", status.version());
    } else {
        println!("Already up to date (version {}).", status.version());
    }

    Ok(())
}
