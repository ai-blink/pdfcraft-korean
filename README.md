# PdfCraft Korean

[English](README.md) · [한국어](README.ko.md) · [简体中文](README.zh-CN.md) · [日本語](README.ja.md)

## Windows download

[Windows x64 preview — 0.18.2-korean.1](https://github.com/ai-blink/pdfcraft-korean/releases/tag/v0.18.2-korean.1). Extract the portable ZIP and run `pdfcraft-korean.exe`, or use the MSI installer. These packages are unsigned; updates are manual. Other platforms are source-only.

**Korean PDF text-editing improvements by [@ai-blink](https://github.com/ai-blink).**

An independent derivative of [PdfCraft](https://github.com/storytold/pdfcraft), focused on editing existing Korean text and preserving it through save, reopen, search, selection and copy. The original PDF engine, application and earlier contributions remain the work of their respective authors.

## What @ai-blink added

- Selectively integrated the Korean editing changes from upstream [PR #6101](https://github.com/storytold/pdfcraft/pull/6101), preserving the tested base's Thai fallback, variation-selector handling and row behavior.
- Added Korean document and inline-editor font fallback using the pinned NanumGothic input, with Japanese font fallback for supported Hanja.
- Fixed modern Hangul composition and UTF-8 style-offset mapping, including style-only edits, invalid source ranges, oversized decomposed input and unchanged dehyphenated text.
- Verified edit → save → close → reopen → render → search highlight → select → copy against the actual clipboard inside a Windows Hyper-V guest.
- Diagnosed a Ctrl+C ACK timeout in the local test controller's STA clipboard handling. A message-pumping controller resolved it; this was a test-fixture fix, separate from the application source.

## Validation status

The Korean implementation at [`df68b4d`](https://github.com/ai-blink/pdfcraft-korean/commit/df68b4dc70672482766c2a71dedfe0feea3e29ca) passed **3,945 tests**, with **9 existing tests ignored**. Changed-crate clippy, fmt, layers, assets, parity and wasm checks passed. New regressions failed before their fixes and passed afterward.

Windows guest checks covered precomposed Hangul, decomposed modern jamo and mixed Hangul/Hanja, saved-file reopening, render inspection, search highlights, selection and native clipboard comparison. The corrected controller produced **9/9 successful Ctrl+C ACKs and exact clipboard matches**. A missing-font build refused the edit clearly and preserved the source document and draft.

These results describe that implementation snapshot. Later publishing/branding changes do not inherit a new full GUI certification. The tested base is not the latest upstream main. See [validation details and reproduction](docs/KOREAN-VALIDATION.md) for scope, normalization and commands.

## Build with Korean fonts and translations

Requires Rust **1.95 or newer** and the native build tools for your platform. The example uses PowerShell and sibling directories; run it from a directory where the three clone destinations do not already exist.

```powershell
git clone https://github.com/ai-blink/pdfcraft-korean.git
git clone https://github.com/storytold/craft-fonts.git
git clone https://github.com/storytold/craft-i18n.git
git -C craft-fonts checkout --detach b076417b5880bd4154bd22f00695b1d28c6bea83
git -C craft-i18n checkout --detach 575cfdef63227b14daf173b8cc87928bfdfa325e
Set-Location pdfcraft-korean
$env:CRAFT_FONTS_DIR = (Resolve-Path ../craft-fonts).Path
$env:CRAFT_I18N_DIR = (Resolve-Path ../craft-i18n).Path
$env:CRAFT_FONTS_REQUIRED = '1'
$env:CRAFT_I18N_REQUIRED = '1'
cargo build --locked --release -p pdfcraft -p pdfcraft-cli
cargo run --locked --release -p pdfcraft -- example.pdf
```

The fonts are a build input, not files copied from installed commercial software. Keep both checkouts at the pinned revisions. The font pin comes from the inherited release configuration; the translation pin is recorded in [craft-i18n.rev](craft-i18n.rev). See [font behavior](crates/fonts/README.md) for details.

**Desktop scope:** the current wasm font subset does not include a Korean face. Passing the wasm build check does not establish Korean editing support in the web app. Upstream downloads are not builds of these changes.

## Try the editing flow

1. Open a synthetic PDF with editable existing text and choose **Edit text**.
2. Replace a sentence with `안녕하세요. 한글 편집 검증 2026, 계약 금액 1,234,567원.`
3. Save, close the document and open the saved PDF again.
4. Inspect the rendered text, search for `한글 편집`, then select and copy the sentence.
5. Compare the actual clipboard string with the expected composed sentence. Repeat with decomposed modern jamo and mixed Hangul/Hanja.

Modern Hangul jamo are composed for storage and copy. This is limited Hangul composition, not general Unicode NFC/NFKC normalization or a claim that every decomposed search query is equivalent. See the [normalization policy](docs/KOREAN-VALIDATION.md#normalization-policy).

## Attribution and license

- **Korean integration, additional fixes and validation:** [@ai-blink](https://github.com/ai-blink).
- **Original project:** [PdfCraft by the ArtCraft team and contributors](https://github.com/storytold/pdfcraft).
- **Earlier Korean editing work:** upstream [PR #6101](https://github.com/storytold/pdfcraft/pull/6101) and its authors; selectively adapted rather than claimed as newly authored work.
- **Code license:** [MIT](LICENSE-MIT) OR [Apache-2.0](LICENSE-APACHE). Original copyright notices, [NOTICE](NOTICE) and [third-party attribution](ATTRIBUTION.md) are preserved.

This independent repository is not an official upstream release and does not imply upstream endorsement. Upstream trademarks are separate from the code license; see [the original brand terms](docs/brand/LICENSE-brand.txt).
