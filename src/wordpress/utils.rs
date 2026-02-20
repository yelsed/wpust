use crate::config::Config;

pub fn prepare_wordpress_url(site: String, config: &Config) -> String {
    if let Some(auth) = config.find_basic_auth(&site) {
        format!("https://{}:{}@{}", auth.username, auth.password, site)
    } else {
        format!("https://{}", site)
    }
}
