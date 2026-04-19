use qweather::api::{
    air::AirAPI, astronomy::AstronomyAPI, geo::GeoAPI, grid_weather::GridWeatherAPI,
    indices::IndicesAPI, minutely::MinutelyAPI, warning::WarningAPI, weather::WeatherAPI,
};
use qweather::cli::format::*;
use qweather::{Config, QWeatherClient};

fn test_config(server_url: &str) -> Config {
    Config::new(
        Some("test-key".to_string()),
        None,
        None,
        None,
        None,
        Some(format!("{}/v7", server_url)),
        Some(format!("{}/v2", server_url)),
    )
    .unwrap()
}

fn test_config_jwt(server_url: &str) -> (Config, String) {
    // Ed25519 test key generated with openssl genpkey -algorithm Ed25519
    let pem = "-----BEGIN PRIVATE KEY-----\nMC4CAQAwBQYDK2VwBCIEIEad3OG2Qkdcdb08+PHiHswWILKnQqJsxUz0Mtt9gmCa\n-----END PRIVATE KEY-----\n";
    let config = Config::new(
        None,
        Some("TEST_KID".to_string()),
        Some("TEST_PROJECT".to_string()),
        Some(pem.to_string()),
        None,
        Some(format!("{}/v7", server_url)),
        Some(format!("{}/v2", server_url)),
    )
    .unwrap();
    (config, pem.to_string())
}

#[tokio::test]
async fn test_weather_now() {
    let mut server = mockito::Server::new_async().await;
    let _m = server
        .mock("GET", "/v7/weather/now")
        .match_query(mockito::Matcher::AllOf(vec![
            mockito::Matcher::UrlEncoded("location".into(), "101010100".into()),
            mockito::Matcher::UrlEncoded("lang".into(), "zh".into()),
            mockito::Matcher::UrlEncoded("key".into(), "test-key".into()),
        ]))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"code":"200","updateTime":"2024-01-01T12:00+08:00","fxLink":"https://www.qweather.com","now":{"obsTime":"2024-01-01T12:00+08:00","temp":"25","feelsLike":"26","icon":"100","text":"晴","wind360":"180","windDir":"南风","windScale":"3","windSpeed":"12","humidity":"45","precip":"0.0","pressure":"1012","vis":"15","cloud":"10","dew":"12"},"refer":{"sources":["QWeather"],"license":["CC BY-SA 4.0"]}}"#)
        .create_async().await;

    let config = test_config(&server.url());
    let client = QWeatherClient::new(config);
    let api = WeatherAPI::new(&client);
    let resp = api.now("101010100", "zh", None).await.unwrap();
    assert_eq!(resp.code, "200");
    assert_eq!(resp.now.temp, "25");
    assert_eq!(resp.now.text, "晴");
}

#[tokio::test]
async fn test_daily_forecast() {
    let mut server = mockito::Server::new_async().await;
    let _m = server
        .mock("GET", "/v7/weather/3d")
        .match_query(mockito::Matcher::Any)
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"code":"200","updateTime":"2024-01-01T12:00+08:00","fxLink":"https://www.qweather.com","daily":[{"fxDate":"2024-01-01","sunrise":"07:36","sunset":"17:01","moonrise":"23:08","moonset":"11:52","moonPhase":"亏凸月","tempMax":"5","tempMin":"-4","iconDay":"100","textDay":"晴","iconNight":"150","textNight":"晴","wind360Day":"270","windDirDay":"西风","windScaleDay":"1-2","windSpeedDay":"7","wind360Night":"0","windDirNight":"北风","windScaleNight":"1-2","windSpeedNight":"6","humidity":"21","precip":"0.0","pressure":"1027","vis":"25","cloud":"0","uvIndex":"2"}],"refer":{"sources":["QWeather"],"license":["CC BY-SA 4.0"]}}"#)
        .create_async().await;

    let config = test_config(&server.url());
    let client = QWeatherClient::new(config);
    let api = WeatherAPI::new(&client);
    let resp = api.daily_forecast("101010100", 3, "zh", None).await.unwrap();
    assert_eq!(resp.code, "200");
    assert_eq!(resp.daily.len(), 1);
    assert_eq!(resp.daily[0].tempMax, "5");
    assert_eq!(resp.daily[0].textDay, "晴");
}

#[tokio::test]
async fn test_hourly_forecast() {
    let mut server = mockito::Server::new_async().await;
    let _m = server
        .mock("GET", "/v7/weather/24h")
        .match_query(mockito::Matcher::Any)
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"code":"200","updateTime":"2024-01-01T12:00+08:00","fxLink":"https://www.qweather.com","hourly":[{"fxTime":"2024-01-01T13:00+08:00","temp":"4","icon":"100","text":"晴","wind360":"270","windDir":"西风","windScale":"1-2","windSpeed":"6","humidity":"20","pop":"0","precip":"0.0","pressure":"1027","cloud":"0","dew":"-14"}],"refer":{"sources":["QWeather"],"license":["CC BY-SA 4.0"]}}"#)
        .create_async().await;

    let config = test_config(&server.url());
    let client = QWeatherClient::new(config);
    let api = WeatherAPI::new(&client);
    let resp = api.hourly_forecast("101010100", 24, "zh", None).await.unwrap();
    assert_eq!(resp.code, "200");
    assert_eq!(resp.hourly.len(), 1);
    assert_eq!(resp.hourly[0].temp, "4");
}

#[test]
fn test_invalid_days() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let config = Config::new(Some("k".to_string()), None, None, None, None, None, None).unwrap();
        let client = QWeatherClient::new(config);
        let api = WeatherAPI::new(&client);
        let result = api.daily_forecast("101010100", 5, "zh", None).await;
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("不支持 days=5"));
    });
}

#[test]
fn test_invalid_hours() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let config = Config::new(Some("k".to_string()), None, None, None, None, None, None).unwrap();
        let client = QWeatherClient::new(config);
        let api = WeatherAPI::new(&client);
        let result = api.hourly_forecast("101010100", 48, "zh", None).await;
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("不支持 hours=48"));
    });
}

#[tokio::test]
async fn test_geo_city_lookup() {
    let mut server = mockito::Server::new_async().await;
    let _m = server
        .mock("GET", "/v2/city/lookup")
        .match_query(mockito::Matcher::Any)
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"code":"200","location":[{"name":"北京","id":"101010100","lat":"39.90","lon":"116.40","adm1":"北京市","adm2":"北京"}]}"#)
        .create_async().await;

    let config = test_config(&server.url());
    let client = QWeatherClient::new(config);
    let api = GeoAPI::new(&client);
    let resp = api.city_lookup("北京", None, None, 1, "zh").await.unwrap();
    assert_eq!(resp.code, "200");
    assert_eq!(resp.location.len(), 1);
    assert_eq!(resp.location[0].name, "北京");
}

#[tokio::test]
async fn test_air_now_v1() {
    let mut server = mockito::Server::new_async().await;
    let _m = server
        .mock("GET", mockito::Matcher::Regex(r#"/airquality/v1/current/\d+\.\d+/\d+\.\d+"#.to_string()))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"metadata":{},"indexes":[{"code":"aqi","name":"AQI","aqi":"50","aqiDisplay":"50","level":"1","category":"优","color":{},"primaryPollutant":{},"health":{}}],"pollutants":[],"stations":[]}"#)
        .create_async().await;

    let config = test_config(&server.url());
    let client = QWeatherClient::new(config);
    let api = AirAPI::new(&client);
    let resp = api.now("116.40,39.90", "zh").await.unwrap();
    match resp {
        qweather::api::air::AirNowResult::V1(r) => {
            assert_eq!(r.indexes.len(), 1);
            assert_eq!(r.indexes[0].category, "优");
        }
        _ => panic!("expected v1 response"),
    }
}

#[tokio::test]
async fn test_astronomy_sun() {
    let mut server = mockito::Server::new_async().await;
    let _m = server
        .mock("GET", "/v7/astronomy/sun")
        .match_query(mockito::Matcher::Any)
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"code":"200","updateTime":"2024-12-01T12:00+08:00","fxLink":"","sunrise":"07:15","sunset":"17:00","refer":{}}"#)
        .create_async().await;

    let config = test_config(&server.url());
    let client = QWeatherClient::new(config);
    let api = AstronomyAPI::new(&client);
    let resp = api.sunrise_sunset("101010100", "20241201", "zh").await.unwrap();
    assert_eq!(resp.code, "200");
    assert_eq!(resp.sunrise, "07:15");
}

#[tokio::test]
async fn test_astronomy_moon() {
    let mut server = mockito::Server::new_async().await;
    let _m = server
        .mock("GET", "/v7/astronomy/moon")
        .match_query(mockito::Matcher::Any)
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"code":"200","updateTime":"2024-12-01T12:00+08:00","fxLink":"","moonrise":"22:00","moonset":"08:00","moonPhase":[{"fxTime":"","value":"","name":"上弦月","illumination":"50","icon":""}],"refer":{}}"#)
        .create_async().await;

    let config = test_config(&server.url());
    let client = QWeatherClient::new(config);
    let api = AstronomyAPI::new(&client);
    let resp = api.moon_phase("101010100", "20241201", "zh").await.unwrap();
    assert_eq!(resp.code, "200");
    assert_eq!(resp.moonPhase[0].name, "上弦月");
}

#[tokio::test]
async fn test_indices() {
    let mut server = mockito::Server::new_async().await;
    let _m = server
        .mock("GET", "/v7/indices/1d")
        .match_query(mockito::Matcher::Any)
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"code":"200","updateTime":"2024-12-01T12:00+08:00","fxLink":"","daily":[{"date":"2024-12-01","type":"1","name":"运动","level":"1","category":"适宜","text":"天气不错，适宜户外运动。"}],"refer":{}}"#)
        .create_async().await;

    let config = test_config(&server.url());
    let client = QWeatherClient::new(config);
    let api = IndicesAPI::new(&client);
    let resp = api.forecast("101010100", "1d", "0", "zh").await.unwrap();
    assert_eq!(resp.code, "200");
    assert_eq!(resp.daily[0].name, "运动");
}

#[tokio::test]
async fn test_minutely() {
    let mut server = mockito::Server::new_async().await;
    let _m = server
        .mock("GET", "/v7/minutely/5m")
        .match_query(mockito::Matcher::Any)
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"code":"200","updateTime":"2024-12-01T12:00+08:00","fxLink":"","summary":"未来2小时无降水","minutely":[{"fxTime":"2024-12-01T12:00+08:00","precip":"0.0","type":"rain"}],"refer":{}}"#)
        .create_async().await;

    let config = test_config(&server.url());
    let client = QWeatherClient::new(config);
    let api = MinutelyAPI::new(&client);
    let resp = api.precipitation("116.40,39.90", "zh").await.unwrap();
    assert_eq!(resp.code, "200");
    assert_eq!(resp.summary, "未来2小时无降水");
}

#[tokio::test]
async fn test_warning() {
    let mut server = mockito::Server::new_async().await;
    let _m = server
        .mock("GET", "/v7/warning/now")
        .match_query(mockito::Matcher::Any)
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"code":"200","updateTime":"2024-12-01T12:00+08:00","fxLink":"","warning":[{"id":"123","sender":"","pubTime":"2024-12-01T10:00+08:00","title":"大风蓝色预警","start":"","end":"","status":"","level":"蓝色","type":"","typeName":"","urgency":"","certainty":"","text":"预计未来24小时有大风天气，请注意防范。"}],"refer":{}}"#)
        .create_async().await;

    let config = test_config(&server.url());
    let client = QWeatherClient::new(config);
    let api = WarningAPI::new(&client);
    let resp = api.now("101010100", "zh").await.unwrap();
    assert_eq!(resp.code, "200");
    assert_eq!(resp.warning[0].title, "大风蓝色预警");
}

#[tokio::test]
async fn test_grid_weather_now() {
    let mut server = mockito::Server::new_async().await;
    let _m = server
        .mock("GET", "/v7/grid-weather/now")
        .match_query(mockito::Matcher::Any)
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"code":"200","updateTime":"2024-12-01T12:00+08:00","fxLink":"","now":{"obsTime":"","temp":"11","icon":"100","text":"晴","wind360":"","windDir":"","windScale":"","windSpeed":"","humidity":"","precip":"","pressure":"","cloud":"","dew":""},"refer":{}}"#)
        .create_async().await;

    let config = test_config(&server.url());
    let client = QWeatherClient::new(config);
    let api = GridWeatherAPI::new(&client);
    let resp = api.now("116.40,39.90", "zh", None).await.unwrap();
    assert_eq!(resp.code, "200");
    assert_eq!(resp.now.text, "晴");
}

#[tokio::test]
async fn test_api_error() {
    let mut server = mockito::Server::new_async().await;
    let _m = server
        .mock("GET", "/v7/weather/now")
        .match_query(mockito::Matcher::Any)
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"code":"401"}"#)
        .create_async().await;

    let config = test_config(&server.url());
    let client = QWeatherClient::new(config);
    let api = WeatherAPI::new(&client);
    let result = api.now("101010100", "zh", None).await;
    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(err.contains("401"));
}

#[tokio::test]
async fn test_jwt_auth() {
    let mut server = mockito::Server::new_async().await;
    let _m = server
        .mock("GET", "/v7/weather/now")
        .match_query(mockito::Matcher::Any)
        .match_header("authorization", mockito::Matcher::Regex(r#"Bearer .+"#.to_string()))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"code":"200","updateTime":"","fxLink":"","now":{"obsTime":"","temp":"20","icon":"","text":"多云","wind360":"","windDir":"","windScale":"","windSpeed":"","humidity":"","precip":"","pressure":"","cloud":"","dew":""},"refer":{}}"#)
        .create_async().await;

    let (config, _) = test_config_jwt(&server.url());
    let client = QWeatherClient::new(config);
    let api = WeatherAPI::new(&client);
    let resp = api.now("101010100", "zh", None).await.unwrap();
    assert_eq!(resp.now.temp, "20");
}

// ============================================================================
// Formatting tests
// ============================================================================

#[test]
fn test_format_now() {
    let weather: qweather::models::WeatherNowResponse = serde_json::from_str(r#"{"code":"200","updateTime":"2024-12-01T12:00+08:00","now":{"temp":"10","text":"晴","feelsLike":"8","windDir":"北","windScale":"3","humidity":"30","precip":"0.0","vis":"10"}}"#).unwrap();
    let out = format_now(&weather);
    assert!(out.contains("晴"));
    assert!(out.contains("10°C"));
}

#[test]
fn test_format_daily() {
    let weather: qweather::models::WeatherDailyResponse = serde_json::from_str(r#"{"code":"200","updateTime":"2024-12-01T12:00+08:00","daily":[{"fxDate":"2024-12-01","textDay":"多云","textNight":"晴","tempMin":"5","tempMax":"15","windDirDay":"东南","windScaleDay":"2"}]}"#).unwrap();
    let out = format_daily(&weather);
    assert!(out.contains("多云/晴"));
    assert!(out.contains("5°C ~ 15°C"));
}

#[test]
fn test_format_hourly() {
    let weather: qweather::models::WeatherHourlyResponse = serde_json::from_str(r#"{"code":"200","updateTime":"2024-12-01T12:00+08:00","hourly":[{"fxTime":"2024-12-01T13:00+08:00","text":"晴","temp":"12","pop":"10"}]}"#).unwrap();
    let out = format_hourly(&weather);
    assert!(out.contains("13:00"));
    assert!(out.contains("晴"));
}

#[test]
fn test_format_air_v7() {
    let air: qweather::models::AirNowResponse = serde_json::from_str(r#"{"code":"200","updateTime":"2024-12-01T12:00+08:00","now":{"aqi":"50","category":"优","primary":"NA","pm2p5":"20","pm10":"40","no2":"10","so2":"5","co":"0.5","o3":"30"},"station":[]}"#).unwrap();
    let out = format_air(&qweather::api::air::AirNowResult::V7(Box::new(air)));
    assert!(out.contains("AQI: 50 (优)"));
}

#[test]
fn test_format_indices() {
    let indices: qweather::models::IndicesResponse = serde_json::from_str(r#"{"code":"200","updateTime":"2024-12-01T12:00+08:00","daily":[{"name":"运动","category":"适宜","text":"天气不错，适宜户外运动。"}]}"#).unwrap();
    let out = format_indices(&indices);
    assert!(out.contains("运动: 适宜"));
}

#[test]
fn test_format_minutely() {
    let m: qweather::models::MinutelyResponse = serde_json::from_str(r#"{"code":"200","updateTime":"2024-12-01T12:00+08:00","summary":"未来2小时无降水","minutely":[{"fxTime":"2024-12-01T12:00+08:00","precip":"0.0","type":"rain"}]}"#).unwrap();
    let out = format_minutely(&m);
    assert!(out.contains("未来2小时无降水"));
}

#[test]
fn test_format_warning() {
    let w: qweather::models::WarningResponse = serde_json::from_str(r#"{"code":"200","updateTime":"2024-12-01T12:00+08:00","warning":[{"level":"蓝色","title":"大风蓝色预警","pubTime":"2024-12-01T10:00+08:00","text":"预计未来24小时有大风天气，请注意防范。"}]}"#).unwrap();
    let out = format_warning(&w);
    assert!(out.contains("大风蓝色预警"));
}

#[test]
fn test_format_astronomy() {
    let a: qweather::models::AstronomySunResponse = serde_json::from_str(r#"{"code":"200","updateTime":"2024-12-01T12:00+08:00","sunrise":"07:15","sunset":"17:00"}"#).unwrap();
    let out = format_astronomy(&a);
    assert!(out.contains("日出: 07:15"));
    assert!(out.contains("日落: 17:00"));
}

#[test]
fn test_format_moon() {
    let a: qweather::models::AstronomyMoonResponse = serde_json::from_str(r#"{"code":"200","updateTime":"2024-12-01T12:00+08:00","moonrise":"22:00","moonset":"08:00","moonPhase":[{"name":"上弦月","illumination":"50"}]}"#).unwrap();
    let out = format_moon(&a);
    assert!(out.contains("上弦月"));
    assert!(out.contains("照明度: 50%"));
}

#[test]
fn test_format_solar() {
    let a: qweather::models::SolarElevationAngleResponse = serde_json::from_str(r#"{"code":"200","solarElevationAngle":"45","solarAzimuthAngle":"180","solarHour":"12:00"}"#).unwrap();
    let out = format_solar(&a);
    assert!(out.contains("太阳高度角: 45°"));
}
