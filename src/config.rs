use std::env;
use std::path::Path;

use crate::error::QWeatherError;

const DEFAULT_BASE_URL: &str = "https://devapi.qweather.com/v7";
const DEFAULT_GEO_URL: &str = "https://geoapi.qweather.com/v2";

#[derive(Debug, Clone)]
pub struct Config {
    pub api_key: Option<String>,
    pub kid: Option<String>,
    pub project_id: Option<String>,
    pub private_key: Option<String>,
    pub token_ttl: i64,
    pub base_url: String,
    pub geo_url: String,
}

impl Config {
    pub fn new(
        api_key: Option<String>,
        kid: Option<String>,
        project_id: Option<String>,
        private_key: Option<String>,
        token_ttl: Option<i64>,
        base_url: Option<String>,
        geo_url: Option<String>,
    ) -> Result<Self, QWeatherError> {
        let mut config = Config {
            api_key: api_key.or_else(|| env::var("QWEATHER_API_KEY").ok()),
            kid: kid.or_else(|| env::var("QWEATHER_KID").ok()),
            project_id: project_id.or_else(|| env::var("QWEATHER_PROJECT_ID").ok()),
            private_key: private_key.or_else(|| env::var("QWEATHER_PRIVATE_KEY").ok()),
            token_ttl: token_ttl.unwrap_or(900),
            base_url: base_url
                .or_else(|| env::var("QWEATHER_BASE_URL").ok())
                .unwrap_or_else(|| DEFAULT_BASE_URL.to_string()),
            geo_url: geo_url
                .or_else(|| env::var("QWEATHER_GEO_URL").ok())
                .unwrap_or_else(|| DEFAULT_GEO_URL.to_string()),
        };

        if let Some(ref pk) = config.private_key {
            let mut pk = pk.clone();
            if pk.contains("\\n") {
                pk = pk.replace("\\n", "\n");
            }
            let expanded = shellexpand::tilde(&pk);
            let path = Path::new(expanded.as_ref());
            if !pk.starts_with("---") && path.is_file() {
                pk = std::fs::read_to_string(path)?;
            }
            config.private_key = Some(pk);
        }

        config.base_url = config.base_url.trim_end_matches('/').to_string();
        config.geo_url = config.geo_url.trim_end_matches('/').to_string();

        let has_jwt = config.kid.as_ref().map(|s| !s.is_empty()).unwrap_or(false)
            && config
                .project_id
                .as_ref()
                .map(|s| !s.is_empty())
                .unwrap_or(false)
            && config
                .private_key
                .as_ref()
                .map(|s| !s.is_empty())
                .unwrap_or(false);
        let has_api_key = config.api_key.is_some();

        if !has_jwt && !has_api_key {
            return Err(QWeatherError::config_error(
                "必须提供 JWT 凭据（kid / project_id / private_key）或 API KEY（api_key）。\
                 可通过环境变量 QWEATHER_KID、QWEATHER_PROJECT_ID、QWEATHER_PRIVATE_KEY 或 QWEATHER_API_KEY 设置。",
            ));
        }

        if has_jwt && config.using_default_public_host() {
            return Err(QWeatherError::config_error(
                "使用 JWT 身份认证时，不能使用默认的公共 API 域名。\
                 请在和风天气控制台查看你的自定义 API Host，并通过 base_url 和 geo_url 参数进行配置。\
                 例如：base_url='https://<your-host>.qweatherapi.com/v7', geo_url='https://<your-host>.qweatherapi.com/geo/v2'",
            ));
        }

        Ok(config)
    }

    pub fn use_jwt(&self) -> bool {
        self.kid.as_ref().map(|s| !s.is_empty()).unwrap_or(false)
            && self
                .project_id
                .as_ref()
                .map(|s| !s.is_empty())
                .unwrap_or(false)
            && self
                .private_key
                .as_ref()
                .map(|s| !s.is_empty())
                .unwrap_or(false)
    }

    pub fn airquality_url(&self) -> String {
        let mut url = self.base_url.clone();
        if url.ends_with("/v7") {
            url.truncate(url.len() - 3);
        }
        url
    }

    fn using_default_public_host(&self) -> bool {
        self.base_url.starts_with("https://devapi.qweather.com")
            || self.geo_url.starts_with("https://geoapi.qweather.com")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_from_param() {
        let config = Config::new(
            Some("param-key".to_string()),
            Some(String::new()),
            Some(String::new()),
            Some(String::new()),
            None,
            Some("https://devapi.qweather.com/v7".to_string()),
            Some("https://geoapi.qweather.com/v2".to_string()),
        )
        .unwrap();
        assert_eq!(config.api_key, Some("param-key".to_string()));
        assert!(!config.use_jwt());
        assert_eq!(config.base_url, "https://devapi.qweather.com/v7");
        assert_eq!(config.geo_url, "https://geoapi.qweather.com/v2");
    }

    #[test]
    fn test_config_custom_urls() {
        let config = Config::new(
            Some("key".to_string()),
            Some(String::new()),
            Some(String::new()),
            Some(String::new()),
            None,
            Some("https://api.qweather.com/v7/".to_string()),
            Some("https://geoapi.qweather.com/v2/".to_string()),
        )
        .unwrap();
        assert_eq!(config.base_url, "https://api.qweather.com/v7");
        assert_eq!(config.geo_url, "https://geoapi.qweather.com/v2");
    }

    #[test]
    fn test_config_jwt() {
        let pem = "-----BEGIN PRIVATE KEY-----\nMC4CAQAwBQYDK2VwBCIEIEad3OG2Qkdcdb08+PHiHswWILKnQqJsxUz0Mtt9gmCa\n-----END PRIVATE KEY-----\n";
        let config = Config::new(
            None,
            Some("TEST_KID".to_string()),
            Some("TEST_PROJECT".to_string()),
            Some(pem.to_string()),
            None,
            Some("https://custom.qweatherapi.com/v7".to_string()),
            Some("https://custom.qweatherapi.com/geo/v2".to_string()),
        )
        .unwrap();
        assert!(config.use_jwt());
        assert_eq!(config.kid, Some("TEST_KID".to_string()));
        assert_eq!(config.project_id, Some("TEST_PROJECT".to_string()));
    }

    #[test]
    fn test_config_jwt_default_host_error() {
        let pem = "-----BEGIN PRIVATE KEY-----\nMC4CAQAwBQYDK2VwBCIEIEad3OG2Qkdcdb08+PHiHswWILKnQqJsxUz0Mtt9gmCa\n-----END PRIVATE KEY-----\n";
        let result = Config::new(
            None,
            Some("TEST_KID".to_string()),
            Some("TEST_PROJECT".to_string()),
            Some(pem.to_string()),
            None,
            Some("https://devapi.qweather.com/v7".to_string()),
            Some("https://geoapi.qweather.com/v2".to_string()),
        );
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("不能使用默认的公共 API 域名"));
    }

    #[test]
    fn test_config_missing_key() {
        let result = Config::new(
            None,
            Some(String::new()),
            Some(String::new()),
            Some(String::new()),
            None,
            None,
            None,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_airquality_url() {
        let config = Config::new(
            Some("k".to_string()),
            Some(String::new()),
            Some(String::new()),
            Some(String::new()),
            None,
            Some("https://devapi.qweather.com/v7".to_string()),
            Some("https://geoapi.qweather.com/v2".to_string()),
        )
        .unwrap();
        assert_eq!(config.airquality_url(), "https://devapi.qweather.com");

        let config2 = Config::new(
            Some("k".to_string()),
            Some(String::new()),
            Some(String::new()),
            Some(String::new()),
            None,
            Some("https://custom.qweatherapi.com/v7".to_string()),
            None,
        )
        .unwrap();
        assert_eq!(config2.airquality_url(), "https://custom.qweatherapi.com");
    }
}
