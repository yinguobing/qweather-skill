use anyhow::Result;

use crate::api::air::AirAPI;
use crate::cli::format::*;
use crate::cli::{resolve_location, AirSubcommand};
use crate::client::QWeatherClient;

pub async fn execute(client: &QWeatherClient, cmd: &AirSubcommand, lang: &str) -> Result<()> {
    let air = AirAPI::new(client);

    match cmd {
        AirSubcommand::Now { location } => {
            let resolved = resolve_location(location, client).await?;
            let resp = air.now(&resolved.coord, lang).await?;
            print!("{}", format_air(&resp));
        }
        AirSubcommand::Hourly { hours, location } => {
            let resolved = resolve_location(location, client).await?;
            let resp = air.hourly_forecast(&resolved.coord, *hours, lang).await?;
            print!("{}", format_air_hourly(&resp));
        }
        AirSubcommand::Daily { days, location } => {
            let resolved = resolve_location(location, client).await?;
            let resp = air.daily_forecast(&resolved.coord, *days, lang).await?;
            print!("{}", format_air_daily(&resp));
        }
        AirSubcommand::Station { location } => {
            let resolved = resolve_location(location, client).await?;
            let resp = air.station(&resolved.coord, lang).await?;
            print!("{}", format_air_station(&resp));
        }
    }

    Ok(())
}
