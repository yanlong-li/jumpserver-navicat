# JumpServer 客户端拉起 Navicat
>当前仅支持 MySQL 数据库，如需要其它数据库可提交 issue 交流。

## 注意，注意，注意，请避免使用第三方编译后的二进制文件，最好自己编译一份，防止可能存在的数据窃取。


因 `Navicat` 本身机制，程序会将数据库连接信息写入注册表，为保护敏感信息，程序将在结束前删除写入的数据。

请注意，如果你本地已存在的连接和 `jumpserver` 中的连接同名，则会覆盖并清理，请注意不要同名。

请先备份好本地数据，最好先测试再使用。


## 使用步骤

本程序当前已升级到 v0.3.0，可将本程序放置于任意目录（避免中文和特殊符号等），然后双击运行一次。

* 首次运行本程序会覆盖堡垒机 jms:// 协议的执行程序路径，一旦您打开堡垒机原始程序的GUI程序则可能会失效，需要重新再双击运行一次本程序。

程序将自动检测 `Navicat` 的安装目录,如您安装目录非默认目录，请在本程序同目录下创建 `navicat.path` 文件，并将Navicat实际目录写入文件。
> 比如 `C:\Program Files\PremiumSoft\Navicat Premium 16\navicat.exe`

当前检测列表为：

    C:\Program Files\PremiumSoft\Navicat Premium 17
    C:\Program Files\PremiumSoft\Navicat Premium Lite 17
    C:\Program Files\PremiumSoft\Navicat 17 for MySQL
    C:\Program Files\PremiumSoft\Navicat Premium 16
    C:\Program Files\PremiumSoft\Navicat 16 for MySQL

当前程序仅支持 `MySQL` 协议，如遇到其他协议如 `SSH`、`Redis` 等会调用 `jms` 原始客户端。将在以下目录检测 `jms` 原始客户端。

    %UserProfile%\AppData\Roaming\JumpServer\client\bin\JumpServerClient.exe
    %UserProfile%\AppData\Local\Programs\JumpServerClient\resources\bin\windows\JumpServerClient.exe
    %UserProfile%\AppData\Local\Programs\JumpServerClient\resources\windows\JumpServerClient.exe
    %ProgramFiles%\JumpServerClient\resources\bin\windows\JumpServerClient.exe
    %ProgramFiles%\JumpServerClient\resources\windows\JumpServerClient.exe


## 关于Navicat的连接配置，可以修改注册表进行自定义

复制源码中的 `mysql.reg` 到 本程序目录下，进行自定义的一些编辑。程序将以此文件作为模板替换一些关键词。

例如：
```text
// 开启心跳
"UsePingInterval"=dword:00000001
// 关闭心跳
"UsePingInterval"=dword:00000000
```

* 其它自行定义
