use std::collections::HashMap;

use crate::client::QWeatherClient;
use crate::error::QWeatherError;
use crate::models::IndicesResponse;

pub struct IndicesAPI<'a> {
    client: &'a QWeatherClient,
}

impl<'a> IndicesAPI<'a> {
    pub fn new(client: &'a QWeatherClient) -> Self {
        IndicesAPI { client }
    }

    pub async fn forecast(
        &self,
        location: &str,
        days: &str,
        r#type: &str,
        lang: &str,
    ) -> Result<IndicesResponse, QWeatherError> {
        let mut params: HashMap<&str, String> = HashMap::new();
        params.insert("location", location.to_string());
        params.insert("type", r#type.to_string());
        params.insert("lang", lang.to_string());
        let url = format!("{}/indices/{}", self.client.config.base_url, days);
        self.client.request(reqwest::Method::GET, &url, Some(params)).await
    }
}
