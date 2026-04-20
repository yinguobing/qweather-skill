use anyhow::Result;

use crate::api::astronomy::AstronomyAPI;
use crate::cli::format::*;
use crate::cli::{resolve_location, SolarArgs};
use crate::client::QWeatherClient;

pub async fn execute(client: &QWeatherClient, args: &SolarArgs, lang: &str) -> Result<()> {
    let resolved = resolve_location(&args.location, client).await?;
    let api = AstronomyAPI::new(client);
    let resp = api
        .solar_elevation_angle(
            &resolved.coord,
            &args.date,
            &args.time,
            &args.tz,
            &args.alt,
            lang,
        )
        .await?;
    print!("{}", format_solar(&resp));
    Ok(())
}
