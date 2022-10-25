# IP-Extract

可用于将 [ip2location](https://lite.ip2location.com/database-download) 数据库的 **除中文 IP 和保留 IP 之外** 的 IP 提取出来，转成 `proxifier` 配置的形式。

除此之外，还提供了 `ip` 转换工具，可用于 
```
ipv4 -> u32 , u32 -> ipv4
ipv6 -> u128 , u128 -> ipv6

ipv4 -> ipv6 , ipv6 -> ipv4

ipv4 -> cidr , cidr -> ipv4
ipv6 -> cidr , cidr -> ipv6
```
详见 [ip_tool](/src/ip_tool.rs)

`docker` 构建

```shell
git clone --dept=1 https://github.com/NPCDW/ip-extract.git
cd ip-extract
docker-compose up
```