# JumpServer 客户端拉起 Navicat


# 注意，注意，注意，请避免使用第三方编译后的二进制文件，最好自己编译一份，防止可能存在的数据窃取。


程序会导入注册表数据，并在程序结束前删除。

请注意，如果你本地已存在的连接和 jumpserver 中的连接同名，则会覆盖并清理，请注意不要同名。

请先备份好本地数据，最好先测试再使用。


## 使用步骤

1. 进入 `%AppData%\JumpServer\Client\bin\` 目录， 原有的 JumpServerClient.exe 重命名为 JumpServerClient2.exe
2. 编译本项目，并将生成的 exe 文件放置到 `%AppData%\JumpServer\Client\bin\JumpServerClient.exe`
3. 在 `%AppData%\JumpServer\Client\bin\` 创建navicat.path 文件，写 navicat 程序的路径，到exe的，比如 `C:\Program Files\PremiumSoft\Navicat Premium 16\navicat.exe`


一次暂时只能启动一个 Navicat，后续慢慢优化，可以先在 工具>选项>高级>允许重复运行 navicat。

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