use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

use serde::Serialize;

use crate::config::Config;
use crate::error::QWeatherError;

#[derive(Debug, Clone)]
pub struct QWeatherClient {
    pub config: Config,
    client: reqwest::Client,
}

#[derive(Serialize)]
struct Claims {
    sub: String,
    iat: i64,
    exp: i64,
}

impl QWeatherClient {
    pub fn new(config: Config) -> Self {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .gzip(true)
            .build()
            .expect("构建 HTTP client 失败");
        QWeatherClient { config, client }
    }

    fn generate_jwt(&self) -> Result<String, QWeatherError> {
        let config = &self.config;
        let kid = config
            .kid
            .as_ref()
            .ok_or_else(|| QWeatherError::jwt_error("缺少 kid"))?;
        let project_id = config
            .project_id
            .as_ref()
            .ok_or_else(|| QWeatherError::jwt_error("缺少 project_id"))?;
        let private_key = config
            .private_key
            .as_ref()
            .ok_or_else(|| QWeatherError::jwt_error("缺少 private_key"))?;

        let iat = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64
            - 30;
        let exp = iat + config.token_ttl;

        let claims = Claims {
            sub: project_id.clone(),
            iat,
            exp,
        };

        let mut header = jsonwebtoken::Header::new(jsonwebtoken::Algorithm::EdDSA);
        header.kid = Some(kid.clone());

        let encoding_key = jsonwebtoken::EncodingKey::from_ed_pem(private_key.as_bytes())
            .map_err(|e| QWeatherError::jwt_error(format!("Ed25519 PEM 解析失败: {}", e)))?;

        jsonwebtoken::encode(&header, &claims, &encoding_key)
            .map_err(|e| QWeatherError::jwt_error(format!("JWT 编码失败: {}", e)))
    }

    pub async fn request<T: serde::de::DeserializeOwned>(
        &self,
        method: reqwest::Method,
        url: &str,
        params: Option<HashMap<&str, String>>,
    ) -> Result<T, QWeatherError> {
        let mut query = params.unwrap_or_default();
        query.entry("lang").or_insert_with(|| "zh".to_string());

        let mut req = self
            .client
            .request(method, url)
            .header("Accept-Encoding", "gzip");

        if self.config.use_jwt() {
            let token = self.generate_jwt()?;
            req = req.header("Authorization", format!("Bearer {}", token));
        } else {
            let key = self
                .config
                .api_key
                .as_ref()
                .ok_or_else(|| QWeatherError::config_error("缺少 API KEY"))?;
            query.insert("key", key.clone());
        }

        let response = req.query(&query).send().await?;
        let status = response.status();

        // 处理 v2 错误码（非 2xx HTTP status）
        if !status.is_success() {
            let body = response.text().await.unwrap_or_default();
            if let Ok(data) = serde_json::from_str::<serde_json::Value>(&body) {
                if let Some(error) = data.get("error") {
                    let detail = error.get("detail").and_then(|v| v.as_str()).unwrap_or("");
                    let title = error.get("title").and_then(|v| v.as_str()).unwrap_or("");
                    let message = if !detail.is_empty() {
                        detail.to_string()
                    } else if !title.is_empty() {
                        title.to_string()
                    } else {
                        format!("HTTP {}", status.as_u16())
                    };
                    return Err(QWeatherError::api_error_with_message(
                        status.as_u16().to_string(),
                        message,
                    ));
                }
            }
            return Err(QWeatherError::api_error(status.as_u16().to_string()));
        }

        // 处理 v1 错误码（HTTP 2xx，但 JSON code 非 "200"）
        let data: serde_json::Value = response.json().await?;
        if let Some(code) = data.get("code") {
            let code_str = code.as_str().unwrap_or("");
            if code_str != "200" {
                return Err(QWeatherError::api_error(code_str));
            }
        }

        let parsed: T = serde_json::from_value(data)?;
        Ok(parsed)
    }
}
