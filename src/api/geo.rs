use std::collections::HashMap;

use crate::client::QWeatherClient;
use crate::error::QWeatherError;
use crate::models::{GeoResponse, POIResponse};

pub struct GeoAPI<'a> {
    client: &'a QWeatherClient,
}

impl<'a> GeoAPI<'a> {
    pub fn new(client: &'a QWeatherClient) -> Self {
        GeoAPI { client }
    }

    pub async fn city_lookup(
        &self,
        location: &str,
        adm: Option<&str>,
        range: Option<&str>,
        number: i32,
        lang: &str,
    ) -> Result<GeoResponse, QWeatherError> {
        let mut params: HashMap<&str, String> = HashMap::new();
        params.insert("location", location.to_string());
        params.insert("number", number.to_string());
        params.insert("lang", lang.to_string());
        if let Some(adm) = adm {
            params.insert("adm", adm.to_string());
        }
        if let Some(range) = range {
            params.insert("range", range.to_string());
        }
        let url = format!("{}/city/lookup", self.client.config.geo_url);
        self.client
            .request(reqwest::Method::GET, &url, Some(params))
            .await
    }

    pub async fn top_city(
        &self,
        range: &str,
        number: i32,
        lang: &str,
    ) -> Result<GeoResponse, QWeatherError> {
        let mut params: HashMap<&str, String> = HashMap::new();
        params.insert("range", range.to_string());
        params.insert("number", number.to_string());
        params.insert("lang", lang.to_string());
        let url = format!("{}/city/top", self.client.config.geo_url);
        self.client
            .request(reqwest::Method::GET, &url, Some(params))
            .await
    }

    pub async fn poi_lookup(
        &self,
        location: &str,
        r#type: &str,
        city: Option<&str>,
        number: i32,
        lang: &str,
    ) -> Result<POIResponse, QWeatherError> {
        let mut params: HashMap<&str, String> = HashMap::new();
        params.insert("location", location.to_string());
        params.insert("type", r#type.to_string());
        params.insert("number", number.to_string());
        params.insert("lang", lang.to_string());
        if let Some(city) = city {
            params.insert("city", city.to_string());
        }
        let url = format!("{}/poi/lookup", self.client.config.geo_url);
        self.client
            .request(reqwest::Method::GET, &url, Some(params))
            .await
    }

    pub async fn poi_range(
        &self,
        location: &str,
        r#type: &str,
        radius: i32,
        number: i32,
        lang: &str,
    ) -> Result<POIResponse, QWeatherError> {
        let mut params: HashMap<&str, String> = HashMap::new();
        params.insert("location", location.to_string());
        params.insert("type", r#type.to_string());
        params.insert("radius", radius.to_string());
        params.insert("number", number.to_string());
        params.insert("lang", lang.to_string());
        let url = format!("{}/poi/range", self.client.config.geo_url);
        self.client
            .request(reqwest::Method::GET, &url, Some(params))
            .await
    }
}
