use anyhow::Result;

use crate::api::grid_weather::GridWeatherAPI;
use crate::cli::format::*;
use crate::cli::{resolve_location, GridSubcommand};
use crate::client::QWeatherClient;

pub async fn execute(client: &QWeatherClient, cmd: &GridSubcommand, lang: &str) -> Result<()> {
    let grid = GridWeatherAPI::new(client);

    match cmd {
        GridSubcommand::Now { location } => {
            let resolved = resolve_location(location, client).await?;
            let resp = grid.now(&resolved.coord, lang, None).await?;
            print!("{}", format_grid_now(&resp));
        }
        GridSubcommand::Daily { days, location } => {
            let resolved = resolve_location(location, client).await?;
            let resp = grid.daily_forecast(&resolved.coord, *days, lang, None).await?;
            print!("{}", format_grid_daily(&resp));
        }
        GridSubcommand::Hourly { hours, location } => {
            let resolved = resolve_location(location, client).await?;
            let resp = grid.hourly_forecast(&resolved.coord, *hours, lang, None).await?;
            print!("{}", format_grid_hourly(&resp));
        }
    }

    Ok(())
}
