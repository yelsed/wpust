mod utils;

use color_eyre::eyre::{Result, eyre};
use crate::config::load_config;
use crate::wordpress::utils::prepare_wordpress_url;

fn open_wordpress_admin_page(
    site: String,
    page: &str,
    username: Option<String>,
    password: Option<String>,
) -> Result<()> {
    if site.is_empty() {
        return Err(color_eyre::eyre::eyre!("Site name may not be empty"));
    }

    let config = load_config()?;
    let admin_path = config.wp_admin_path.as_deref().unwrap_or("wp-admin");
    let prepared_url = prepare_wordpress_url(
        site,
        &config,
        username.as_deref(),
        password.as_deref(),
    );
    let url = format!("{}/{}/{}", prepared_url, admin_path, page);
    println!("opening: {}", url);

    let open_result = match config.browser.as_deref() {
        Some("default") | None => open::that(&url),
        Some(browser) => open::with(&url, browser),
    };

    if let Err(_) = open_result {
        return Err(eyre!(
            "Failed to open URL in browser. \
             Is a default browser configured?\n\
             Try: xdg-settings set default-web-browser firefox.desktop\n\
             Or run `wpust config` to set a specific browser."
        ));
    }

    Ok(())
}

pub fn perma(site: String, username: Option<String>, password: Option<String>) -> Result<()> {
    open_wordpress_admin_page(site, "options-permalink.php", username, password)
}

pub fn themes(site: String, username: Option<String>, password: Option<String>) -> Result<()> {
    open_wordpress_admin_page(site, "themes.php", username, password)
}

pub fn plugins(site: String, username: Option<String>, password: Option<String>) -> Result<()> {
    open_wordpress_admin_page(site, "plugins.php", username, password)
}

pub fn site_health(site: String, username: Option<String>, password: Option<String>) -> Result<()> {
    open_wordpress_admin_page(site, "site-health.php?tab=debug", username, password)
}
