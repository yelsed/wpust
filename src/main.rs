mod cli;
mod wordpress;
mod network;
mod utils;

use color_eyre::eyre::Result;
use clap::Parser;
use cli::{Args, Commands};
use wordpress::{perma, themes, plugins, site_health};
use network::{dns, ip, ssl, whois};

fn main() -> Result<()> {
    color_eyre::install()?;
    let args = Args::parse();

    match args.command {
        Commands::Perma { site } => perma(site)?,
        Commands::Themes { site } => themes(site)?,
        Commands::Plugins { site } => plugins(site)?,
        Commands::Siteinfo { site } => site_health(site)?,
        Commands::Dns { site } => dns(site)?,
        Commands::Ip { site } => ip(site)?,
        Commands::Ssl { site } => ssl(site)?,
        Commands::Whois { site } => whois(site)?,
    }

    Ok(())
}
