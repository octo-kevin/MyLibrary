# auto build 规则

1. 代码推送到 github 之后，自动运行前后端的单元测试
2. 对于新创建的 release-* tag，先运行单元测试，再进行自动打包，推送到 docker hub
3. release 发行中提供一个压缩包，其中是启动文档和启动脚本，项目根目录下的 release 存储这些文档
4. 启动方式提供 docker compose，启动脚本用于拉起 compose，提供一个配置文件用于配置所需的配置项，数据库可选使用compose或者用户自己部署的数据库
5. docker 提供多个硬件平台的版本

