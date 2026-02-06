use clap::{Parser, Subcommand};

const ABOUT: &str = "WordPress admin CLI: open admin pages and run network diagnostics.";
const LONG_ABOUT: &str = "wpust is a WordPress administration CLI utility that opens WordPress \
admin pages in the browser and performs network diagnostics (DNS, IP, SSL, HTTP response timing) \
for sites.\n\nAll commands accept a site argument (hostname or URL).";

#[derive(Parser, Debug)]
#[command(version, about = ABOUT, long_about = LONG_ABOUT)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[command(
        about = "Open WordPress permalink settings in the browser",
        after_help = "Example:\n  wpust perma example.com"
    )]
    Perma {
        #[arg(help = "Hostname or URL")]
        site: String,
    },
    #[command(
        about = "Open WordPress themes page in the browser",
        after_help = "Example:\n  wpust themes example.com"
    )]
    Themes {
        #[arg(help = "Hostname or URL")]
        site: String,
    },
    #[command(
        about = "Open WordPress plugins page in the browser",
        after_help = "Example:\n  wpust plugins example.com"
    )]
    Plugins {
        #[arg(help = "Hostname or URL")]
        site: String,
    },
    #[command(
        about = "Open WordPress site health (debug) page in the browser",
        after_help = "Example:\n  wpust siteinfo example.com"
    )]
    Siteinfo {
        #[arg(help = "Hostname or URL")]
        site: String,
    },
    #[command(
        about = "Look up DNS records (A, AAAA, MX, TXT, NS, CNAME, SOA)",
        after_help = "Example:\n  wpust dns example.com"
    )]
    Dns {
        #[arg(help = "Hostname or URL")]
        site: String,
    },
    #[command(
        about = "Look up IP address and reverse DNS",
        after_help = "Example:\n  wpust ip example.com"
    )]
    Ip {
        #[arg(help = "Hostname or URL")]
        site: String,
    },
    #[command(
        about = "Show SSL certificate details and expiry",
        after_help = "Example:\n  wpust ssl example.com"
    )]
    Ssl {
        #[arg(help = "Hostname or URL")]
        site: String,
    },
    #[command(
        about = "Measure HTTP response timing",
        after_help = "Example:\n  wpust response example.com"
    )]
    Response {
        #[arg(help = "Hostname or URL")]
        site: String,
    },
    #[command(
        about = "Measure page load time in a headless browser (WIP)",
        after_help = "Example:\n  wpust page-load example.com"
    )]
    PageLoad {
        #[arg(help = "Hostname or URL")]
        site: String,
    },
    #[command(
        about = "Goose command",
        after_help = "Example:\n  wpust goose example.com"
    )]
    Goose {
        #[arg(help = "Hostname or URL")]
        site: String,
    },
    #[command(
        name = "about",
        about = "Show this help screen",
        after_help = "Example:\n  wpust about"
    )]
    About,
}
