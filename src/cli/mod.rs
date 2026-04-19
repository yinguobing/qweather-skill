pub mod format;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "qweather", version)]
#[command(about = "和风天气查询")]
pub struct Args {
    pub city: String,

    #[arg(long, hide = true)]
    pub key: Option<String>,

    #[arg(long, help = "JWT 凭据 ID")]
    pub kid: Option<String>,

    #[arg(long, help = "JWT 项目 ID（sub）")]
    pub project_id: Option<String>,

    #[arg(long, help = "JWT Ed25519 私钥内容或 PEM 文件路径")]
    pub private_key: Option<String>,

    #[arg(long, help = "自定义 API Host 的天气数据基础 URL（JWT 必填）")]
    pub base_url: Option<String>,

    #[arg(long, help = "自定义 API Host 的 Geo 基础 URL（JWT 必填）")]
    pub geo_url: Option<String>,

    #[arg(long, value_name = "TYPE", default_value = "now", help = "查询类型")]
    pub r#type: QueryType,

    #[arg(long, default_value_t = 3, help = "预报天数（daily / grid-daily / air-daily）")]
    pub days: i32,

    #[arg(long, default_value_t = 24, help = "预报小时数（hourly / grid-hourly / air-hourly）")]
    pub hours: i32,

    #[arg(long, value_name = "INDEX_DAYS", default_value = "1d", help = "指数预报天数（indices）")]
    pub index_days: String,

    #[arg(long, default_value = "", help = "查询日期 yyyyMMdd（sun / moon / solar）")]
    pub date: String,

    #[arg(long, default_value = "", help = "查询时间 HHmm（solar）")]
    pub time: String,

    #[arg(long, default_value = "", help = "时区偏移如 0800（solar）")]
    pub tz: String,

    #[arg(long, default_value = "", help = "海拔高度米（solar）")]
    pub alt: String,
}

#[derive(Debug, Clone, clap::ValueEnum)]
pub enum QueryType {
    Now,
    Daily,
    Hourly,
    Air,
    Indices,
    Minutely,
    Warning,
    Sun,
    GridNow,
    GridDaily,
    GridHourly,
    AirDaily,
    AirHourly,
    Moon,
    Solar,
}
