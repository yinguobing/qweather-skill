# 天气相关技能

基于和风天气服务实现天气查询智能体技能。

关于“技能”：https://agentskills.io/what-are-skills

## 技能点

### GeoAPI

和风天气GeoAPI提供全球地理位位置、全球城市搜索服务，支持经纬度坐标反查、多语言、模糊搜索等功能。

天气数据是基于地理位置的数据，因此获取天气之前需要先知道具体的位置信息。和风天气提供一个功能强大的位置信息搜索API服务：GeoAPI。通过GeoAPI，你可获取到需要查询城市或POI的基本信息，包括查询地区的Location ID（你需要这个ID去查询天气），多语言名称、经纬度、时区、海拔、Rank值、归属上级行政区域、所在行政区域等。

除此之外，GeoAPI还可以帮助你：

避免重名城市的困扰
支持名称模糊搜索
在你的应用或网站中根据用户输入的名称返回多个城市结果，便于用户选择准确的城市并返回该城市天气
在你的应用或网站中展示热门城市
不需要维护城市列表，城市信息更新实时获取

#### 城市搜索
城市搜索API提供全球地理位位置、全球城市搜索服务，支持经纬度坐标反查、多语言、模糊搜索等功能。
API文档：https://dev.qweather.com/docs/api/geoapi/city-lookup/

#### 热门城市查询
获取全球各国热门城市列表。
API文档：https://dev.qweather.com/docs/api/geoapi/top-city/

#### POI搜索
使用关键字和坐标查询POI信息（景点、火车站、飞机场、港口等）
API文档：https://dev.qweather.com/docs/api/geoapi/poi-lookup/

#### POI范围搜索
提供指定区域范围内查询所有POI信息。
API文档：https://dev.qweather.com/docs/api/geoapi/poi-range/


### 天气预报
天气API提供全球20多万个城市的实时天气和预报数据，并支持基于数值模式的天气预报，分辨率达3–5公里，覆盖全球坐标点。

#### 实时天气
获取中国3000+市县区和海外20万个城市实时天气数据，包括实时温度、体感温度、风力风向、相对湿度、大气压强、降水量、能见度、露点温度、云量等。
https://dev.qweather.com/docs/api/weather/weather-now/

#### 每日天气预报
每日天气预报API，提供全球城市未来3-30天天气预报，包括：日出日落、月升月落、最高最低温度、天气白天和夜间状况、风力、风速、风向、相对湿度、大气压强、降水量、露点温度、紫外线强度、能见度等。
https://dev.qweather.com/docs/api/weather/weather-daily-forecast/

#### 逐小时天气预报
逐小时天气预报API，提供全球城市未来24-168小时逐小时天气预报，包括：温度、天气状况、风力、风速、风向、相对湿度、大气压强、降水概率、露点温度、云量。
https://dev.qweather.com/docs/api/weather/weather-hourly-forecast/

#### 格点实时天气
基于数值模式的天气预报数据，提供全球指定坐标的实时天气，分辨率3-5公里。
https://dev.qweather.com/docs/api/weather/grid-weather-now/

#### 格点每日天气预报
基于数值模式的天气预报数据，提供全球指定坐标的每日天气预报，分辨率3-5公里。
https://dev.qweather.com/docs/api/weather/grid-weather-daily-forecast/

#### 格点逐小时天气预报
基于数值模式的天气预报数据，提供全球指定坐标的逐小时天气预报，分辨率3-5公里。
https://dev.qweather.com/docs/api/weather/grid-weather-hourly-forecast/


### 分钟预报
分钟级降水API（临近预报）支持中国1公里精度的分钟级降雨预报数据，为每一分钟的降雨进行精准预测。

#### 分钟级降水
分钟级降水API（临近预报）支持中国1公里精度的未来2小时每5分钟降雨预报数据。
https://dev.qweather.com/docs/api/minutely/minutely-precipitation/

### 天气指数
天气生活指数包括洗车指数、穿衣指数、感冒指数、过敏指数、紫外线指数、钓鱼指数等数据。天气指数支持中国3000+个市县区和海外15万个城市天气预报。

#### 天气指数预报
获取中国和全球城市天气生活指数预报数据。


### 预警
和风极端天气预警API提供了全球官方发布的极端天气预警服务，覆盖中国及全球国家或地区。

和风极端天气预警API覆盖中国及全球国家或地区，目前已包括数百种灾害预警事件，例如：台风、暴雨、暴雪、寒潮、大风、沙尘暴、高温、干旱、雷电、冰雹、霜冻、大雾、霾、道路结冰、寒冷、灰霾、雷雨大风、森林火险、降温、道路冰雪、干热风、低温、冰冻、空气重污染、海上大雾、雷暴大风、持续低温、浓浮尘、龙卷风、低温冻害、海上大风、低温雨雪冰冻、强对流、臭氧、大雪、强降雨、强降温、雪灾、森林（草原）火险、雷暴、严寒、沙尘等等。

#### 实时天气预警
根据指定的经纬度坐标，查询中国和全球正在生效的官方天气预警信息。阅读实用资料-预警以了解预警信息支持的国家和地区、事件类型等必要信息。
https://dev.qweather.com/docs/api/warning/weather-alert/


### 空气质量
全球空气质量API，适配当地空气质量标准，可以轻松的获取指定位置的空气质量、污染物和健康建议。目前已经覆盖100多个国家或地区数据，包括实时和预报数据，分辨率为1x1公里。

#### 实时空气质量
实时空气质量API提供指定地点的实时空气质量数据，精度为1x1公里。
https://dev.qweather.com/docs/api/air-quality/air-current/

#### 空气质量小时预报
空气质量小时预报API提供未来24小时空气质量的数据，包括AQI、污染物浓度、分指数以及健康建议。
https://dev.qweather.com/docs/api/air-quality/air-hourly-forecast/

#### 空气质量每日预报
空气质量每日预报API提供未来3天的空气质量（AQI）预报、污染物浓度值和健康建议。
https://dev.qweather.com/docs/api/air-quality/air-daily-forecast/

#### 监测站数据
监测站数据API提供各个国家或地区监测站的污染物浓度值。
https://dev.qweather.com/docs/api/air-quality/air-station/


### 天文
天文API提供了全球任意地点未来60天的日出日落、太阳高度角、月升月落和月相数据，

#### 日出日落
获取未来60天全球任意地点日出日落时间。
https://dev.qweather.com/docs/api/astronomy/sunrise-sunset/

#### 月升月落和月相
获取未来60天全球城市月升月落和逐小时的月相数据。
https://dev.qweather.com/docs/api/astronomy/moon-and-moon-phase/

#### 太阳高度角
任意时间点的全球太阳高度及方位角。
https://dev.qweather.com/docs/api/astronomy/solar-elevation-angle/


## 最佳实践
https://dev.qweather.com/docs/best-practices/
本文档介绍了一些使用我们服务的常见做法和经验，以便你能够快速的、稳定的获取你需要的数据。
- 不要假设
- 优化请求
- 处理Gzip
- 缓存你的数据
- 安全指南


## 实用资料
浏览全部和风天气开发平台所必备的实用资料，这些是开发文档的必要补充信息，也是开发过程中的扩展工具，例如错误代码的说明、专业名词解释、预警类型、天气图标、城市列表等等。
https://dev.qweather.com/docs/resource/
- 专用词汇表
- 错误码
- 多语言
- 图标说明
- 常见城市列表
- 单位
- 风向风速和等级
- 预警信息
- 天气指数信息
- 空气质量信息
- 太阳和月亮信息



