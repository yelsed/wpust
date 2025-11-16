use clap::{Parser, Subcommand};
use color_eyre::eyre::Result;
// use crossterm::{ExecutableCommand, terminal};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Perma {
        site: String
    },
    Theme {
        site: String
    }
}

fn main() -> Result<()> {
    color_eyre::install()?; 
    let args = Args::parse();

    match args.command {
        Commands::Perma {site} => {
            let site_after_check = acccheck(site);
            perma(site_after_check)?;
        },
        Commands::Theme {site} => {
            let site_after_check = acccheck(site);
            theme(site_after_check)?;
        }
    }
    
    Ok(())
}

fn acccheck(site: String) -> String { 
    let acceptation_basic_auth = "https://REDACTED:REDACTED@";
    let acceptation_string = "acc.";

    let url = if site.contains(acceptation_string) {
        acceptation_basic_auth.to_owned() + &site.to_string()
    } else {
        format!("https://{}", site)
    };

    url
}

fn perma(site: String) -> Result<()> {
    if site.is_empty() {
        return Err(color_eyre::eyre::eyre!("Site name may not be empty"));
    }
   
    let url = site + "/wp-admin/options-permalink.php";

    println!("{}", url);

    open::that(url)?;
    Ok(())
}


fn theme(site: String) -> Result<()> {
    if site.is_empty() {
        return Err(color_eyre::eyre::eyre!("Site name may not be empty"));
    }
    let url = site + "/wp-admin/themes.php";

    println!("{}", url);

    open::that(url)?;
    Ok(())
} 

