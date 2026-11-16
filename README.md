# JSON Type Studio — Desktop Downloads

JSON Type Studio 是 Windows 与 macOS 的本地 JSON 编辑器、阅读器和类型可视化工具。

这个公开仓库用于发布安装包、公钥和签名更新清单。源码单独维护。

## 策划版 Config

前往 [Config 下载列表](https://github.com/dongrv/json-type-studio-releases/releases) 选择 `config-v` 开头的版本。

- Windows x64：下载 `windows-x86_64-setup.exe` 安装包，双击安装。
- macOS Apple Silicon：选择 `darwin-aarch64.dmg`。
- macOS Intel：选择 `darwin-x86_64.dmg`。

0.2.0 提供 JSON / JSONL 编辑、源码与结构/表格联动、Go/Rust/TypeScript/PHP/C# 类型推导及手动标注、32 MiB 文件支持，以及策划配置的目录检查和权重对照。类型标注单独保存在 JSON 文件旁，原始数值保持无损。

Windows 从 0.2.0 起支持应用内检查和安装签名更新。更新前自动保留未保存的 JSON、JSONL 和类型标注草稿；必要更新时仍可保存、复制、导出当前工作。

macOS 0.2.0 为未公证的预览构建。Mac 自动更新通道将在完成 Developer ID 签名、公证与安装验证后启用。

`config-updates/planner.json` 是客户端读取的签名更新清单，下载安装包请使用编号版本。通用版和策划版保持独立发布。

更新签名公钥见 [updater.pub](updater.pub)。更新签名与操作系统发布者认证是不同机制；目前 Windows 包未配置 Authenticode。
