mod cli;
mod config;
mod config_cmd;
mod goose;
mod network;
mod uninstall;
mod update;
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
        Commands::Perma { wp } => perma(wp.site, wp.username, wp.password)?,
        Commands::Themes { wp } => themes(wp.site, wp.username, wp.password)?,
        Commands::Plugins { wp } => plugins(wp.site, wp.username, wp.password)?,
        Commands::Siteinfo { wp } => site_health(wp.site, wp.username, wp.password)?,
        Commands::Dns { site } => dns(site)?,
        Commands::Ip { site } => ip(site)?,
        Commands::Ssl { site } => ssl(site)?,
        Commands::Response { site } => response(site)?,
        Commands::PageLoad { site, strategy, key } => page_load(site, strategy, key)?,
        Commands::Config => config_cmd::setup()?,
        Commands::Update => update::update()?,
        Commands::Goose => goose::goose()?,
        Commands::Version => println!("wpust {}", env!("CARGO_PKG_VERSION")),
        Commands::Uninstall => uninstall::uninstall()?,
        Commands::About => Args::command().print_long_help().map_err(color_eyre::eyre::Report::from)?,
    }

    Ok(())
}
