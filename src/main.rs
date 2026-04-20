use std::process;

use anyhow::Result;
use clap::Parser;

use qweather::cli::commands;
use qweather::cli::Cli;
use qweather::{Config, QWeatherClient};

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        eprintln!("错误: {e}");
        process::exit(1);
    }
}

async fn run() -> Result<()> {
    let cli = Cli::parse();

    let private_key = if let Some(ref pk) = cli.private_key {
        if pk.starts_with("---") {
            Some(pk.clone())
        } else {
            let expanded = shellexpand::tilde(pk);
            let path = std::path::Path::new(expanded.as_ref());
            if path.is_file() {
                Some(std::fs::read_to_string(path)?)
            } else {
                Some(pk.clone())
            }
        }
    } else {
        None
    };

    let config = Config::new(
        None,
        cli.kid,
        cli.project_id,
        private_key,
        None,
        cli.base_url,
        cli.geo_url,
    )
    .map_err(|e| anyhow::anyhow!(e))?;

    let client = QWeatherClient::new(config);
    let lang = &cli.lang;

    match &cli.command {
        qweather::cli::Commands::Geo(args) => {
            commands::geo::execute(&client, &args.command, lang).await?;
        }
        qweather::cli::Commands::Weather(args) => {
            commands::weather::execute(&client, &args.command, lang).await?;
        }
        qweather::cli::Commands::Grid(args) => {
            commands::grid::execute(&client, &args.command, lang).await?;
        }
        qweather::cli::Commands::Precipitation(args) => {
            commands::precipitation::execute(&client, args, lang).await?;
        }
        qweather::cli::Commands::Indices(args) => {
            commands::indices::execute(&client, args, lang).await?;
        }
        qweather::cli::Commands::Warning(args) => {
            commands::warning::execute(&client, args, lang).await?;
        }
        qweather::cli::Commands::Air(args) => {
            commands::air::execute(&client, &args.command, lang).await?;
        }
        qweather::cli::Commands::Sun(args) => {
            commands::sun::execute(&client, args, lang).await?;
        }
        qweather::cli::Commands::Moon(args) => {
            commands::moon::execute(&client, args, lang).await?;
        }
        qweather::cli::Commands::Solar(args) => {
            commands::solar::execute(&client, args, lang).await?;
        }
    }

    Ok(())
}
