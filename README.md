![应用图标](config/trojan_ui.svg)

# 介绍
这是一个使用rust egui编写的一个trojan GUI工具，主要目的是方便从桌面直接启动trojan，避免了命令行的烦恼。

# 配置
配置文件需要是json格式的，并且文件名称必须为config.json。
具体路径有如下选择：
- $XDG_CONFIG_HOME/trojan_ui/config.json
- $HOME/trojan_ui/config.json

下面给出一个可用的配置例子：
```
{
  "version":1,
  "dark_mode":false,
  "configs":[
    {
      "remarks":"example",
      "server":"example.com",
      "server_port":443,
      "client":"127.0.0.1",
      "client_port":1080,
      "sni":"",
      "password":"fawiefaslclaisf",
      "verify":true
    },
    {
      "remarks": "test",
      "server": "test.cn",
      "server_port": 443,
      "client": "127.0.0.1",
      "client_port": 1080,
      "sni": "test.cn",
      "password": "fasdfjlsdfwwer",
      "verify": true
    }
  ]
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

# 协议参考
[trojan protocol](https://trojan-gfw.github.io/trojan/protocol)

[socks5 protocol](https://www.rfc-editor.org/rfc/rfc1928)

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