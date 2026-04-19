use std::collections::HashMap;

use crate::client::QWeatherClient;
use crate::error::QWeatherError;
use crate::models::{AstronomyMoonResponse, AstronomySunResponse, SolarElevationAngleResponse};

pub struct AstronomyAPI<'a> {
    client: &'a QWeatherClient,
}

impl<'a> AstronomyAPI<'a> {
    pub fn new(client: &'a QWeatherClient) -> Self {
        AstronomyAPI { client }
    }

    pub async fn sunrise_sunset(
        &self,
        location: &str,
        date: &str,
        lang: &str,
    ) -> Result<AstronomySunResponse, QWeatherError> {
        let mut params: HashMap<&str, String> = HashMap::new();
        params.insert("location", location.to_string());
        params.insert("date", date.to_string());
        params.insert("lang", lang.to_string());
        let url = format!("{}/astronomy/sun", self.client.config.base_url);
        self.client
            .request(reqwest::Method::GET, &url, Some(params))
            .await
    }

    pub async fn moon_phase(
        &self,
        location: &str,
        date: &str,
        lang: &str,
    ) -> Result<AstronomyMoonResponse, QWeatherError> {
        let mut params: HashMap<&str, String> = HashMap::new();
        params.insert("location", location.to_string());
        params.insert("date", date.to_string());
        params.insert("lang", lang.to_string());
        let url = format!("{}/astronomy/moon", self.client.config.base_url);
        self.client
            .request(reqwest::Method::GET, &url, Some(params))
            .await
    }

    pub async fn solar_elevation_angle(
        &self,
        location: &str,
        date: &str,
        time: &str,
        tz: &str,
        alt: &str,
        lang: &str,
    ) -> Result<SolarElevationAngleResponse, QWeatherError> {
        let mut params: HashMap<&str, String> = HashMap::new();
        params.insert("location", location.to_string());
        params.insert("date", date.to_string());
        params.insert("time", time.to_string());
        params.insert("tz", tz.to_string());
        params.insert("alt", alt.to_string());
        params.insert("lang", lang.to_string());
        let url = format!(
            "{}/astronomy/solar-elevation-angle",
            self.client.config.base_url
        );
        self.client
            .request(reqwest::Method::GET, &url, Some(params))
            .await
    }
}
