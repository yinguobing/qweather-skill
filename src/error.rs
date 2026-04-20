use std::collections::HashMap;

lazy_static::lazy_static! {
    static ref ERROR_MESSAGES: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("204", "请求成功，但你查询的地区暂时没有你需要的数据。");
        m.insert("400", "请求错误，可能包含错误的请求参数或缺少必备的参数。");
        m.insert("401", "认证失败，请检查 API KEY、JWT 签名或凭据是否正确。");
        m.insert("402", "超过访问次数或余额不足，请充值后再试。");
        m.insert("403", "无访问权限，可能是绑定的 PackageName、BundleID、域名或 API Host 不在允许的列表中。");
        m.insert("404", "查询的数据或地区不存在。");
        m.insert("429", "超过限定的 QPM（每分钟访问次数），请降低请求频率。");
        m.insert("500", "无响应或超时，接口服务异常请稍后重试。");
        m
    };
}

#[derive(thiserror::Error, Debug)]
pub enum QWeatherError {
    #[error("[QWeather API Error {code}] {message}")]
    ApiError { code: String, message: String },

    #[error("配置错误: {0}")]
    ConfigError(String),

    #[error("HTTP 请求错误: {0}")]
    HttpError(#[from] reqwest::Error),

    #[error("JSON 解析错误: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("JWT 错误: {0}")]
    JwtError(String),

    #[error("IO 错误: {0}")]
    IoError(#[from] std::io::Error),

    #[error("参数错误: {0}")]
    InvalidParameter(String),
}

impl QWeatherError {
    pub fn api_error(code: impl Into<String>) -> Self {
        let code = code.into();
        let message = ERROR_MESSAGES
            .get(code.as_str())
            .unwrap_or(&"未知错误")
            .to_string();
        QWeatherError::ApiError { code, message }
    }

    pub fn api_error_with_message(code: impl Into<String>, message: impl Into<String>) -> Self {
        QWeatherError::ApiError {
            code: code.into(),
            message: message.into(),
        }
    }

    pub fn config_error(msg: impl Into<String>) -> Self {
        QWeatherError::ConfigError(msg.into())
    }

    pub fn jwt_error(msg: impl Into<String>) -> Self {
        QWeatherError::JwtError(msg.into())
    }
}
