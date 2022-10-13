# IP-Extract

将 `https://lite.ip2location.com/database-download` 数据库的 **除中文 IP 和保留 IP 之外** 的 IP 提取出来，转成 `proxifier` 配置的形式。

除此之外，还提供了 `ip` 转换工具，可用于 `ip4 > u32` ， `u32 > ip4` ， `ip6 > u128` ， `u128 > ip6` ，详见 [ip_tool](/src/ip_tool.rs)

`docker` 构建

```shell
git clone --dept=1 https://github.com/NPCDW/ip-extract.git
cd ip-extract
docker-compose up
```