# PdfCraft Korean

[English](README.md) · [한국어](README.ko.md) · [简体中文](README.zh-CN.md) · [日本語](README.ja.md)

## Windows 下载

[Windows x64 预览版 — 0.18.2-korean.1](https://github.com/ai-blink/pdfcraft-korean/releases/tag/v0.18.2-korean.1)。解压便携 ZIP 后运行 `pdfcraft-korean.exe`，或使用 MSI 安装包。软件包未签名，需手动更新。

**由 [@ai-blink](https://github.com/ai-blink) 改进的韩语 PDF 文本编辑项目。**

这是基于 [PdfCraft](https://github.com/storytold/pdfcraft) 的独立衍生项目，重点改进现有韩语文本的编辑，以及保存、重新打开、搜索、选择和复制流程。原有 PDF 引擎、应用及先前贡献的著作权仍归各自作者所有。

## @ai-blink 的贡献

- 选择性整合上游 [PR #6101](https://github.com/storytold/pdfcraft/pull/6101) 的韩语编辑变更，保留验证基线中的泰语字体回退、变体选择符及行处理行为。
- 使用固定版本的 NanumGothic 支持文档及编辑框中的韩语，并使用日文字体补充支持的汉字。
- 修复现代韩文字母合成与 UTF-8 样式偏移映射，包括仅修改样式、原始范围无效、分解输入过大、未变更的去连字符文本等回归问题。
- 在 Windows Hyper-V 来宾中验证编辑 → 保存 → 关闭 → 重新打开 → 渲染 → 搜索高亮 → 选择 → 复制，并比对实际剪贴板。
- 定位本地测试控制器 STA 剪贴板处理导致的 Ctrl+C ACK 超时，通过消息处理修复。该修复属于测试控制器，与应用源码区分。

## 验证状态

韩语实现提交 [`df68b4d`](https://github.com/ai-blink/pdfcraft-korean/commit/df68b4dc70672482766c2a71dedfe0feea3e29ca) **通过 3,945 项测试**，**保留 9 项既有忽略项**。修改的 crate 通过 clippy、fmt、layers、assets、parity 和 wasm 检查。新增回归测试在修复前失败、修复后通过。

测试覆盖预组合韩文、分解字母及韩文/汉字混合句子的保存文件重开、渲染、搜索位置、选择和来宾实际剪贴板。修复后的控制器实现 **9/9 次 Ctrl+C ACK 成功且剪贴板完全匹配**。不含韩文字体的构建明确拒绝编辑，并保留原文档与草稿。

这些结果对应上述实现快照，并不表示之后的发布品牌变更已完成整套 GUI 复验。测试基线也并非最新 upstream main。请参阅[验证详情与复现步骤](docs/KOREAN-VALIDATION.md)。

## 构建与使用

需要 Rust **1.95 或更新版本**及平台原生构建工具。请使用英文主页的[固定字体与翻译构建步骤](README.md#build-with-korean-fonts-and-translations)，按指定版本准备两个输入，并设置 `CRAFT_FONTS_REQUIRED=1`、`CRAFT_I18N_REQUIRED=1`。

1. 打开合成 PDF，通过 **Edit text** 选择原有句子。
2. 将其替换为 `안녕하세요. 한글 편집 검증 2026, 계약 금액 1,234,567원.`。
3. 保存、关闭文档，然后重新打开保存的 PDF。
4. 检查渲染，搜索 `한글 편집`，再选择并复制句子。
5. 将实际剪贴板与合成后的预期字符串比对，并重复测试分解字母及汉字混合句子。

现代韩文字母会在保存和复制时合成。这不是完整的 NFC/NFKC 规范化，也不保证任意分解形式搜索词的等价性。当前 Web 字体子集不含韩文字体，因此 wasm 构建通过不代表 Web 韩语编辑可用。其他平台仅提供源码。

## 来源与许可

- **韩语整合、追加修复及验证：**[@ai-blink](https://github.com/ai-blink)。
- **原项目：**[ArtCraft 团队及贡献者的 PdfCraft](https://github.com/storytold/pdfcraft)。
- **先前韩语编辑工作：**[PR #6101](https://github.com/storytold/pdfcraft/pull/6101) 及其作者；选择性整合不代表重新原创这些代码。
- **代码许可：**[MIT](LICENSE-MIT) 或 [Apache-2.0](LICENSE-APACHE)。保留原版权声明、[NOTICE](NOTICE) 和[第三方归属](ATTRIBUTION.md)。

本项目并非上游官方发行版，也不表示原团队的支持或认可。上游商标与代码许可分开，适用[原品牌条款](docs/brand/LICENSE-brand.txt)。
