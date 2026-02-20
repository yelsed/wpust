use color_eyre::eyre::Result;
use self_update::cargo_crate_version;

pub fn update() -> Result<()> {
    println!("Checking for updates...\n");

    let current = cargo_crate_version!();

    // Fetch the latest release tag to determine the target version.
    let releases = self_update::backends::github::ReleaseList::configure()
        .repo_owner("yelsed")
        .repo_name("wpust")
        .build()
        .map_err(|e| color_eyre::eyre::eyre!("Failed to fetch releases: {}", e))?
        .fetch()
        .map_err(|e| color_eyre::eyre::eyre!("Failed to fetch releases: {}", e))?;

    let latest = releases
        .first()
        .ok_or_else(|| color_eyre::eyre::eyre!("No releases found on GitHub"))?;

    let latest_version = latest.version.trim_start_matches('v');

    if latest_version == current {
        println!("Already up to date (version {}).", current);
        return Ok(());
    }

    println!("Current version: {}", current);
    println!("Latest version:  {}", latest_version);
    println!("Downloading update...\n");

    // Use target_version_tag to download directly, bypassing semver compatibility check.
    let status = self_update::backends::github::Update::configure()
        .repo_owner("yelsed")
        .repo_name("wpust")
        .bin_name("wpust")
        .show_download_progress(true)
        .current_version(current)
        .target_version_tag(&format!("v{}", latest_version))
        .build()
        .map_err(|e| color_eyre::eyre::eyre!("Failed to configure updater: {}", e))?
        .update()
        .map_err(|e| color_eyre::eyre::eyre!("Update failed: {}", e))?;

    println!("\nUpdated wpust from {} to {}.", current, status.version());

    Ok(())
}
