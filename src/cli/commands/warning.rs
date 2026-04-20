use anyhow::Result;

use crate::api::warning::WarningAPI;
use crate::cli::format::*;
use crate::cli::{resolve_location, WarningArgs};
use crate::client::QWeatherClient;

pub async fn execute(client: &QWeatherClient, args: &WarningArgs, lang: &str) -> Result<()> {
    let resolved = resolve_location(&args.location, client).await?;
    let api = WarningAPI::new(client);
    let resp = api.now(&resolved.id, lang).await?;
    print!("{}", format_warning(&resp));
    Ok(())
}
