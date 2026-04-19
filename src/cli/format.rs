use crate::api::air::{AirDailyResult, AirHourlyResult, AirNowResult};
use crate::models::{
    AstronomyMoonResponse, AstronomySunResponse, GridWeatherDailyResponse,
    GridWeatherHourlyResponse, GridWeatherNowResponse, IndicesResponse, MinutelyResponse,
    SolarElevationAngleResponse, WarningResponse, WeatherDailyResponse, WeatherHourlyResponse,
    WeatherNowResponse,
};

pub fn format_now(weather: &WeatherNowResponse) -> String {
    let now = &weather.now;
    if now.text.is_empty() && now.temp.is_empty() {
        return "[实时天气] 暂无数据\n".to_string();
    }
    let mut lines = vec![
        format!("[{}] 实时天气", weather.updateTime),
        format!("天气: {}", now.text),
        format!("温度: {}°C", now.temp),
    ];
    if !now.feelsLike.is_empty() {
        lines.push(format!("体感温度: {}°C", now.feelsLike));
    }
    if !now.windDir.is_empty() || !now.windScale.is_empty() {
        lines.push(format!("风向: {} {}级", now.windDir, now.windScale));
    }
    if !now.humidity.is_empty() {
        lines.push(format!("湿度: {}%", now.humidity));
    }
    if !now.precip.is_empty() {
        lines.push(format!("降水量: {}mm", now.precip));
    }
    if !now.vis.is_empty() {
        lines.push(format!("能见度: {}km", now.vis));
    }
    lines.join("\n") + "\n"
}

pub fn format_daily(weather: &WeatherDailyResponse) -> String {
    let mut lines = vec![format!(
        "[{}] 未来{}天预报",
        weather.updateTime,
        weather.daily.len()
    )];
    for day in &weather.daily {
        lines.push(format!(
            "{}: {}/{}, {}°C ~ {}°C, {} {}级",
            day.fxDate,
            day.textDay,
            day.textNight,
            day.tempMin,
            day.tempMax,
            day.windDirDay,
            day.windScaleDay
        ));
    }
    lines.join("\n") + "\n"
}

pub fn format_hourly(weather: &WeatherHourlyResponse) -> String {
    let mut lines = vec![format!(
        "[{}] 未来{}小时预报",
        weather.updateTime,
        weather.hourly.len()
    )];
    for hour in &weather.hourly {
        let time_str = if hour.fxTime.len() >= 16 {
            &hour.fxTime[11..16]
        } else {
            &hour.fxTime
        };
        lines.push(format!(
            "{}: {}, {}°C, 降水概率{}%",
            time_str, hour.text, hour.temp, hour.pop
        ));
    }
    lines.join("\n") + "\n"
}

pub fn format_air(air: &AirNowResult) -> String {
    match air {
        AirNowResult::V7(a) => {
            let now = &a.now;
            let mut lines = vec![
                format!("[{}] 实时空气质量", a.updateTime),
                format!("AQI: {} ({})", now.aqi, now.category),
                format!("首要污染物: {}", now.primary),
                format!("PM2.5: {}  PM10: {}", now.pm2p5, now.pm10),
                format!("NO₂: {}  SO₂: {}", now.no2, now.so2),
                format!("CO: {}  O₃: {}", now.co, now.o3),
            ];
            if !a.station.is_empty() {
                lines.push(format!("监测站数: {}", a.station.len()));
            }
            lines.join("\n") + "\n"
        }
        AirNowResult::V1(a) => {
            let mut lines = vec!["[实时空气质量]".to_string()];
            for idx in &a.indexes {
                let aqi = idx.aqi.as_str().unwrap_or("-");
                lines.push(format!("{}: {} ({})", idx.name, aqi, idx.category));
            }
            if !a.pollutants.is_empty() {
                lines.push("污染物浓度:".to_string());
                for p in &a.pollutants {
                    let val = p
                        .concentration
                        .get("value")
                        .and_then(|v| v.as_str())
                        .unwrap_or("");
                    let unit = p
                        .concentration
                        .get("unit")
                        .and_then(|v| v.as_str())
                        .unwrap_or("");
                    lines.push(format!("  {}: {} {}", p.name, val, unit));
                }
            }
            if !a.stations.is_empty() {
                let names: Vec<&str> = a.stations.iter().map(|s| s.name.as_str()).collect();
                lines.push(format!("监测站: {}", names.join(", ")));
            }
            lines.join("\n") + "\n"
        }
    }
}

pub fn format_indices(indices: &IndicesResponse) -> String {
    let mut lines = vec![format!("[{}] 天气生活指数", indices.updateTime)];
    for idx in &indices.daily {
        lines.push(format!("{}: {} - {}", idx.name, idx.category, idx.text));
    }
    lines.join("\n") + "\n"
}

pub fn format_minutely(m: &MinutelyResponse) -> String {
    let mut lines = vec![
        format!("[{}] 分钟级降水", m.updateTime),
        format!("摘要: {}", m.summary),
    ];
    for item in m.minutely.iter().take(6) {
        let time_str = if item.fxTime.len() >= 16 {
            &item.fxTime[11..16]
        } else {
            &item.fxTime
        };
        lines.push(format!(
            "{}: 降水量 {}mm  ({})",
            time_str, item.precip, item.r#type
        ));
    }
    lines.join("\n") + "\n"
}

pub fn format_warning(w: &WarningResponse) -> String {
    let mut lines = vec![format!("[{}] 实时天气预警", w.updateTime)];
    if w.warning.is_empty() {
        lines.push("当前无生效中的天气预警。".to_string());
        return lines.join("\n") + "\n";
    }
    for item in &w.warning {
        let text_preview: String = item.text.chars().take(80).collect();
        lines.push(format!(
            "[{}] {}\n  发布时间: {}\n  内容: {}...",
            item.level, item.title, item.pubTime, text_preview
        ));
    }
    lines.join("\n") + "\n"
}

pub fn format_astronomy(a: &AstronomySunResponse) -> String {
    let sunrise = if a.sunrise.is_empty() {
        "无日出（可能是极昼/极夜）"
    } else {
        &a.sunrise
    };
    let sunset = if a.sunset.is_empty() {
        "无日落（可能是极昼/极夜）"
    } else {
        &a.sunset
    };
    format!(
        "[{}] 日出日落\n日出: {}\n日落: {}\n",
        a.updateTime, sunrise, sunset
    )
}

pub fn format_moon(a: &AstronomyMoonResponse) -> String {
    let moon_name = a
        .moonPhase
        .first()
        .map(|m| m.name.as_str())
        .unwrap_or("无数据");
    let illumination = a.moonPhase.first().and_then(|m| {
        if m.illumination.is_empty() {
            None
        } else {
            Some(m.illumination.as_str())
        }
    });
    let mut lines = vec![
        format!("[{}] 月升月落和月相", a.updateTime),
        format!(
            "月升: {}",
            if a.moonrise.is_empty() {
                "无"
            } else {
                &a.moonrise
            }
        ),
        format!(
            "月落: {}",
            if a.moonset.is_empty() {
                "无"
            } else {
                &a.moonset
            }
        ),
        format!("当前月相: {}", moon_name),
    ];
    if let Some(ill) = illumination {
        lines.push(format!("照明度: {}%", ill));
    }
    lines.join("\n") + "\n"
}

pub fn format_solar(a: &SolarElevationAngleResponse) -> String {
    format!(
        "太阳高度角: {}°\n太阳方位角: {}°\n太阳时: {}\n",
        a.solarElevationAngle, a.solarAzimuthAngle, a.solarHour
    )
}

pub fn format_air_daily(resp: &AirDailyResult) -> String {
    match resp {
        AirDailyResult::V7(a) => {
            let mut lines = vec![format!("[{}] 空气质量每日预报", a.updateTime)];
            for day in &a.daily {
                let date = if day.pubTime.len() >= 10 {
                    &day.pubTime[..10]
                } else {
                    &day.pubTime
                };
                lines.push(format!(
                    "{}: AQI {} ({}), 首要污染物 {}",
                    date, day.aqi, day.category, day.primary
                ));
            }
            lines.join("\n") + "\n"
        }
        AirDailyResult::V1(a) => {
            let mut lines = vec!["[空气质量每日预报]".to_string()];
            for day in &a.days {
                let start = if day.forecastStartTime.len() >= 10 {
                    &day.forecastStartTime[..10]
                } else {
                    &day.forecastStartTime
                };
                let idx = day.indexes.first();
                let aqi = idx.and_then(|i| i.aqi.as_str()).unwrap_or("-");
                let cat = idx.map(|i| i.category.as_str()).unwrap_or("-");
                let pol = idx
                    .and_then(|i| i.primaryPollutant.get("name"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("-");
                lines.push(format!(
                    "{}: AQI {} ({}), 首要污染物 {}",
                    start, aqi, cat, pol
                ));
            }
            lines.join("\n") + "\n"
        }
    }
}

pub fn format_air_hourly(resp: &AirHourlyResult) -> String {
    match resp {
        AirHourlyResult::V7(a) => {
            let mut lines = vec![format!("[{}] 空气质量小时预报", a.updateTime)];
            for hour in &a.hourly {
                let time_str = if hour.pubTime.len() >= 16 {
                    &hour.pubTime[11..16]
                } else {
                    &hour.pubTime
                };
                lines.push(format!(
                    "{}: AQI {} ({})",
                    time_str, hour.aqi, hour.category
                ));
            }
            lines.join("\n") + "\n"
        }
        AirHourlyResult::V1(a) => {
            let mut lines = vec!["[空气质量小时预报]".to_string()];
            for hour in &a.hours {
                let time_str = if hour.forecastTime.len() >= 16 {
                    &hour.forecastTime[11..16]
                } else {
                    &hour.forecastTime
                };
                let idx = hour.indexes.first();
                let aqi = idx.and_then(|i| i.aqi.as_str()).unwrap_or("-");
                let cat = idx.map(|i| i.category.as_str()).unwrap_or("-");
                lines.push(format!("{}: AQI {} ({})", time_str, aqi, cat));
            }
            lines.join("\n") + "\n"
        }
    }
}

// Grid weather formatting reuses weather formatters since structures are similar
pub fn format_grid_now(weather: &GridWeatherNowResponse) -> String {
    let now = &weather.now;
    if now.text.is_empty() && now.temp.is_empty() {
        return "[实时天气] 暂无数据\n".to_string();
    }
    let mut lines = vec![
        format!("[{}] 实时天气", weather.updateTime),
        format!("天气: {}", now.text),
        format!("温度: {}°C", now.temp),
    ];
    if !now.windDir.is_empty() || !now.windScale.is_empty() {
        lines.push(format!("风向: {} {}级", now.windDir, now.windScale));
    }
    if !now.humidity.is_empty() {
        lines.push(format!("湿度: {}%", now.humidity));
    }
    lines.join("\n") + "\n"
}

pub fn format_grid_daily(weather: &GridWeatherDailyResponse) -> String {
    let mut lines = vec![format!(
        "[{}] 未来{}天预报",
        weather.updateTime,
        weather.daily.len()
    )];
    for day in &weather.daily {
        lines.push(format!(
            "{}: {}/{}, {}°C ~ {}°C, {} {}级",
            day.fxDate,
            day.textDay,
            day.textNight,
            day.tempMin,
            day.tempMax,
            day.windDirDay,
            day.windScaleDay
        ));
    }
    lines.join("\n") + "\n"
}

pub fn format_grid_hourly(weather: &GridWeatherHourlyResponse) -> String {
    let mut lines = vec![format!(
        "[{}] 未来{}小时预报",
        weather.updateTime,
        weather.hourly.len()
    )];
    for hour in &weather.hourly {
        let time_str = if hour.fxTime.len() >= 16 {
            &hour.fxTime[11..16]
        } else {
            &hour.fxTime
        };
        lines.push(format!("{}: {}, {}°C", time_str, hour.text, hour.temp));
    }
    lines.join("\n") + "\n"
}
