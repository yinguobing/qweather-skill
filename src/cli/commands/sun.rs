use anyhow::Result;

use crate::api::astronomy::AstronomyAPI;
use crate::cli::format::*;
use crate::cli::{resolve_location, SunArgs};
use crate::client::QWeatherClient;

pub async fn execute(client: &QWeatherClient, args: &SunArgs, lang: &str) -> Result<()> {
    let resolved = resolve_location(&args.location, client).await?;
    let api = AstronomyAPI::new(client);
    let resp = api.sunrise_sunset(&resolved.id, &args.date, lang).await?;
    print!("{}", format_astronomy(&resp));
    Ok(())
}
