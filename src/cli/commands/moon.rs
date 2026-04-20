use anyhow::Result;

use crate::api::astronomy::AstronomyAPI;
use crate::cli::format::*;
use crate::cli::{resolve_location, MoonArgs};
use crate::client::QWeatherClient;

pub async fn execute(client: &QWeatherClient, args: &MoonArgs, lang: &str) -> Result<()> {
    let resolved = resolve_location(&args.location, client).await?;
    let api = AstronomyAPI::new(client);
    let resp = api.moon_phase(&resolved.id, &args.date, lang).await?;
    print!("{}", format_moon(&resp));
    Ok(())
}
