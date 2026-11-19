# JSON Type Studio — Desktop Downloads

JSON Type Studio 是 Windows 与 macOS 的本地 JSON 编辑器、阅读器和类型可视化工具。

这个公开仓库用于发布安装包、公钥和签名更新清单。源码单独维护。

## 策划版 Config

当前版本：[**0.3.1 下载与更新说明**](https://github.com/dongrv/json-type-studio-releases/releases/tag/config-v0.3.1)。历史版本见 [Config 下载列表](https://github.com/dongrv/json-type-studio-releases/releases)，请选择 `config-v` 开头的编号版本。

- Windows x64：下载 `windows-x86_64-setup.exe` 安装包，双击安装。
- macOS Apple Silicon：选择 `darwin-aarch64.dmg`。
- macOS Intel：选择 `darwin-x86_64.dmg`。

0.3.1 提供 JSON / JSONL 编辑、源码与结构/表格联动、Go/Rust/TypeScript/PHP/C# 类型推导与手动标注、32 MiB 文件支持，以及策划配置的目录检查和权重对照。原始数值保持无损。

0.3.1 支持通过系统打开方式直接打开文件、设置页和默认 12px 字号。

主要功能：

- `.jts` 完整文档，将原始 JSON / JSONL 与类型标注放在一个文件中传递；仍可选择原生格式，原生格式的标注单独保存在数据文件旁。
- 五种语言的数据结构预览、复制和文件导出；点击树或表格中的类型标签，可直接选择 `int32` 等语言类型。
- 12–24px 阅读字号、舒适／大字预设、Ctrl＋滚轮缩放，离线内置开源字体。
- 更明显的源码联动高亮、太阳／月亮一键切换主题，以及适应大字号和窄窗口的工具分组。

Windows 从 0.2.0 起支持应用内检查和安装签名更新。0.3.1 为可选更新；更新前保留未保存的 JSON、JSONL 和类型标注草稿，必要更新时仍可保存、复制、导出当前工作。

macOS 包仍为未公证的预览构建。Mac 自动更新通道将在完成 Developer ID 签名、公证与安装验证后启用。

`config-updates/planner.json` 是客户端读取的签名更新清单，下载安装包请使用编号版本。通用版和策划版保持独立发布。

更新签名公钥见 [updater.pub](updater.pub)。更新签名与操作系统发布者认证是不同机制；目前 Windows 包未配置 Authenticode。

维护者可通过 [最低支持版本配置](update-policy.json) 控制需要更新的客户端；提交后由 Actions 校验并重新签名更新清单，客户端在下一次检查时生效。最低版本不能高于当前频道版本。
