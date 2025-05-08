# JumpServer 客户端拉起 Navicat


# 注意，注意，注意，请避免使用第三方编译后的二进制文件，最好自己编译一份，防止可能存在的数据窃取。

## 2025年5月8日 此程序仅针对客户端 v2.0.0 进行测试，其它版本可能存在不兼容。


程序会导入注册表数据，并在程序结束前删除。

请注意，如果你本地已存在的连接和 jumpserver 中的连接同名，则会覆盖并清理，请注意不要同名。

请先备份好本地数据，最好先测试再使用。


## 使用步骤 客户端 v2.0.0

1. 进入 `%AppData%\JumpServer\Client\bin\` 目录， 原有的 JumpServerClient.exe 重命名为 JumpServerClient2.exe
2. 编译本项目，并将生成的 exe 文件放置到 `%AppData%\JumpServer\Client\bin\JumpServerClient.exe`
3. 在 `%AppData%\JumpServer\Client\bin\` 创建navicat.path 文件，写 navicat 程序的路径，到 exe 的，比如 `C:\Program Files\PremiumSoft\Navicat Premium 16\navicat.exe`
4. 在 服务端 点击连接，选择 “客户端” > “数据库客户端” > “连接”

由于 Navicat 本身机制，只在启动时读取注册表中的连接配置信息，故而如果 Navicat 已在运行则新连接不会显示，可以在 工具>选项>高级>允许重复运行 navicat，这样每次启动的 Navicat 客户端都会独立运行且读取新的连接配置。

### 使用步骤 客户端 v2.1.0+

大致一样，只是目录换了

1. 进入 `%USERPROFILE%\appdata\local\Programs\JumpServerClient\resources\bin\windows\` 目录， 原有的 JumpServerClient.exe 重命名为 JumpServerClient2.exe
2. 编译本项目，并将生成的 exe 文件放置到 `%USERPROFILE%\appdata\local\Programs\JumpServerClient\resources\bin\windows\JumpServerClient.exe`
3. 在 `%USERPROFILE%\appdata\local\Programs\JumpServerClient\resources\bin\windows\` 创建 navicat.path 文件，写 navicat 程序的路径，到 exe 的，比如 `C:\Program Files\PremiumSoft\Navicat Premium 16\navicat.exe`
4. 在 服务端 点击连接，选择 “客户端” > “数据库客户端” > “连接”

由于 Navicat 本身机制，只在启动时读取注册表中的连接配置信息，故而如果 Navicat 已在运行则新连接不会显示，可以在 工具>选项>高级>允许重复运行 navicat，这样每次启动的 Navicat 客户端都会独立运行且读取新的连接配置。

## 使用步骤 客户端 v3.0.0+

我安装了服务端 v4.9.0 对应客户端 v3.0.3，看客户端界面支持 Navicat Premium 17 Lite,但是我点击 只提示 “连接成功”  就没反应了，配置 DBeaver 也是，不知道是不是我电脑问题。
服务端的连接数据库仅显示支持 web cli 、web gui，没显示唤醒客户端按钮。
客户端和服务端连接 xshell 等倒是正常，不知道哪里有问题。

没法继续测试下去了

## 关于Navicat的连接配置，可以修改注册表进行自定义

复制源码中的 `reg.reg` 到 `%AppData%\JumpServer\Client\bin\` 目录下，进行自定义的一些编辑。程序将以此文件作为模板替换一些关键词。

例如：
```text
// 开启心跳
"UsePingInterval"=dword:00000001
// 关闭心跳
"UsePingInterval"=dword:00000000
```

* 其它自行定义
