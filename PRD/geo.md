# 地理位置

天气数据是基于地理位置的数据，因此获取天气之前需要先知道具体的位置信息。和风天气提供一个功能强大的位置信息搜索API服务：GeoAPI。通过GeoAPI，你可获取到需要查询城市或POI的基本信息，包括查询地区的Location ID（你需要这个ID去查询天气），多语言名称、经纬度、时区、海拔、Rank值、归属上级行政区域、所在行政区域等。

除此之外，GeoAPI还可以帮助你：
- 避免重名城市的困扰
- 支持名称模糊搜索
- 在你的应用或网站中根据用户输入的名称返回多个城市结果，便于用户选择准确的城市并返回该城市天气
- 在你的应用或网站中展示热门城市
- 不需要维护城市列表，城市信息更新实时获取

## 城市搜索
城市搜索API提供全球地理位位置、全球城市搜索服务，支持经纬度坐标反查、多语言、模糊搜索等功能。
API文档：https://dev.qweather.com/docs/api/geoapi/city-lookup/

### 经纬度坐标反查
通过经纬度坐标反查所在城市信息。
命令：`qw geo reverse --lon <LON> --lat <LAT>`

## 热门城市查询
获取全球各国热门城市列表。
API文档：https://dev.qweather.com/docs/api/geoapi/top-city/

## POI搜索
使用关键字和坐标查询POI信息（景点、火车站、飞机场、港口等）
API文档：https://dev.qweather.com/docs/api/geoapi/poi-lookup/

## POI范围搜索
提供指定区域范围内查询所有POI信息。
API文档：https://dev.qweather.com/docs/api/geoapi/poi-range/