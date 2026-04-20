use anyhow::Result;

use crate::api::weather::WeatherAPI;
use crate::cli::format::*;
use crate::cli::{resolve_location, WeatherSubcommand};
use crate::client::QWeatherClient;

pub async fn execute(
    client: &QWeatherClient,
    cmd: &WeatherSubcommand,
    lang: &str,
) -> Result<()> {
    let weather = WeatherAPI::new(client);

    match cmd {
        WeatherSubcommand::Now { location } => {
            let resolved = resolve_location(location, client).await?;
            let resp = weather.now(&resolved.id, lang, None).await?;
            print!("{}", format_now(&resp));
        }
        WeatherSubcommand::Daily { days, location } => {
            let resolved = resolve_location(location, client).await?;
            let resp = weather.daily_forecast(&resolved.id, *days, lang, None).await?;
            print!("{}", format_daily(&resp));
        }
        WeatherSubcommand::Hourly { hours, location } => {
            let resolved = resolve_location(location, client).await?;
            let resp = weather.hourly_forecast(&resolved.id, *hours, lang, None).await?;
            print!("{}", format_hourly(&resp));
        }
    }

    Ok(())
}
