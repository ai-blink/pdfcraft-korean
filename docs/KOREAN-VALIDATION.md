# Korean editing validation

[Project overview](../README.md) · [한국어](../README.ko.md) · [简体中文](../README.zh-CN.md) · [日本語](../README.ja.md)

This is a public summary of the Korean implementation validation performed on 2026-11-10 (Korea Standard Time). It separates the application result from the local Windows test-controller correction. Private machine paths, credentials, VM identifiers and raw session records are not published here. Raw logs, screenshots and clipboard comparisons were retained locally; this summary does not bundle those records or a test executable.

## Tested provenance

| Input | Revision or SHA-256 |
| --- | --- |
| Upstream main used as the base | `9ee213eff725e39f4450f0160cfe6ea5a9120f20` |
| Selectively adapted [PR #6101](https://github.com/storytold/pdfcraft/pull/6101) head | `affdb95af3e4f942fac7032007be57da9e98d434` |
| Local Korean implementation commit | `df68b4dc70672482766c2a71dedfe0feea3e29ca` |
| Committed source tree | `d815a02354a2d3b9117e401a4bc5d264613d8466` |
| Validated precommit diff SHA-256 | `a70e05c3537afae246198db2f67ea6f69a06be6f7d654667687e6fd46f89c4ce` |
| craft-fonts revision | `b076417b5880bd4154bd22f00695b1d28c6bea83` |
| craft-i18n revision | `575cfdef63227b14daf173b8cc87928bfdfa325e` |
| Font-enabled Windows executable SHA-256 | `c177f81ff1f74b580e2dee16074d7aae75512ca0464e76672c9df773e88c0cbd` |
| No-font Windows executable SHA-256 | `89c7c87c99c9f120b1cc1214bc0750e99440a08e0b1b44dd6e6b66c9f7907f9b` |

The executable was built before the local delivery commit was created. Its validated source snapshot matches the committed tree; this does not claim the executable embeds that later commit ID. Later branding and README commits are outside these original GUI results. No rebase onto the subsequently advanced upstream main is claimed.

## Acceptance results for that snapshot

| Criterion | Observed result |
| --- | --- |
| Replace existing text with the required Korean sentence | Passed in Windows guest GUI |
| Decomposed modern jamo and mixed Hangul/Hanja | Passed as separate cases |
| Save, close, then reopen the saved bytes | Passed; reopened saved files were inspected |
| Render without missing/replacement/empty glyphs or overlaps | Passed by inspection of actual guest captures |
| Search `한글 편집`, result position and highlight | Passed after reopening |
| GUI selection and actual guest clipboard | Passed; fresh sentinels prevented stale clipboard success |
| Native Ctrl+C success ACK and clipboard | 9/9 after the controller-only fix; three repeats for each of three cases |
| Missing Korean font | Clear refusal; source bytes, document state and draft preserved |
| Changed-crate tests | 3,945 passed, 0 failed, 9 existing ignored |
| fmt, clippy, layers, assets, parity, wasm | Passed |
| New regression tests | Reproduced failure before fixes, success after fixes |
| General decomposed-query equivalence | Not tested or claimed |
| Hardware IME / all Windows keyboard configurations | Not certified |
| Korean editing in wasm | Not supported by the current Korean-font-free web subset |
| Full GUI verification of later publication branding changes | Outside this snapshot's results |

## Normalization policy

Compose **modern Hangul only**: leading jamo + vowel + optional trailing jamo, and an existing LV syllable + trailing jamo. Compatibility jamo, standalone jamo and unrelated Unicode are unchanged. This is not general NFC or NFKC normalization.

Stored text and copied text are compared with the resulting composed syllables. The tested search query is the precomposed string `한글 편집`. Arbitrary decomposed search queries are not claimed to be equivalent.

The inline editor applies this composition to each committed Text/Paste payload before insertion. IME events and separate payload boundaries retain their existing behavior; composition across arbitrary separate input events is not claimed.

## Synthetic cases and saved-file fingerprints

All three cases started from the same synthetic PDF containing editable existing Latin text. No personal document or commercial font was used.

- **Precomposed input and expected output:** `안녕하세요. 한글 편집 검증 2026, 계약 금액 1,234,567원.`
- **Decomposed input:** the modern-jamo decomposition of the same sentence. Expected saved/copied output is the precomposed sentence above.
- **Mixed input and expected output:** `한글 편집 契約 金額 1,234,567원, 勞動 働.`

| Artifact | SHA-256 |
| --- | --- |
| Synthetic input PDF | `1700314b5a59c6b397f771ab3b7ba69163edc21567815785bb61c2af9d032eca` |
| Saved precomposed case | `cc189c4b673a3f44c123683dee6f78af4cca6cf5c728cc5c2de4c21a4380b7af` |
| Saved decomposed-input case | `e6abb54656054ffd468a197b6466e03ba9ae7d8a881c93e8ec8d32b568763604` |
| Saved mixed Hangul/Hanja case | `fd4048f18e1028bf061962f24ad721e2ec1969be045e58c6cc1bf13b0496cadf` |
| No-font case after refusal | Same SHA-256 as the input PDF |

These identify the recorded local artifacts, not byte-for-byte outputs promised for every rerun. Save metadata or execution context may vary.

## Reproduce the code checks

Use the pinned build inputs in the [build instructions](../README.md#build-with-korean-fonts-and-translations). The original code checks ran under Linux; the native GUI checks ran inside Windows 11 Pro on Hyper-V. General unit/static checks do not inject host desktop input.

```sh
cargo fmt --all -- --check
cargo clippy --locked -p pdfcraft-edit -p pdfcraft-fonts -p pdfcraft-engine -p pdfcraft-ui-egui --all-targets --no-deps -- -D warnings
cargo test --locked -p pdfcraft-edit -p pdfcraft-fonts -p pdfcraft-engine -p pdfcraft-ui-egui
cargo xtask layers
cargo xtask assets
cargo xtask parity
cargo xtask wasm
```

To check missing-font behavior separately, keep the translation input but unset the font input in a separate build/test environment:

```sh
env -u CRAFT_FONTS_DIR -u CRAFT_FONTS_REQUIRED cargo test --locked -p pdfcraft-edit -p pdfcraft-fonts
```

The tests in [korean_tests.rs](../crates/edit/src/korean_tests.rs) save to disk, drop the document, reopen saved bytes, check text/search geometry and verify refusal without mutation when fonts are absent. [Inline-editor regressions](../crates/ui-egui/src/tools/edit_text_ui/hangul_tests.rs) cover committed text, paste, cursor/style offsets and preservation of IME/unfocused events. Engine extraction alone is not a GUI clipboard test.

## Reproduce the GUI acceptance flow

1. Use an authorized Hyper-V Windows guest and a native build with both pinned inputs. Record the executable hash and synthetic input hash.
2. Open a fresh synthetic PDF copy and edit its existing text with one of the three cases.
3. Save, close the document and reopen the saved PDF. Record its hash. Do not substitute a still-open in-memory document for reopening.
4. Capture the rendered result and inspect for lost characters, replacement boxes, empty glyphs and overlapping text.
5. Search the saved sentence for `한글 편집`; verify match count, result location and highlight geometry.
6. Select the sentence through the guest GUI. Set a fresh clipboard sentinel, issue real Ctrl+C, wait for the success response and read the guest's actual clipboard string. Compare it exactly with the expected output under the normalization policy.
7. Repeat for all three inputs. Keep screenshots, responses and clipboard comparisons. Do not replace native clipboard inspection with CLI text extraction.
8. With a no-font build, attempt the same edit and capture the clear error. Verify original text, document state and unsaved draft are preserved; the saved document must not silently contain damaged text.

All GUI input and clipboard operations in the recorded runs were confined to the guest. No host keyboard/mouse input or host focus change was used for the tests.

## Ctrl+C ACK timeout: controller correction

The local Windows controller owned the clipboard on an STA thread and blocked that same thread in a synchronous TCP `ReadLine` while awaiting the GUI response. The native clipboard operation needed owner notifications to be dispatched, so Ctrl+C's response missed the application's existing input deadline. The baseline NFC case had 0/3 successful ACKs even though all three clipboard strings eventually matched.

The controller was changed to `ReadLineAsync` with bounded waiting and same-thread Windows Forms message dispatch for authentication and response reads. With the **same application executable**, the corrected controller produced 9/9 successful native Ctrl+C ACKs and exact fresh-sentinel clipboard comparisons: three precomposed, three decomposed and three mixed cases. The application's five-second input deadline was unchanged, and the client's existing 45-second deadline was not increased.

This correction belongs to the local verification fixture, not the application's Rust implementation. Earlier failures and GUI Copy-menu retries were retained rather than overwritten with success records. The fixture and raw machine-specific evidence are not part of this public source commit.

## Publication follow-up (2026-11-11)

The independent repository removes the eight ArtCraft trademark image files and the old Home/tools screenshot containing the mark. Home and About identify the derivative maintainer; support links and update discovery point to `ai-blink/pdfcraft-korean`. Original copyright notices and the open-source app icon remain credited.

The publication changes passed changed-crate engine/UI/CLI tests, xtask tests, doctests, fmt, clippy with warnings denied, layers, assets, parity and the wasm check. Three update-dialog assertions initially expected the old product name; their expectations and mock release URL were corrected, and all eight update integration tests then passed. Passing suites were not repeated without a source-impact reason. Packaging PowerShell syntax and static installer identity checks also passed.

This follow-up does not claim a new Windows GUI run or a rebuilt/redistributed Windows binary. No installer or release is included. Compiled MSI and installation checks remain unrun; Linux/macOS package and settings identities need isolation before an installer release, as described in [packaging notes](../packaging/windows/INSTALLATION.md#independent-korean-fork).
