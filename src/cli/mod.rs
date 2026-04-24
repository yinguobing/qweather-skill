pub mod commands;
pub mod format;

use clap::{Args, Parser, Subcommand};

use crate::api::geo::GeoAPI;
use crate::client::QWeatherClient;
use crate::error::QWeatherError;

#[derive(Parser, Debug)]
#[command(name = "qw", version)]
#[command(about = "和风天气查询")]
pub struct Cli {
    #[arg(long, help = "JWT 凭据 ID")]
    pub kid: Option<String>,

    #[arg(long, help = "JWT 项目 ID（sub）")]
    pub project_id: Option<String>,

    #[arg(long, help = "JWT Ed25519 私钥内容或 PEM 文件路径")]
    pub private_key: Option<String>,

    #[arg(long, help = "自定义 API Host 的天气数据基础 URL")]
    pub base_url: Option<String>,

    #[arg(long, help = "自定义 API Host 的 Geo 基础 URL")]
    pub geo_url: Option<String>,

    #[arg(long, default_value = "zh", help = "语言")]
    pub lang: String,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[command(about = "地理位置查询")]
    Geo(GeoArgs),
    #[command(about = "天气查询")]
    Weather(WeatherArgs),
    #[command(about = "格点天气查询")]
    Grid(GridArgs),
    #[command(about = "分钟级降水预报")]
    Precipitation(PrecipitationArgs),
    #[command(about = "天气指数预报")]
    Indices(IndicesArgs),
    #[command(about = "实时天气预警")]
    Warning(WarningArgs),
    #[command(about = "空气质量查询")]
    Air(AirArgs),
    #[command(about = "日出日落")]
    Sun(SunArgs),
    #[command(about = "月升月落和月相")]
    Moon(MoonArgs),
    #[command(about = "太阳高度角")]
    Solar(SolarArgs),
}

// ============================================================================
// Geo
// ============================================================================

#[derive(Args, Debug)]
pub struct GeoArgs {
    #[command(subcommand)]
    pub command: GeoSubcommand,
}

#[derive(Subcommand, Debug)]
pub enum GeoSubcommand {
    #[command(about = "城市搜索")]
    CityLookup {
        keyword: String,
        #[arg(long, default_value_t = 10)]
        number: i32,
    },
    #[command(about = "经纬度坐标反查城市")]
    Reverse {
        #[arg(long)]
        lon: f64,
        #[arg(long)]
        lat: f64,
        #[arg(long, default_value_t = 10)]
        number: i32,
    },
    #[command(about = "热门城市")]
    TopCity {
        #[arg(long, default_value = "cn")]
        range: String,
        #[arg(long, default_value_t = 10)]
        number: i32,
    },
    #[command(about = "POI 搜索")]
    PoiLookup {
        keyword: String,
        #[arg(long, default_value = "scenic")]
        r#type: String,
        #[arg(long)]
        city: Option<String>,
        #[arg(long, default_value_t = 10)]
        number: i32,
    },
    #[command(about = "POI 范围搜索")]
    PoiRange {
        location: String,
        #[arg(long, default_value = "scenic")]
        r#type: String,
        #[arg(long, default_value_t = 10)]
        radius: i32,
        #[arg(long, default_value_t = 10)]
        number: i32,
    },
}

// ============================================================================
// Weather
// ============================================================================

#[derive(Args, Debug)]
pub struct WeatherArgs {
    #[command(subcommand)]
    pub command: WeatherSubcommand,
}

#[derive(Subcommand, Debug)]
pub enum WeatherSubcommand {
    #[command(about = "实时天气")]
    Now {
        #[command(flatten)]
        location: LocationArgs,
    },
    #[command(about = "每日天气预报")]
    Daily {
        #[arg(value_name = "DAYS", default_value_t = 3)]
        days: i32,
        #[command(flatten)]
        location: LocationArgs,
    },
    #[command(about = "逐小时天气预报")]
    Hourly {
        #[arg(value_name = "HOURS", default_value_t = 24)]
        hours: i32,
        #[command(flatten)]
        location: LocationArgs,
    },
}

// ============================================================================
// Grid
// ============================================================================

#[derive(Args, Debug)]
pub struct GridArgs {
    #[command(subcommand)]
    pub command: GridSubcommand,
}

#[derive(Subcommand, Debug)]
pub enum GridSubcommand {
    #[command(about = "格点实时天气")]
    Now {
        #[command(flatten)]
        location: LocationArgs,
    },
    #[command(about = "格点每日天气预报")]
    Daily {
        #[arg(value_name = "DAYS", default_value_t = 3)]
        days: i32,
        #[command(flatten)]
        location: LocationArgs,
    },
    #[command(about = "格点逐小时天气预报")]
    Hourly {
        #[arg(value_name = "HOURS", default_value_t = 24)]
        hours: i32,
        #[command(flatten)]
        location: LocationArgs,
    },
}

// ============================================================================
// Air
// ============================================================================

#[derive(Args, Debug)]
pub struct AirArgs {
    #[command(subcommand)]
    pub command: AirSubcommand,
}

#[derive(Subcommand, Debug)]
pub enum AirSubcommand {
    #[command(about = "实时空气质量")]
    Now {
        #[command(flatten)]
        location: LocationArgs,
    },
    #[command(about = "空气质量小时预报")]
    Hourly {
        #[arg(value_name = "HOURS", default_value_t = 24)]
        hours: i32,
        #[command(flatten)]
        location: LocationArgs,
    },
    #[command(about = "空气质量每日预报")]
    Daily {
        #[arg(value_name = "DAYS", default_value_t = 3)]
        days: i32,
        #[command(flatten)]
        location: LocationArgs,
    },
    #[command(about = "监测站数据")]
    Station {
        #[command(flatten)]
        location: LocationArgs,
    },
}

// ============================================================================
// Leaf commands
// ============================================================================

#[derive(Args, Debug)]
pub struct PrecipitationArgs {
    #[command(flatten)]
    pub location: LocationArgs,
}

#[derive(Args, Debug)]
pub struct IndicesArgs {
    #[arg(long, default_value = "1d")]
    pub days: String,
    #[arg(long, default_value = "0")]
    pub r#type: String,
    #[command(flatten)]
    pub location: LocationArgs,
}

#[derive(Args, Debug)]
pub struct WarningArgs {
    #[command(flatten)]
    pub location: LocationArgs,
}

#[derive(Args, Debug)]
pub struct SunArgs {
    #[arg(long)]
    pub date: String,
    #[command(flatten)]
    pub location: LocationArgs,
}

#[derive(Args, Debug)]
pub struct MoonArgs {
    #[arg(long)]
    pub date: String,
    #[command(flatten)]
    pub location: LocationArgs,
}

#[derive(Args, Debug)]
pub struct SolarArgs {
    #[arg(long)]
    pub date: String,
    #[arg(long)]
    pub time: String,
    #[arg(long)]
    pub tz: String,
    #[arg(long)]
    pub alt: String,
    #[command(flatten)]
    pub location: LocationArgs,
}

// ============================================================================
// Shared location args
// ============================================================================

#[derive(Args, Clone, Debug)]
pub struct LocationArgs {
    #[arg(long)]
    pub city: Option<String>,
    #[arg(long)]
    pub location_id: Option<String>,
    #[arg(long)]
    pub lon: Option<f64>,
    #[arg(long)]
    pub lat: Option<f64>,
}

pub struct ResolvedLocation {
    pub id: String,
    pub coord: String,
}

pub async fn resolve_location(
    args: &LocationArgs,
    client: &QWeatherClient,
) -> Result<ResolvedLocation, QWeatherError> {
    let geo = GeoAPI::new(client);

    if let Some(city) = &args.city {
        let resp = geo.city_lookup(city, None, None, 1, "zh").await?;
        if resp.location.is_empty() {
            return Err(QWeatherError::InvalidParameter(format!(
                "未找到城市: {}",
                city
            )));
        }
        let loc = &resp.location[0];
        return Ok(ResolvedLocation {
            id: loc.id.clone(),
            coord: format!("{}, {}", loc.lon, loc.lat),
        });
    }

    if let Some(id) = &args.location_id {
        let resp = geo.city_lookup(id, None, None, 1, "zh").await?;
        if resp.location.is_empty() {
            return Err(QWeatherError::InvalidParameter(format!(
                "未找到 location ID: {}",
                id
            )));
        }
        let loc = &resp.location[0];
        return Ok(ResolvedLocation {
            id: loc.id.clone(),
            coord: format!("{}, {}", loc.lon, loc.lat),
        });
    }

    if let (Some(lon), Some(lat)) = (args.lon, args.lat) {
        return Ok(ResolvedLocation {
            id: format!("{:.2},{:.2}", lon, lat),
            coord: format!("{:.2},{:.2}", lon, lat),
        });
    }

    Err(QWeatherError::InvalidParameter(
        "必须提供 --city、--location-id 或 --lon + --lat 中的一种定位方式".to_string(),
    ))
}
