use std::collections::HashMap;

use crate::client::QWeatherClient;
use crate::error::QWeatherError;
use crate::models::{WeatherDailyResponse, WeatherHourlyResponse, WeatherNowResponse};

pub struct WeatherAPI<'a> {
    client: &'a QWeatherClient,
}

impl<'a> WeatherAPI<'a> {
    pub fn new(client: &'a QWeatherClient) -> Self {
        WeatherAPI { client }
    }

    pub async fn now(
        &self,
        location: &str,
        lang: &str,
        unit: Option<&str>,
    ) -> Result<WeatherNowResponse, QWeatherError> {
        let mut params: HashMap<&str, String> = HashMap::new();
        params.insert("location", location.to_string());
        params.insert("lang", lang.to_string());
        if let Some(unit) = unit {
            params.insert("unit", unit.to_string());
        }
        let url = format!("{}/weather/now", self.client.config.base_url);
        self.client
            .request(reqwest::Method::GET, &url, Some(params))
            .await
    }

    pub async fn daily_forecast(
        &self,
        location: &str,
        days: i32,
        lang: &str,
        unit: Option<&str>,
    ) -> Result<WeatherDailyResponse, QWeatherError> {
        if !matches!(days, 3 | 7 | 10 | 15 | 30) {
            return Err(QWeatherError::InvalidParameter(format!(
                "每日天气预报不支持 days={}，仅支持 3, 7, 10, 15, 30",
                days
            )));
        }
        let mut params: HashMap<&str, String> = HashMap::new();
        params.insert("location", location.to_string());
        params.insert("lang", lang.to_string());
        if let Some(unit) = unit {
            params.insert("unit", unit.to_string());
        }
        let url = format!("{}/weather/{}d", self.client.config.base_url, days);
        self.client
            .request(reqwest::Method::GET, &url, Some(params))
            .await
    }

    pub async fn hourly_forecast(
        &self,
        location: &str,
        hours: i32,
        lang: &str,
        unit: Option<&str>,
    ) -> Result<WeatherHourlyResponse, QWeatherError> {
        if !matches!(hours, 24 | 72 | 168) {
            return Err(QWeatherError::InvalidParameter(format!(
                "逐小时天气预报不支持 hours={}，仅支持 24, 72, 168",
                hours
            )));
        }
        let mut params: HashMap<&str, String> = HashMap::new();
        params.insert("location", location.to_string());
        params.insert("lang", lang.to_string());
        if let Some(unit) = unit {
            params.insert("unit", unit.to_string());
        }
        let url = format!("{}/weather/{}h", self.client.config.base_url, hours);
        self.client
            .request(reqwest::Method::GET, &url, Some(params))
            .await
    }
}
