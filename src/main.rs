use std::process;

use anyhow::Result;
use clap::Parser;

use qweather::api::{
    air::AirAPI, astronomy::AstronomyAPI, geo::GeoAPI, grid_weather::GridWeatherAPI,
    indices::IndicesAPI, minutely::MinutelyAPI, warning::WarningAPI, weather::WeatherAPI,
};
use qweather::cli::format::*;
use qweather::cli::{Args, QueryType};
use qweather::{Config, QWeatherClient};

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        eprintln!("错误: {e}");
        process::exit(1);
    }
}

async fn run() -> Result<()> {
    let args = Args::parse();

    let private_key = if let Some(ref pk) = args.private_key {
        if pk.starts_with("---") {
            Some(pk.clone())
        } else {
            let expanded = shellexpand::tilde(pk);
            let path = std::path::Path::new(expanded.as_ref());
            if path.is_file() {
                Some(std::fs::read_to_string(path)?)
            } else {
                Some(pk.clone())
            }
        }
    } else {
        None
    };

    let config = Config::new(
        args.key,
        args.kid,
        args.project_id,
        private_key,
        None,
        args.base_url,
        args.geo_url,
    )
    .map_err(|e| anyhow::anyhow!(e))?;

    let client = QWeatherClient::new(config);

    let geo = GeoAPI::new(&client);
    let weather = WeatherAPI::new(&client);
    let air_api = AirAPI::new(&client);
    let indices_api = IndicesAPI::new(&client);
    let minutely_api = MinutelyAPI::new(&client);
    let warning_api = WarningAPI::new(&client);
    let astro_api = AstronomyAPI::new(&client);
    let grid_api = GridWeatherAPI::new(&client);

    let geo_resp = geo
        .city_lookup(&args.city, None, None, 1, "zh")
        .await
        .map_err(|e| anyhow::anyhow!(e))?;

    if geo_resp.location.is_empty() {
        println!("未找到城市: {}", args.city);
        process::exit(1);
    }

    let loc = &geo_resp.location[0];
    let coord = format!("{},{}", loc.lon, loc.lat);
    println!(
        "城市: {} ({} {})  ID: {}  坐标: {}\n",
        loc.name, loc.adm1, loc.adm2, loc.id, coord
    );

    match args.r#type {
        QueryType::Now => {
            let resp = weather.now(&loc.id, "zh", None).await?;
            print!("{}", format_now(&resp));
        }
        QueryType::Daily => {
            let resp = weather
                .daily_forecast(&loc.id, args.days, "zh", None)
                .await?;
            print!("{}", format_daily(&resp));
        }
        QueryType::Hourly => {
            let resp = weather
                .hourly_forecast(&loc.id, args.hours, "zh", None)
                .await?;
            print!("{}", format_hourly(&resp));
        }
        QueryType::Air => {
            let resp = air_api.now(&coord, "zh").await?;
            print!("{}", format_air(&resp));
        }
        QueryType::Indices => {
            let resp = indices_api
                .forecast(&loc.id, &args.index_days, "0", "zh")
                .await?;
            print!("{}", format_indices(&resp));
        }
        QueryType::Minutely => {
            let resp = minutely_api.precipitation(&coord, "zh").await?;
            print!("{}", format_minutely(&resp));
        }
        QueryType::Warning => {
            let resp = warning_api.now(&loc.id, "zh").await?;
            print!("{}", format_warning(&resp));
        }
        QueryType::Sun => {
            if args.date.is_empty() {
                return Err(anyhow::anyhow!(
                    "错误: 查询日出日落需要提供 --date yyyyMMdd"
                ));
            }
            let resp = astro_api.sunrise_sunset(&loc.id, &args.date, "zh").await?;
            print!("{}", format_astronomy(&resp));
        }
        QueryType::GridNow => {
            let resp = grid_api.now(&coord, "zh", None).await?;
            print!("{}", format_grid_now(&resp));
        }
        QueryType::GridDaily => {
            let resp = grid_api
                .daily_forecast(&coord, args.days, "zh", None)
                .await?;
            print!("{}", format_grid_daily(&resp));
        }
        QueryType::GridHourly => {
            let resp = grid_api
                .hourly_forecast(&coord, args.hours, "zh", None)
                .await?;
            print!("{}", format_grid_hourly(&resp));
        }
        QueryType::AirDaily => {
            let resp = air_api.daily_forecast(&coord, args.days, "zh").await?;
            print!("{}", format_air_daily(&resp));
        }
        QueryType::AirHourly => {
            let resp = air_api.hourly_forecast(&coord, args.hours, "zh").await?;
            print!("{}", format_air_hourly(&resp));
        }
        QueryType::Moon => {
            if args.date.is_empty() {
                return Err(anyhow::anyhow!("错误: 查询月相需要提供 --date yyyyMMdd"));
            }
            let resp = astro_api.moon_phase(&loc.id, &args.date, "zh").await?;
            print!("{}", format_moon(&resp));
        }
        QueryType::Solar => {
            if args.date.is_empty()
                || args.time.is_empty()
                || args.tz.is_empty()
                || args.alt.is_empty()
            {
                return Err(anyhow::anyhow!(
                    "错误: 查询太阳高度角需要提供 --date --time --tz --alt"
                ));
            }
            let resp = astro_api
                .solar_elevation_angle(&coord, &args.date, &args.time, &args.tz, &args.alt, "zh")
                .await?;
            print!("{}", format_solar(&resp));
        }
    }

    Ok(())
}
