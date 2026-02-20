use crate::config::Config;

pub fn prepare_wordpress_url(
    site: String,
    config: &Config,
    cli_username: Option<&str>,
    cli_password: Option<&str>,
) -> String {
    if let (Some(user), Some(pass)) = (cli_username, cli_password) {
        format!("https://{}:{}@{}", user, pass, site)
    } else if let Some(auth) = config.find_basic_auth(&site) {
        format!("https://{}:{}@{}", auth.username, auth.password, site)
    } else {
        format!("https://{}", site)
    }
}
