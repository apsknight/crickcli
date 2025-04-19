use clap::{Parser, Subcommand};
use colored::*;
use std::error::Error;

mod cricket;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Get live cricket scores
    Live {
        #[command(subcommand)]
        filter: Option<LiveFilter>,
    },
    /// Get recent cricket matches
    Recent {
        #[command(subcommand)]
        filter: Option<LiveFilter>,
    },
    /// Get upcoming cricket matches
    Upcoming {
        #[command(subcommand)]
        filter: Option<LiveFilter>,
    },
}

#[derive(Subcommand)]
enum LiveFilter {
    /// Show only IPL matches
    Ipl,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Live { filter } => {
            println!("{}", "Fetching live cricket scores...".yellow());
            let matches = cricket::get_live_matches().await?;
            handle_matches(matches, filter);
        }
        Commands::Recent { filter } => {
            println!("{}", "Fetching recent cricket matches...".yellow());
            let matches = cricket::get_recent_matches().await?;
            handle_matches(matches, filter);
        }
        Commands::Upcoming { filter } => {
            println!("{}", "Fetching upcoming cricket matches...".yellow());
            let matches = cricket::get_upcoming_matches().await?;
            handle_matches(matches, filter);
        }
    }

    Ok(())
}

fn handle_matches(matches: Vec<cricket::Match>, filter: Option<LiveFilter>) {
    if matches.is_empty() {
        println!("{}", "No matches found.".red());
    } else {
        let filtered_matches: Vec<_> = match filter {
            Some(LiveFilter::Ipl) => matches
                .into_iter()
                .filter(|m| m.match_info.series_name.contains("Indian Premier League"))
                .collect(),
            None => matches,
        };
        
        println!("{}", cricket::format_matches(&filtered_matches));
    }
}