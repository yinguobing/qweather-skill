---
name: qweather
version: 0.3.0
description: 查询全球城市实时天气、天气预报、空气质量等信息的技能。当用户询问某个城市的天气、气温、降雨、风向、空气质量时调用。
homepage: https://github.com/yinguobing/qweather-skill
required_env_vars:
  - QWEATHER_KID
  - QWEATHER_PROJECT_ID
  - QWEATHER_PRIVATE_KEY
  - QWEATHER_BASE_URL
  - QWEATHER_GEO_URL
primary_credential: QWEATHER_PRIVATE_KEY
metadata: {"openclaw":{"requires":{"env":["QWEATHER_KID","QWEATHER_PROJECT_ID","QWEATHER_PRIVATE_KEY","QWEATHER_BASE_URL","QWEATHER_GEO_URL"]},"primaryEnv":"QWEATHER_PRIVATE_KEY","homepage":"https://github.com/yinguobing/qweather-skill","install":[{"id":"qweather","kind":"cargo","package":"qweather","bins":["qweather"],"label":"Install qweather via cargo"}]}}
---

# 和风天气查询技能

基于和风天气（QWeather）API，提供全球城市的天气查询能力。

## 安装

使用curl + install.sh安装：
```bash
curl -sSL https://raw.githubusercontent.com/yinguobing/qweather-skill/main/install.sh | bash
```

使用Cargo安装可执行文件：
```bash
cargo install qweather
```


源码安装
```bash
cargo install --git https://github.com/yinguobing/qweather-skill.git
```

## 身份认证

使用 JWT 身份认证，**必须**配置以下环境变量：

```bash
export QWEATHER_KID="YOUR_KID"
export QWEATHER_PROJECT_ID="YOUR_PROJECT_ID"
export QWEATHER_PRIVATE_KEY="/path/to/qweather/private.pem"
export QWEATHER_BASE_URL="https://your-host.qweatherapi.com/v7"
export QWEATHER_GEO_URL="https://your-host.qweatherapi.com/geo/v2"
```

详情可在[和风天气控制台](https://console.qweather.com/project)获取。

## CLI 使用

全局入口为 `qweather`。所有查询都会自动完成城市定位。

### 常用查询

```bash
# 实时天气（默认）
qweather 北京

# 未来N天预报（daily 仅支持 3,7,10,15,30）
qweather 北京 --type daily --days 3

# 逐小时预报（hourly 仅支持 24,72,168）
qweather 北京 --type hourly --hours 24

# 空气质量实时数据
qweather 北京 --type air

# 生活指数
qweather 北京 --type indices --index-days 1d

# 天气预警
qweather 北京 --type warning

# 分钟级降水（仅中国地区，自动获取经纬度）
qweather 北京 --type minutely

# 日出日落（--date 必选，格式 yyyyMMdd，最多未来 60 天）
qweather 北京 --type sun --date 20260416

# 月升月落和月相（--date 必选，格式 yyyyMMdd，最多未来 60 天）
qweather 北京 --type moon --date 20260416

# 太阳高度角（--date 必选；--time 格式 HHmm；--tz 时区偏移如 0800；-alt 海拔高度米）
qweather 北京 --type solar --date 20260416 --time 1200 --tz 0800 --alt 43

# 格点天气（基于经纬度的高分辨率数据）
qweather 北京 --type grid-now
qweather 北京 --type grid-daily --days 3
qweather 北京 --type grid-hourly --hours 24

# 空气质量预报
qweather 北京 --type air-daily --days 5
qweather 北京 --type air-hourly --hours 72
```

### 命令行参数速查

- `--type {now,daily,hourly,air,indices,minutely,warning,sun,moon,solar,grid-now,grid-daily,grid-hourly,air-daily,air-hourly}`
- `--days N`：用于 daily / grid-daily / air-daily
- `--hours N`：用于 hourly / grid-hourly / air-hourly
- `--index-days {1d,3d}`：用于 indices
- `--date yyyyMMdd`：用于 sun / moon / solar（**最多未来 60 天，含今天**）
- `--time HHmm` / `--tz 0800` / `--alt 海拔`：用于 solar
- `--kid` / `--project-id` / `--private-key` / `--base-url` / `--geo-url`：覆盖环境变量

### 参数限制

| 参数 | 适用 `--type` | 限制说明 |
|------|--------------|----------|
| `--days` | `daily` | 仅支持 `3,7,10,15,30` |
| `--days` | `grid-daily` | 仅支持 `3,7` |
| `--days` | `air-daily` | 仅支持 `1,3,5` |
| `--hours` | `hourly` | 仅支持 `24,72,168` |
| `--hours` | `grid-hourly` | 仅支持 `24,72` |
| `--hours` | `air-hourly` | 仅支持 `24,72` |
| `--date` | `sun` / `moon` | 必选，格式 `yyyyMMdd`，**仅未来 60 天内（含今天）** |
| `--date` | `solar` | 必选，格式 `yyyyMMdd` |
| `--time` | `solar` | 必选，格式 `HHmm` |
| `--tz` | `solar` | 必选，时区偏移，如 `0800`、`-0530` |
| `--alt` | `solar` | 必选，海拔高度（米） |
