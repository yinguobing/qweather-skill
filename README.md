# qw

专门为龙虾（OpenClaw）开发的和风天气查询技能。二进制入口为 `qw`。

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
安装后二进制名为 `qw`。


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
qw --kid YOUR_KID \
  --project-id YOUR_PROJECT_ID \
  --private-key /path/to/qweather/private.pem \
  --base-url "https://<YOUR_HOST>.qweatherapi.com/v7" \
  --geo-url "https://<YOUR_HOST>.qweatherapi.com/geo/v2"
```

详情可在[和风天气控制台](https://console.qweather.com/project)获取。

## CLI 使用

全局入口为 `qw`。通过命令+子命令提供具体功能。

### 常用查询

```bash
# 地理位置
qw geo city-lookup 北京
qw geo top-city --range cn
qw geo poi-lookup 故宫 --type scenic
qw geo poi-range 116.40,39.90 --type scenic --radius 10

# 实时天气
qw weather now --city 北京

# 未来N天预报（daily 仅支持 3,7,10,15,30）
qw weather daily --city 北京
qw weather daily 7 --city 北京

# 逐小时预报（hourly 仅支持 24,72,168）
qw weather hourly --city 北京
qw weather hourly 72 --city 北京

# 格点天气（基于经纬度的高分辨率数据）
qw grid now --lon 116.40 --lat 39.90
qw grid daily --lon 116.40 --lat 39.90
qw grid daily 7 --lon 116.40 --lat 39.90
qw grid hourly --lon 116.40 --lat 39.90

# 分钟级降水（仅中国地区）
qw precipitation --lon 116.40 --lat 39.90

# 生活指数
qw indices --city 北京
qw indices --city 北京 --days 3d

# 天气预警
qw warning --city 北京

# 空气质量
qw air now --city 北京
qw air daily --city 北京
qw air daily 5 --city 北京
qw air hourly --city 北京
qw air station --city 北京

# 日出日落（--date 必选，格式 yyyyMMdd，最多未来 60 天）
qw sun --city 北京 --date 20260416

# 月升月落和月相
qw moon --city 北京 --date 20260416

# 太阳高度角
qw solar --lon 116.40 --lat 39.90 --date 20260416 --time 1200 --tz 0800 --alt 43
```

### 命令行参数速查

**全局参数**（所有命令共享）：
- `--kid` / `--project-id` / `--private-key` / `--base-url` / `--geo-url`：覆盖环境变量
- `--lang`：语言（默认 `zh`）

**定位参数**（互斥，三选一）：
- `--city <name>`：城市名称，自动调用 geo API 解析
- `--location-id <id>`：直接使用 Location ID
- `--lon <float>` + `--lat <float>`：经纬度坐标（必须同时提供）

**各命令专属参数**：
- `weather daily [DAYS]`：`3|7|10|15|30`（默认 3）
- `weather hourly [HOURS]`：`24|72|168`（默认 24）
- `grid daily [DAYS]`：`3|7`（默认 3）
- `grid hourly [HOURS]`：`24|72`（默认 24）
- `air hourly [HOURS]`：`24|72`（默认 24）
- `air daily [DAYS]`：`1|3|5`（默认 3）
- `indices --days <1d|3d>`（默认 `1d`）
- `sun --date yyyyMMdd` / `moon --date yyyyMMdd`
- `solar --date yyyyMMdd --time HHmm --tz 偏移 --alt 海拔`

### 参数限制

| 命令 | 参数 | 限制说明 |
|------|------|---------|
| `weather daily` | `[DAYS]` | 仅支持 `3,7,10,15,30` |
| `weather hourly` | `[HOURS]` | 仅支持 `24,72,168` |
| `grid daily` | `[DAYS]` | 仅支持 `3,7` |
| `grid hourly` | `[HOURS]` | 仅支持 `24,72` |
| `air daily` | `[DAYS]` | 仅支持 `1,3,5` |
| `air hourly` | `[HOURS]` | 仅支持 `24,72` |
| `sun` / `moon` | `--date` | 必选，格式 `yyyyMMdd`，**仅未来 60 天内（含今天）** |
| `solar` | `--date` / `--time` / `--tz` / `--alt` | 全部必填 |
