use std::collections::HashMap;

use crate::client::QWeatherClient;
use crate::error::QWeatherError;
use crate::models::MinutelyResponse;

pub struct MinutelyAPI<'a> {
    client: &'a QWeatherClient,
}

impl<'a> MinutelyAPI<'a> {
    pub fn new(client: &'a QWeatherClient) -> Self {
        MinutelyAPI { client }
    }

    pub async fn precipitation(
        &self,
        location: &str,
        lang: &str,
    ) -> Result<MinutelyResponse, QWeatherError> {
        let mut params: HashMap<&str, String> = HashMap::new();
        params.insert("location", location.to_string());
        params.insert("lang", lang.to_string());
        let url = format!("{}/minutely/5m", self.client.config.base_url);
        self.client.request(reqwest::Method::GET, &url, Some(params)).await
    }
}
