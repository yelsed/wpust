use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Perma {
        site: String
    },
    Themes {
        site: String
    },
    Plugins {
        site: String
    },
    Siteinfo {
        site: String
    },
    Dns {
        site: String
    },
    Ip {
        site: String
    },
    Ssl {
        site: String
    },
    Response {
        site: String
    },
    PageLoad {
        site: String
    }
}
