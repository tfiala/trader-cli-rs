use anyhow::Result;
use clap::{Args, Parser, Subcommand};
use colored::Colorize;

#[derive(Parser, Debug)]
#[command(name = "tr-cli")]
#[command(version, about = "trader cli", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// The trader server URI to connect to. Defaults to SERVER_URI in .env file.
    #[arg(short, long, required = false)]
    server_uri: Option<String>,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Retrieve the list of accessible accounts.
    Accounts,

    /// Retrieve end of day summary info for the most recent trading day.
    ///
    /// Typically this is from end of day of the day before the current day.
    EndOfDaySummary(EndOfDaySummaryArgs),
}

#[derive(Args, Debug)]
struct EndOfDaySummaryArgs {
    #[arg(short, long, value_parser = clap::value_parser!(u32), default_value = "1")]
    days: u32,
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv()?;

    let args = Cli::parse();

    let server_uri = args.server_uri.clone().unwrap_or_else(|| {
        dotenvy::var("SERVER_URI").unwrap_or_else(|_| {
            panic!("No server URI provided. Please set the SERVER_URI environment variable or use the --server-uri arg.")
        })
    });

    let arg_str = format!("{:?}", args);
    println!("args: {}", arg_str.bright_white());
    println!("Connecting to server: {}", server_uri);

    Ok(())
}
