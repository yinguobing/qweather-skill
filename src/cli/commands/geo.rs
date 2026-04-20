use anyhow::Result;

use crate::api::geo::GeoAPI;
use crate::cli::format::*;
use crate::cli::GeoSubcommand;
use crate::client::QWeatherClient;

pub async fn execute(client: &QWeatherClient, cmd: &GeoSubcommand, lang: &str) -> Result<()> {
    let geo = GeoAPI::new(client);

    match cmd {
        GeoSubcommand::CityLookup { keyword, number } => {
            let resp = geo
                .city_lookup(keyword, None, None, *number, lang)
                .await?;
            print!("{}", format_geo_city_lookup(&resp));
        }
        GeoSubcommand::TopCity { range, number } => {
            let resp = geo.top_city(range, *number, lang).await?;
            print!("{}", format_geo_top_city(&resp));
        }
        GeoSubcommand::PoiLookup {
            keyword,
            r#type,
            city,
            number,
        } => {
            let resp = geo
                .poi_lookup(keyword, r#type, city.as_deref(), *number, lang)
                .await?;
            print!("{}", format_geo_poi(&resp));
        }
        GeoSubcommand::PoiRange {
            location,
            r#type,
            radius,
            number,
        } => {
            let resp = geo
                .poi_range(location, r#type, *radius, *number, lang)
                .await?;
            print!("{}", format_geo_poi(&resp));
        }
    }

    Ok(())
}
