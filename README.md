# qweather

专门为龙虾（OpenClaw）开发的和风天气查询技能。

## 功能特性

基于 [和风天气 API](https://dev.qweather.com/)，提供以下能力：

- 城市定位：城市搜索、热门城市、POI 搜索
- 天气预报：实时天气、每日预报、逐小时预报
- 格点天气：基于经纬度的高分辨率（3~5公里）天气数据
- 空气质量：实时空气质量、小时/每日预报、监测站数据
- 生活指数：穿衣、洗车、运动、紫外线等指数预报
- 分钟级降水：未来2小时每5分钟降水预报（中国地区）
- 天气预警：台风、暴雨、高温等实时预警信息
- 天文数据：日出日落、月升月落、月相、太阳高度角

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

使用JWT身份认证方式。可以通过环境变量或者CLI参数的方式设置：

环境变量
```bash
export QWEATHER_KID="YOUR_KID"
export QWEATHER_PROJECT_ID="YOUR_PROJECT_ID"
export QWEATHER_PRIVATE_KEY="/path/to/qweather/private.pem"
export QWEATHER_BASE_URL="https://your-host.qweatherapi.com/v7"
export QWEATHER_GEO_URL="https://your-host.qweatherapi.com/geo/v2"
```

CLI参数，会覆盖环境变量。
```bash
qweather --kid YOUR_KID \
  --project-id YOUR_PROJECT_ID \
  --private-key /path/to/qweather/private.pem \
  --base-url "https://<YOUR_HOST>.qweatherapi.com/v7" \
  --geo-url "https://<YOUR_HOST>.qweatherapi.com/geo/v2"
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
