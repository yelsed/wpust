mod cli;
mod config;
mod goose;
mod network;
mod utils;
mod wordpress;

use color_eyre::eyre::Result;
use clap::{CommandFactory, Parser};
use cli::{Args, Commands};
use wordpress::{perma, themes, plugins, site_health};
use network::{dns, ip, ssl, response, page_load};

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
        Commands::Response { site } => response(site)?,
        Commands::PageLoad { site } => page_load(site)?,
        Commands::Goose => goose::goose()?,
        Commands::About => Args::command().print_long_help().map_err(color_eyre::eyre::Report::from)?,
    }

    Ok(())
}
