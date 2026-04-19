use std::collections::HashMap;

use crate::client::QWeatherClient;
use crate::error::QWeatherError;
use crate::models::WarningResponse;

pub struct WarningAPI<'a> {
    client: &'a QWeatherClient,
}

impl<'a> WarningAPI<'a> {
    pub fn new(client: &'a QWeatherClient) -> Self {
        WarningAPI { client }
    }

    pub async fn now(&self, location: &str, lang: &str) -> Result<WarningResponse, QWeatherError> {
        let mut params: HashMap<&str, String> = HashMap::new();
        params.insert("location", location.to_string());
        params.insert("lang", lang.to_string());
        let url = format!("{}/warning/now", self.client.config.base_url);
        self.client
            .request(reqwest::Method::GET, &url, Some(params))
            .await
    }
}
