# netflix-verify

> 注意：项目的主要用途是用于练习 rust 的使用，目前已不再维护。

最新版本: `v1.0.0`

流媒体 Netflix 解锁检测脚本，使用 rust 语言编写。

本项目的实现逻辑参考了[项目](https://github.com/sjlleo/netflix-verify)

在网络正常的情况下，双栈网络也可在几秒内快速完成 IPv4/IPv6 的 netflix 解锁情况判断。

## 功能实现

- [X] 代理情况判断（netflix 判定的代理）
- [X] 地域信息显示
- [X] 双栈网络测试
- [X] 支持本地代理

## 使用说明

### 本地编译运行

- 环境安装 rust 和 cargo
- 终端运行命令 `cargo run -- [OPTIONS]`
- 默认情况可以直接运行 `cargo run` 省略参数

### 二进制文件运行

> 命令中的 netflix-verify 为二进制文件名，可能需要指定目录例如 ./netflix-verify 才能运行

- 可以根据系统和架构，执行 `cargo build` 自行打包二进制文件
- 终端运行命令 `netflix-verify [OPTIONS]`

## 相关名词解释

1. **不提供服务** - 所在的地区 netflix 未提供服务，网页可能无法正常访问
2. **代理情况** - 此处特指 netflix 对于您网络是否使用代理的判断结果
3. **解锁自制剧** - 可以看由 netflix 自己发行的影片（不同区域自制剧范围可能不同）
4. **解锁全部** - 可以看 netflix 在此区域的所有影片（包括第三方版权影片）

一般来说，需要能解锁全部才算真正意义上的 netflix 解锁

## 鸣谢

- 感谢 [@sjlleo](https://github.com/sjlleo) 开源 GO 版本项目以供参考
