# 天气相关技能

基于和风天气服务实现天气查询智能体技能。

关于“技能”：https://agentskills.io/what-are-skills

## 功能与命令

主命令同时也是二进制文件名为 `qw`。通过子命令来提供具体功能，具体如下。

### `geo`
地理位置。提供全球地理位位置、全球城市搜索服务，支持经纬度坐标反查、多语言、模糊搜索等功能。
该功能可以用来获取指定地理位置的经纬度以及Location ID。

文档：[地理位置](./geo.md)

### `weather`
天气查询。提供全球20多万个城市的实时天气和预报数据，并支持基于数值模式的天气预报，分辨率达3–5公里，覆盖全球坐标点。

输入参数包括：

地理位置
- `city` 城市名称。需要使用转换为Location ID或者经纬度。

或者
- `location-id` 地理位置的唯一ID

或者
- `longitude` 经度，保留小数点后2位。
- `latitude` 纬度，保留小数点后2位。

经度与纬度必须同时提供。

查询时间
- `now` 实时
- `days` 天数，未来`3,7,10,15,30`天
- `hours` 小时数，未来`24,72,168`小时

文档：[天气查询](./weather.md)

### `precipitation`
分钟级降水预报。分钟级降水API（临近预报）支持中国1公里精度的分钟级降雨预报数据，为每一分钟的降雨进行精准预测。
分钟级降水API（临近预报）支持中国1公里精度的未来2小时每5分钟降雨预报数据。

API文档：https://dev.qweather.com/docs/api/minutely/minutely-precipitation/

### `indices`
天气指数预报。获取中国和全球城市天气生活指数预报数据。天气生活指数包括洗车指数、穿衣指数、感冒指数、过敏指数、紫外线指数、钓鱼指数等数据。天气指数支持中国3000+个市县区和海外15万个城市天气预报。

API文档：https://dev.qweather.com/docs/api/indices/indices-forecast/

### `warning`
实时天气预警。极端天气预警提供了全球官方发布的极端天气预警服务，覆盖中国及全球国家或地区。

文档：[天气预警](./warning.md)

### `air`
全球空气质量。适配当地空气质量标准，可以轻松的获取指定位置的空气质量、污染物和健康建议。目前已经覆盖100多个国家或地区数据，包括实时和预报数据，分辨率为1x1公里。

文档：[全球空气质量](./air.md)

### `sun`
日出日落。获取未来60天全球任意地点日出日落时间。

API文档：https://dev.qweather.com/docs/api/astronomy/sunrise-sunset/

### `moon`
月升月落和月相。获取未来60天全球城市月升月落和逐小时的月相数据。

API文档：https://dev.qweather.com/docs/api/astronomy/moon-and-moon-phase/

### `solar`
太阳高度角。任意时间点的全球太阳高度及方位角。

API文档：https://dev.qweather.com/docs/api/astronomy/solar-elevation-angle/


## 最佳实践
本文档介绍了一些使用我们服务的常见做法和经验，以便你能够快速的、稳定的获取你需要的数据。
- 不要假设
- 优化请求
- 处理Gzip
- 缓存你的数据
- 安全指南
文档：https://dev.qweather.com/docs/best-practices/


## 实用资料
浏览全部和风天气开发平台所必备的实用资料，这些是开发文档的必要补充信息，也是开发过程中的扩展工具，例如错误代码的说明、专业名词解释、预警类型、天气图标、城市列表等等。
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
文档：https://dev.qweather.com/docs/resource/
