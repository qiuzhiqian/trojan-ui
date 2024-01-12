![应用图标](config/trui.svg)

# 介绍
这是一个使用rust slint编写的一个trojan GUI工具，主要目的是方便从桌面直接启动trojan，避免了命令行的烦恼。

# 配置
配置文件需要是json格式的，路径有如下选择：
- $HOME/trui/
本软件会扫猫该目录下面所有的client_*.json文件

下面给出一个可用的配置例子：
```
{
  "remarks":"example",
  "server":"example.com",
  "server_port":443,
  "client":"127.0.0.1",
  "client_port":1080,
  "sni":"",
  "password":"fawiefaslclaisf",
  "verify":true
}
```

# 编译
```
cargo build
```

如果要编译release版本，请使用
```
cargo build -r
```

如果需要编译deb包，请确保正确安装了cargo-deb，并运行：
```
cargo deb
```
# 运行
```
./trojan-ui
```
或者双击应用程序运行

# 效果图
**主界面**

![主界面](media/screenshot-main-light.png) 
![主界面](media/screenshot-main-dark.png)

**配置添加界面**

![配置添加界面](media/screenshot-add-config-light.png)
![配置添加界面](media/screenshot-add-config-dark.png)

**配置分享界面**

![配置分享界面](media/screenshot-share-config-light.png)
![配置分享界面](media/screenshot-share-config-dark.png)

# 特性
1. trojan协议代理
2. socks5支持
3. 多配置支持和选择
4. 配置添加，支持通过url方式添加配置以及配置删除
5. 支持通过二维码方式分享配置
6. 明亮和暗色主题切换

# TODO
1. 界面美化
2. 配置编辑功能
3. 添加系统托盘支持
4. 添加连接测试功能
5. 添加流量统计功能
6. 全局代理
7. 开机自启
8. 日志文件记录