use std::collections::HashMap;

use crate::client::QWeatherClient;
use crate::error::QWeatherError;
use crate::models::{
    AirDailyResponse, AirDailyV1Response, AirHourlyResponse, AirHourlyV1Response,
    AirNowResponse, AirNowV1Response, AirStationResponse, AirStationV1Response,
};

pub enum AirNowResult {
    V7(Box<AirNowResponse>),
    V1(AirNowV1Response),
}

pub enum AirHourlyResult {
    V7(AirHourlyResponse),
    V1(AirHourlyV1Response),
}

pub enum AirDailyResult {
    V7(AirDailyResponse),
    V1(AirDailyV1Response),
}

pub enum AirStationResult {
    V7(Box<AirStationResponse>),
    V1(AirStationV1Response),
}

pub struct AirAPI<'a> {
    client: &'a QWeatherClient,
}

impl<'a> AirAPI<'a> {
    pub fn new(client: &'a QWeatherClient) -> Self {
        AirAPI { client }
    }

    fn use_v1(&self) -> bool {
        !self.client.config.base_url.starts_with("https://devapi.qweather.com")
    }

    fn parse_coord(location: &str) -> Result<(f64, f64), QWeatherError> {
        let parts: Vec<&str> = location.split(',').collect();
        if parts.len() != 2 {
            return Err(QWeatherError::InvalidParameter(
                "v1 空气质量 location 必须为 'lon,lat' 格式".to_string(),
            ));
        }
        let lon: f64 = parts[0]
            .parse()
            .map_err(|_| QWeatherError::InvalidParameter("无效的经度".to_string()))?;
        let lat: f64 = parts[1]
            .parse()
            .map_err(|_| QWeatherError::InvalidParameter("无效的纬度".to_string()))?;
        Ok((lon, lat))
    }

    pub async fn now(&self, location: &str, lang: &str) -> Result<AirNowResult, QWeatherError> {
        let mut params: HashMap<&str, String> = HashMap::new();
        params.insert("lang", lang.to_string());

        if self.use_v1() {
            let (lon, lat) = Self::parse_coord(location)?;
            let url = format!(
                "{}/airquality/v1/current/{:.2}/{:.2}",
                self.client.config.airquality_url(),
                lat,
                lon
            );
            let resp: AirNowV1Response = self
                .client
                .request(reqwest::Method::GET, &url, Some(params))
                .await?;
            return Ok(AirNowResult::V1(resp));
        }

        params.insert("location", location.to_string());
        let url = format!("{}/air/now", self.client.config.base_url);
        let resp: AirNowResponse = self
            .client
            .request(reqwest::Method::GET, &url, Some(params))
            .await?;
        Ok(AirNowResult::V7(Box::new(resp)))
    }

    pub async fn hourly_forecast(
        &self,
        location: &str,
        hours: i32,
        lang: &str,
    ) -> Result<AirHourlyResult, QWeatherError> {
        if !self.use_v1() && !matches!(hours, 24 | 72) {
            return Err(QWeatherError::InvalidParameter(format!(
                "空气质量小时预报不支持 hours={}，仅支持 24, 72",
                hours
            )));
        }
        let mut params: HashMap<&str, String> = HashMap::new();
        params.insert("lang", lang.to_string());

        if self.use_v1() {
            let (lon, lat) = Self::parse_coord(location)?;
            let url = format!(
                "{}/airquality/v1/hourly/{:.2}/{:.2}",
                self.client.config.airquality_url(),
                lat,
                lon
            );
            let resp: AirHourlyV1Response = self
                .client
                .request(reqwest::Method::GET, &url, Some(params))
                .await?;
            return Ok(AirHourlyResult::V1(resp));
        }

        params.insert("location", location.to_string());
        let url = format!("{}/air/{}h", self.client.config.base_url, hours);
        let resp: AirHourlyResponse = self
            .client
            .request(reqwest::Method::GET, &url, Some(params))
            .await?;
        Ok(AirHourlyResult::V7(resp))
    }

    pub async fn daily_forecast(
        &self,
        location: &str,
        days: i32,
        lang: &str,
    ) -> Result<AirDailyResult, QWeatherError> {
        if !self.use_v1() && !matches!(days, 1 | 3 | 5) {
            return Err(QWeatherError::InvalidParameter(format!(
                "空气质量每日预报不支持 days={}，仅支持 1, 3, 5",
                days
            )));
        }
        let mut params: HashMap<&str, String> = HashMap::new();
        params.insert("lang", lang.to_string());

        if self.use_v1() {
            let (lon, lat) = Self::parse_coord(location)?;
            let url = format!(
                "{}/airquality/v1/daily/{:.2}/{:.2}",
                self.client.config.airquality_url(),
                lat,
                lon
            );
            let resp: AirDailyV1Response = self
                .client
                .request(reqwest::Method::GET, &url, Some(params))
                .await?;
            return Ok(AirDailyResult::V1(resp));
        }

        params.insert("location", location.to_string());
        let url = format!("{}/air/{}d", self.client.config.base_url, days);
        let resp: AirDailyResponse = self
            .client
            .request(reqwest::Method::GET, &url, Some(params))
            .await?;
        Ok(AirDailyResult::V7(resp))
    }

    pub async fn station(
        &self,
        location: &str,
        lang: &str,
    ) -> Result<AirStationResult, QWeatherError> {
        let mut params: HashMap<&str, String> = HashMap::new();
        params.insert("lang", lang.to_string());

        if self.use_v1() {
            let url = format!(
                "{}/airquality/v1/station/{}",
                self.client.config.airquality_url(),
                location
            );
            let resp: AirStationV1Response = self
                .client
                .request(reqwest::Method::GET, &url, Some(params))
                .await?;
            return Ok(AirStationResult::V1(resp));
        }

        params.insert("location", location.to_string());
        let url = format!("{}/air/station", self.client.config.base_url);
        let resp: AirStationResponse = self
            .client
            .request(reqwest::Method::GET, &url, Some(params))
            .await?;
        Ok(AirStationResult::V7(Box::new(resp)))
    }
}
