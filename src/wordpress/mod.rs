mod utils;

use color_eyre::eyre::Result;
use crate::wordpress::utils::prepare_wordpress_url;

fn open_wordpress_admin_page(site: String, admin_path: &str) -> Result<()> {
    if site.is_empty() {
        return Err(color_eyre::eyre::eyre!("Site name may not be empty"));
    }

    let prepared_url = prepare_wordpress_url(site);
    let url = format!("{}/{}", prepared_url, admin_path);
    println!("opening: {}", url);
    open::that(url)?;
    Ok(())
}

pub fn perma(site: String) -> Result<()> {
    open_wordpress_admin_page(site, "wp-admin/options-permalink.php")
}

pub fn themes(site: String) -> Result<()> {
    open_wordpress_admin_page(site, "wp-admin/themes.php")
}

pub fn plugins(site: String) -> Result<()> {
    open_wordpress_admin_page(site, "wp-admin/plugins.php")
}

pub fn site_health(site: String) -> Result<()> {
    open_wordpress_admin_page(site, "wp-admin/site-health.php?tab=debug")
}
