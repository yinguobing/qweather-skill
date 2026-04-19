# qweather

专门为龙虾（OpenClaw）开发的和风天气查询技能。

## 功能特性

基于 [和风天气 API](https://dev.qweather.com/)，提供以下能力：

- **城市定位**：城市搜索、热门城市、POI 搜索
- **天气预报**：实时天气、每日预报、逐小时预报
- **格点天气**：基于经纬度的高分辨率（3-5 公里）天气数据
- **空气质量**：实时空气质量、小时/每日预报、监测站数据
- **生活指数**：穿衣、洗车、运动、紫外线等指数预报
- **分钟级降水**：未来 2 小时每 5 分钟降水预报（中国地区）
- **天气预警**：台风、暴雨、高温等实时预警信息
- **天文数据**：日出日落、月升月落、月相、太阳高度角

## 安装

### 从源码构建

```bash
git clone https://github.com/yinguobing/qweather-skill.git
cd qweather-skill
cargo build --release
```

构建产物位于 `target/release/qweather`。

## 身份认证

使用 JWT 身份认证，**必须**配置以下环境变量：

```bash
export QWEATHER_KID="your_kid"
export QWEATHER_PROJECT_ID="your_project_id"
export QWEATHER_PRIVATE_KEY="-----BEGIN PRIVATE KEY-----\n...\n-----END PRIVATE KEY-----"
export QWEATHER_BASE_URL="https://your-host.qweatherapi.com/v7"
export QWEATHER_GEO_URL="https://your-host.qweatherapi.com/geo/v2"
```

详情可在[和风天气控制台](https://console.qweather.com/project)获取。

## 快速开始

### CLI 命令行工具

```bash
# 通过参数传入 JWT 凭据（必须同时指定自定义 API Host）
qweather 北京 \
  --kid YOUR_KID \
  --project-id YOUR_PROJECT_ID \
  --private-key ./ed25519-private.pem \
  --base-url https://your-host.qweatherapi.com/v7 \
  --geo-url https://your-host.qweatherapi.com/geo/v2

# 私钥也可以直接传 PEM 内容
qweather 北京 \
  --kid YOUR_KID \
  --project-id YOUR_PROJECT_ID \
  --private-key "-----BEGIN PRIVATE KEY-----\nMC4CAQAwBQYDK2VwBCIE..." \
  --base-url https://your-host.qweatherapi.com/v7 \
  --geo-url https://your-host.qweatherapi.com/geo/v2
```

#### 更多查询类型

```bash
# 逐小时预报
qweather 北京 --type hourly --hours 24

# 空气质量
qweather 北京 --type air

# 生活指数
qweather 北京 --type indices

# 分钟级降水
qweather 北京 --type minutely

# 天气预警
qweather 北京 --type warning

# 日出日落
qweather 北京 --type sun --date 20260201

# 格点实时天气（基于经纬度）
qweather 北京 --type grid-now

# 空气质量每日预报
qweather 北京 --type air-daily --days 5

# 月相
qweather 北京 --type moon --date 20260201

# 太阳高度角
qweather 北京 --type solar --date 20260201 --time 1200 --tz 0800 --alt 43
```

## API 模块一览

| 模块 | 说明 |
|------|------|
| `geo` | 城市搜索、热门城市、POI 搜索 |
| `weather` | 实时天气、每日/逐小时预报 |
| `grid_weather` | 格点实时/每日/逐小时天气 |
| `air` | 实时空气质量、预报、监测站（支持 v7 / v1） |
| `indices` | 天气生活指数预报 |
| `minutely` | 分钟级降水预报 |
| `warning` | 实时天气预警 |
| `astronomy` | 日出日落、月相、太阳高度角 |

## 运行测试

```bash
cargo test
```

## 项目结构

```
qweather-skill/
├── Cargo.toml         # Rust 项目配置
├── src/
│   ├── main.rs        # CLI 入口
│   ├── lib.rs         # 库入口
│   ├── client.rs      # HTTP 客户端
│   ├── config.rs      # 配置管理
│   ├── error.rs       # 错误类型
│   ├── models.rs      # 数据模型
│   ├── api/           # API 封装
│   └── cli/           # CLI 参数与格式化
└── tests/             # 集成测试
```

## 许可证

[MIT](LICENSE)
