use anyhow::Result;

use crate::api::minutely::MinutelyAPI;
use crate::cli::format::*;
use crate::cli::{resolve_location, PrecipitationArgs};
use crate::client::QWeatherClient;

pub async fn execute(client: &QWeatherClient, args: &PrecipitationArgs, lang: &str) -> Result<()> {
    let resolved = resolve_location(&args.location, client).await?;
    let api = MinutelyAPI::new(client);
    let resp = api.precipitation(&resolved.coord, lang).await?;
    print!("{}", format_minutely(&resp));
    Ok(())
}
