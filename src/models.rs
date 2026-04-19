#![allow(non_snake_case)]

use serde::Deserialize;
use serde_json::Value;

fn default_vec<T>() -> Vec<T> {
    Vec::new()
}

fn default_refer() -> Refer {
    Refer {
        sources: Vec::new(),
        license: Vec::new(),
    }
}

// ============================================================================
// Common
// ============================================================================

#[derive(Debug, Clone, Deserialize, Default)]
pub struct Refer {
    #[serde(default = "default_vec")]
    pub sources: Vec<String>,
    #[serde(default = "default_vec")]
    pub license: Vec<String>,
}

// ============================================================================
// Geo
// ============================================================================

#[derive(Debug, Clone, Deserialize, Default)]
pub struct Location {
    pub name: String,
    pub id: String,
    pub lat: String,
    pub lon: String,
    #[serde(default)]
    pub adm2: String,
    #[serde(default)]
    pub adm1: String,
    #[serde(default)]
    pub country: String,
    #[serde(default)]
    pub tz: String,
    #[serde(default)]
    pub utcOffset: String,
    #[serde(default)]
    pub isDst: String,
    #[serde(default)]
    pub r#type: String,
    #[serde(default)]
    pub rank: String,
    #[serde(default)]
    pub fxLink: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GeoResponse {
    pub code: String,
    #[serde(default = "default_vec")]
    pub location: Vec<Location>,
    #[serde(default = "default_refer")]
    pub refer: Refer,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct POI {
    pub name: String,
    pub id: String,
    pub lat: String,
    pub lon: String,
    #[serde(default)]
    pub adm2: String,
    #[serde(default)]
    pub adm1: String,
    #[serde(default)]
    pub country: String,
    #[serde(default)]
    pub tz: String,
    #[serde(default)]
    pub utcOffset: String,
    #[serde(default)]
    pub isDst: String,
    #[serde(default)]
    pub r#type: String,
    #[serde(default)]
    pub rank: String,
    #[serde(default)]
    pub fxLink: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct POIResponse {
    pub code: String,
    #[serde(default = "default_vec")]
    pub poi: Vec<POI>,
    #[serde(default = "default_refer")]
    pub refer: Refer,
}

// ============================================================================
// Weather
// ============================================================================

#[derive(Debug, Clone, Deserialize, Default)]
pub struct NowWeather {
    #[serde(default)]
    pub obsTime: String,
    #[serde(default)]
    pub temp: String,
    #[serde(default)]
    pub feelsLike: String,
    #[serde(default)]
    pub icon: String,
    #[serde(default)]
    pub text: String,
    #[serde(default)]
    pub wind360: String,
    #[serde(default)]
    pub windDir: String,
    #[serde(default)]
    pub windScale: String,
    #[serde(default)]
    pub windSpeed: String,
    #[serde(default)]
    pub humidity: String,
    #[serde(default)]
    pub precip: String,
    #[serde(default)]
    pub pressure: String,
    #[serde(default)]
    pub vis: String,
    #[serde(default)]
    pub cloud: String,
    #[serde(default)]
    pub dew: String,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct DailyForecast {
    #[serde(default)]
    pub fxDate: String,
    #[serde(default)]
    pub sunrise: String,
    #[serde(default)]
    pub sunset: String,
    #[serde(default)]
    pub moonrise: String,
    #[serde(default)]
    pub moonset: String,
    #[serde(default)]
    pub moonPhase: String,
    #[serde(default)]
    pub tempMax: String,
    #[serde(default)]
    pub tempMin: String,
    #[serde(default)]
    pub iconDay: String,
    #[serde(default)]
    pub textDay: String,
    #[serde(default)]
    pub iconNight: String,
    #[serde(default)]
    pub textNight: String,
    #[serde(default)]
    pub wind360Day: String,
    #[serde(default)]
    pub windDirDay: String,
    #[serde(default)]
    pub windScaleDay: String,
    #[serde(default)]
    pub windSpeedDay: String,
    #[serde(default)]
    pub wind360Night: String,
    #[serde(default)]
    pub windDirNight: String,
    #[serde(default)]
    pub windScaleNight: String,
    #[serde(default)]
    pub windSpeedNight: String,
    #[serde(default)]
    pub humidity: String,
    #[serde(default)]
    pub precip: String,
    #[serde(default)]
    pub pressure: String,
    #[serde(default)]
    pub vis: String,
    #[serde(default)]
    pub cloud: String,
    #[serde(default)]
    pub uvIndex: String,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct HourlyForecast {
    #[serde(default)]
    pub fxTime: String,
    #[serde(default)]
    pub temp: String,
    #[serde(default)]
    pub icon: String,
    #[serde(default)]
    pub text: String,
    #[serde(default)]
    pub wind360: String,
    #[serde(default)]
    pub windDir: String,
    #[serde(default)]
    pub windScale: String,
    #[serde(default)]
    pub windSpeed: String,
    #[serde(default)]
    pub humidity: String,
    #[serde(default)]
    pub pop: String,
    #[serde(default)]
    pub precip: String,
    #[serde(default)]
    pub pressure: String,
    #[serde(default)]
    pub cloud: String,
    #[serde(default)]
    pub dew: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WeatherNowResponse {
    pub code: String,
    #[serde(default)]
    pub updateTime: String,
    #[serde(default)]
    pub fxLink: String,
    #[serde(default)]
    pub now: NowWeather,
    #[serde(default = "default_refer")]
    pub refer: Refer,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WeatherDailyResponse {
    pub code: String,
    #[serde(default)]
    pub updateTime: String,
    #[serde(default)]
    pub fxLink: String,
    #[serde(default = "default_vec")]
    pub daily: Vec<DailyForecast>,
    #[serde(default = "default_refer")]
    pub refer: Refer,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WeatherHourlyResponse {
    pub code: String,
    #[serde(default)]
    pub updateTime: String,
    #[serde(default)]
    pub fxLink: String,
    #[serde(default = "default_vec")]
    pub hourly: Vec<HourlyForecast>,
    #[serde(default = "default_refer")]
    pub refer: Refer,
}

// ============================================================================
// Air Quality
// ============================================================================

#[derive(Debug, Clone, Deserialize, Default)]
pub struct AirNow {
    #[serde(default)]
    pub pubTime: String,
    #[serde(default)]
    pub aqi: String,
    #[serde(default)]
    pub level: String,
    #[serde(default)]
    pub category: String,
    #[serde(default)]
    pub primary: String,
    #[serde(default)]
    pub pm10: String,
    #[serde(default)]
    pub pm2p5: String,
    #[serde(default)]
    pub no2: String,
    #[serde(default)]
    pub so2: String,
    #[serde(default)]
    pub co: String,
    #[serde(default)]
    pub o3: String,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct AirStation {
    #[serde(default)]
    pub pubTime: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub aqi: String,
    #[serde(default)]
    pub level: String,
    #[serde(default)]
    pub category: String,
    #[serde(default)]
    pub primary: String,
    #[serde(default)]
    pub pm10: String,
    #[serde(default)]
    pub pm2p5: String,
    #[serde(default)]
    pub no2: String,
    #[serde(default)]
    pub so2: String,
    #[serde(default)]
    pub co: String,
    #[serde(default)]
    pub o3: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AirNowResponse {
    pub code: String,
    #[serde(default)]
    pub updateTime: String,
    #[serde(default)]
    pub fxLink: String,
    #[serde(default)]
    pub now: AirNow,
    #[serde(default = "default_vec")]
    pub station: Vec<AirStation>,
    #[serde(default = "default_refer")]
    pub refer: Refer,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct DailyIndex {
    #[serde(default)]
    pub date: String,
    #[serde(default)]
    pub r#type: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub level: String,
    #[serde(default)]
    pub category: String,
    #[serde(default)]
    pub text: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct IndicesResponse {
    pub code: String,
    #[serde(default)]
    pub updateTime: String,
    #[serde(default)]
    pub fxLink: String,
    #[serde(default = "default_vec")]
    pub daily: Vec<DailyIndex>,
    #[serde(default = "default_refer")]
    pub refer: Refer,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct MinutelyPrecip {
    #[serde(default)]
    pub fxTime: String,
    #[serde(default)]
    pub precip: String,
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MinutelyResponse {
    pub code: String,
    #[serde(default)]
    pub updateTime: String,
    #[serde(default)]
    pub fxLink: String,
    #[serde(default)]
    pub summary: String,
    #[serde(default = "default_vec")]
    pub minutely: Vec<MinutelyPrecip>,
    #[serde(default = "default_refer")]
    pub refer: Refer,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct WarningItem {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub sender: String,
    #[serde(default)]
    pub pubTime: String,
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub start: String,
    #[serde(default)]
    pub end: String,
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub level: String,
    #[serde(default)]
    pub r#type: String,
    #[serde(default)]
    pub typeName: String,
    #[serde(default)]
    pub urgency: String,
    #[serde(default)]
    pub certainty: String,
    #[serde(default)]
    pub text: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WarningResponse {
    pub code: String,
    #[serde(default)]
    pub updateTime: String,
    #[serde(default)]
    pub fxLink: String,
    #[serde(default = "default_vec")]
    pub warning: Vec<WarningItem>,
    #[serde(default = "default_refer")]
    pub refer: Refer,
}

// ============================================================================
// Astronomy
// ============================================================================

#[derive(Debug, Clone, Deserialize)]
pub struct AstronomySunResponse {
    pub code: String,
    #[serde(default)]
    pub updateTime: String,
    #[serde(default)]
    pub fxLink: String,
    #[serde(default)]
    pub sunrise: String,
    #[serde(default)]
    pub sunset: String,
    #[serde(default = "default_refer")]
    pub refer: Refer,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct MoonPhase {
    #[serde(default)]
    pub fxTime: String,
    #[serde(default)]
    pub value: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub illumination: String,
    #[serde(default)]
    pub icon: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AstronomyMoonResponse {
    pub code: String,
    #[serde(default)]
    pub updateTime: String,
    #[serde(default)]
    pub fxLink: String,
    #[serde(default)]
    pub moonrise: String,
    #[serde(default)]
    pub moonset: String,
    #[serde(default = "default_vec")]
    pub moonPhase: Vec<MoonPhase>,
    #[serde(default = "default_refer")]
    pub refer: Refer,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SolarElevationAngleResponse {
    pub code: String,
    #[serde(default)]
    pub solarElevationAngle: String,
    #[serde(default)]
    pub solarAzimuthAngle: String,
    #[serde(default)]
    pub solarHour: String,
    #[serde(default)]
    pub hourAngle: String,
    #[serde(default = "default_refer")]
    pub refer: Refer,
}

// ============================================================================
// Grid Weather
// ============================================================================

#[derive(Debug, Clone, Deserialize, Default)]
pub struct GridNowWeather {
    #[serde(default)]
    pub obsTime: String,
    #[serde(default)]
    pub temp: String,
    #[serde(default)]
    pub icon: String,
    #[serde(default)]
    pub text: String,
    #[serde(default)]
    pub wind360: String,
    #[serde(default)]
    pub windDir: String,
    #[serde(default)]
    pub windScale: String,
    #[serde(default)]
    pub windSpeed: String,
    #[serde(default)]
    pub humidity: String,
    #[serde(default)]
    pub precip: String,
    #[serde(default)]
    pub pressure: String,
    #[serde(default)]
    pub cloud: String,
    #[serde(default)]
    pub dew: String,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct GridDailyForecast {
    #[serde(default)]
    pub fxDate: String,
    #[serde(default)]
    pub tempMax: String,
    #[serde(default)]
    pub tempMin: String,
    #[serde(default)]
    pub iconDay: String,
    #[serde(default)]
    pub textDay: String,
    #[serde(default)]
    pub iconNight: String,
    #[serde(default)]
    pub textNight: String,
    #[serde(default)]
    pub wind360Day: String,
    #[serde(default)]
    pub windDirDay: String,
    #[serde(default)]
    pub windScaleDay: String,
    #[serde(default)]
    pub windSpeedDay: String,
    #[serde(default)]
    pub wind360Night: String,
    #[serde(default)]
    pub windDirNight: String,
    #[serde(default)]
    pub windScaleNight: String,
    #[serde(default)]
    pub windSpeedNight: String,
    #[serde(default)]
    pub humidity: String,
    #[serde(default)]
    pub precip: String,
    #[serde(default)]
    pub pressure: String,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct GridHourlyForecast {
    #[serde(default)]
    pub fxTime: String,
    #[serde(default)]
    pub temp: String,
    #[serde(default)]
    pub icon: String,
    #[serde(default)]
    pub text: String,
    #[serde(default)]
    pub wind360: String,
    #[serde(default)]
    pub windDir: String,
    #[serde(default)]
    pub windScale: String,
    #[serde(default)]
    pub windSpeed: String,
    #[serde(default)]
    pub humidity: String,
    #[serde(default)]
    pub precip: String,
    #[serde(default)]
    pub pressure: String,
    #[serde(default)]
    pub cloud: String,
    #[serde(default)]
    pub dew: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GridWeatherNowResponse {
    pub code: String,
    #[serde(default)]
    pub updateTime: String,
    #[serde(default)]
    pub fxLink: String,
    #[serde(default)]
    pub now: GridNowWeather,
    #[serde(default = "default_refer")]
    pub refer: Refer,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GridWeatherDailyResponse {
    pub code: String,
    #[serde(default)]
    pub updateTime: String,
    #[serde(default)]
    pub fxLink: String,
    #[serde(default = "default_vec")]
    pub daily: Vec<GridDailyForecast>,
    #[serde(default = "default_refer")]
    pub refer: Refer,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GridWeatherHourlyResponse {
    pub code: String,
    #[serde(default)]
    pub updateTime: String,
    #[serde(default)]
    pub fxLink: String,
    #[serde(default = "default_vec")]
    pub hourly: Vec<GridHourlyForecast>,
    #[serde(default = "default_refer")]
    pub refer: Refer,
}

// ============================================================================
// Air Forecast (v7)
// ============================================================================

#[derive(Debug, Clone, Deserialize, Default)]
pub struct AirHourlyForecast {
    #[serde(default)]
    pub pubTime: String,
    #[serde(default)]
    pub aqi: String,
    #[serde(default)]
    pub level: String,
    #[serde(default)]
    pub category: String,
    #[serde(default)]
    pub primary: String,
    #[serde(default)]
    pub pm10: String,
    #[serde(default)]
    pub pm2p5: String,
    #[serde(default)]
    pub no2: String,
    #[serde(default)]
    pub so2: String,
    #[serde(default)]
    pub co: String,
    #[serde(default)]
    pub o3: String,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct AirDailyForecast {
    #[serde(default)]
    pub pubTime: String,
    #[serde(default)]
    pub aqi: String,
    #[serde(default)]
    pub level: String,
    #[serde(default)]
    pub category: String,
    #[serde(default)]
    pub primary: String,
    #[serde(default)]
    pub pm10: String,
    #[serde(default)]
    pub pm2p5: String,
    #[serde(default)]
    pub no2: String,
    #[serde(default)]
    pub so2: String,
    #[serde(default)]
    pub co: String,
    #[serde(default)]
    pub o3: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AirHourlyResponse {
    pub code: String,
    #[serde(default)]
    pub updateTime: String,
    #[serde(default)]
    pub fxLink: String,
    #[serde(default = "default_vec")]
    pub hourly: Vec<AirHourlyForecast>,
    #[serde(default = "default_refer")]
    pub refer: Refer,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AirDailyResponse {
    pub code: String,
    #[serde(default)]
    pub updateTime: String,
    #[serde(default)]
    pub fxLink: String,
    #[serde(default = "default_vec")]
    pub daily: Vec<AirDailyForecast>,
    #[serde(default = "default_refer")]
    pub refer: Refer,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AirStationResponse {
    pub code: String,
    #[serde(default)]
    pub updateTime: String,
    #[serde(default)]
    pub fxLink: String,
    #[serde(default)]
    pub station: AirStation,
    #[serde(default = "default_refer")]
    pub refer: Refer,
}

// ============================================================================
// Air Quality V1
// ============================================================================

#[derive(Debug, Clone, Deserialize, Default)]
pub struct AirQualityIndex {
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub aqi: Value,
    #[serde(default)]
    pub aqiDisplay: String,
    #[serde(default)]
    pub level: String,
    #[serde(default)]
    pub category: String,
    #[serde(default)]
    pub color: Value,
    #[serde(default)]
    pub primaryPollutant: Value,
    #[serde(default)]
    pub health: Value,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct AirQualityPollutant {
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub fullName: String,
    #[serde(default)]
    pub concentration: Value,
    #[serde(default = "default_vec")]
    pub subIndexes: Vec<Value>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct AirStationRef {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct AirHourV1 {
    #[serde(default)]
    pub forecastTime: String,
    #[serde(default = "default_vec")]
    pub indexes: Vec<AirQualityIndex>,
    #[serde(default = "default_vec")]
    pub pollutants: Vec<AirQualityPollutant>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct AirDayV1 {
    #[serde(default)]
    pub forecastStartTime: String,
    #[serde(default)]
    pub forecastEndTime: String,
    #[serde(default = "default_vec")]
    pub indexes: Vec<AirQualityIndex>,
    #[serde(default = "default_vec")]
    pub pollutants: Vec<AirQualityPollutant>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct AirNowV1Response {
    #[serde(default)]
    pub metadata: Value,
    #[serde(default = "default_vec")]
    pub indexes: Vec<AirQualityIndex>,
    #[serde(default = "default_vec")]
    pub pollutants: Vec<AirQualityPollutant>,
    #[serde(default = "default_vec")]
    pub stations: Vec<AirStationRef>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct AirHourlyV1Response {
    #[serde(default)]
    pub metadata: Value,
    #[serde(default = "default_vec")]
    pub hours: Vec<AirHourV1>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct AirDailyV1Response {
    #[serde(default)]
    pub metadata: Value,
    #[serde(default = "default_vec")]
    pub days: Vec<AirDayV1>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct AirStationV1Response {
    #[serde(default)]
    pub metadata: Value,
    #[serde(default = "default_vec")]
    pub pollutants: Vec<AirQualityPollutant>,
}
