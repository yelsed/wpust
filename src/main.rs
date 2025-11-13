use clap::{Parser, Subcommand};
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
    }
}

fn main() {
    let args = Args::parse();

    match args.command {
        Commands::Perma {site} => {
            perma(site)
        }
    }
}

fn perma(site: String)-> Result<(), std::io::Error>{
    let url = format!("https://{}", site);
    open::that(url)?;
    Ok(())
}
