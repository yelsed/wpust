pub fn prepare_wordpress_url(site: String) -> String {
    let acceptation_basic_auth = "https://REDACTED:REDACTED@";
    let acceptation_string = "acc.";

    if site.contains(acceptation_string) {
        acceptation_basic_auth.to_owned() + &site.to_string()
    } else {
        format!("https://{}", site)
    }
}
