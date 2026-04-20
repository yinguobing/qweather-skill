use anyhow::Result;

use crate::api::indices::IndicesAPI;
use crate::cli::format::*;
use crate::cli::{resolve_location, IndicesArgs};
use crate::client::QWeatherClient;

pub async fn execute(client: &QWeatherClient, args: &IndicesArgs, lang: &str) -> Result<()> {
    let resolved = resolve_location(&args.location, client).await?;
    let api = IndicesAPI::new(client);
    let resp = api
        .forecast(&resolved.id, &args.days, &args.r#type, lang)
        .await?;
    print!("{}", format_indices(&resp));
    Ok(())
}
