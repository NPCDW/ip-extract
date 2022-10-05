# IP-Extract

`rust` 练手项目，用于将 `https://lite.ip2location.com/database-download` 数据库的 **除中文 IP 和保留 IP 之外** 的 IP 提取出来，转成 `proxifier` 配置的形式。

`docker` 自己构建

```shell
git clone --dept=1 https://github.com/NPCDW/ip-extract.git
cd ip-extract
docker-compose up
```