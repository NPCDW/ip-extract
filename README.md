# IP-Extract

可用于将 [ip2location](https://lite.ip2location.com/database-download) 数据库的 **除中文 IP 和保留 IP 之外** 的 IP 提取出来，转成 `proxifier` 和 `clash` 配置的形式。

除此之外，还提供了 `ip` 转换工具，可用于 
```
ipv4 -> u32 , u32 -> ipv4
ipv6 -> u128 , u128 -> ipv6

ipv4 -> ipv6 , ipv6 -> ipv4

ipv4 -> cidr , cidr -> ipv4
ipv6 -> cidr , cidr -> ipv6
```
详见 [ip_tool](/src/ip_tool.rs)，也可以去 [https://0520.site/ip/](https://0520.site/ip/) 在线体验

## 使用

建议复制该 [`docker-compose.yml`](docker-compose.yml) 文件到服务器，并在同目录下运行
```bash
docker compose up
```