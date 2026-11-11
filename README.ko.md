# PdfCraft Korean

[English](README.md) · [한국어](README.ko.md) · [简体中文](README.zh-CN.md) · [日本語](README.ja.md)

**[@ai-blink](https://github.com/ai-blink)의 한국어 PDF 텍스트 편집 개선 프로젝트입니다.**

[PdfCraft](https://github.com/storytold/pdfcraft)를 기반으로 기존 한글 텍스트를 편집하고, 저장·재열기·검색·선택·복사까지 보존하는 흐름을 개선한 독립 파생 프로젝트입니다. 원본 PDF 엔진과 앱, 선행 기여의 저작자는 각각의 원저작자입니다.

## @ai-blink의 기여

- 원본 [PR #6101](https://github.com/storytold/pdfcraft/pull/6101)의 한국어 편집 변경을 선별 적용하고, 검증 기준의 태국어 fallback·variation selector·행 처리 동작을 보존했습니다.
- 고정된 NanumGothic을 문서와 편집 입력창에 연결하고, 지원되는 한자는 일본어 폰트로 보완했습니다.
- 현대 한글 자모 조합과 UTF-8 스타일 위치 대응을 보강했습니다. 스타일만 변경하는 편집, 잘못된 원문 범위, 과도하게 큰 분해형 입력, 변경 없는 하이픈 제거 텍스트의 회귀를 수정했습니다.
- Windows Hyper-V 게스트에서 편집 → 저장 → 닫기 → 재열기 → 렌더 확인 → 검색 하이라이트 → 선택 → 복사와 실제 클립보드를 검증했습니다.
- 로컬 시험 제어기의 STA 클립보드 처리로 발생한 Ctrl+C ACK timeout을 진단하고 메시지 처리 방식으로 수정했습니다. 앱 소스와 구분되는 시험 제어기 수정입니다.

## 검증 상태

한국어 구현 커밋 [`df68b4d`](https://github.com/ai-blink/pdfcraft-korean/commit/df68b4dc70672482766c2a71dedfe0feea3e29ca)에서 **테스트 3,945개 통과**, **기존 제외 9개**를 확인했습니다. 변경한 크레이트의 clippy, fmt, layers, assets, parity, wasm 검사도 통과했습니다. 새 회귀 검사는 수정 전 실패·수정 후 통과를 확인했습니다.

완성형·분해 자모·한글/한자 혼용 문장으로 저장 파일 재열기, 화면, 검색 위치, 선택 및 게스트의 실제 클립보드를 확인했습니다. 수정한 제어기에서는 **Ctrl+C 성공 응답과 클립보드 정확 일치가 9/9회**였습니다. 한국어 폰트가 없는 빌드는 명확한 오류를 반환하고 원문과 초안을 보존했습니다.

이 결과는 해당 구현 스냅샷의 결과입니다. 이후 공개용 브랜드 변경에 대한 전체 GUI 재검증을 뜻하지 않습니다. 검증한 기준도 최신 upstream main과 다릅니다. [검증 상세·재현 절차](docs/KOREAN-VALIDATION.md)를 참고하세요.

## 빌드 및 사용

Rust **1.95 이상**과 플랫폼별 네이티브 빌드 도구가 필요합니다. 영어 기본 문서의 [고정 폰트·번역 빌드 절차](README.md#build-with-korean-fonts-and-translations)를 사용하세요. 두 입력을 프로젝트가 고정한 버전으로 준비하고 `CRAFT_FONTS_REQUIRED=1`, `CRAFT_I18N_REQUIRED=1`로 빌드해야 합니다.

1. 합성 PDF를 열고 **Edit text**로 기존 문장을 선택합니다.
2. `안녕하세요. 한글 편집 검증 2026, 계약 금액 1,234,567원.`으로 바꿉니다.
3. 저장하고 문서를 닫은 뒤, 저장된 PDF를 새로 엽니다.
4. 화면을 확인하고 `한글 편집`을 검색한 다음 문장을 선택·복사합니다.
5. 실제 클립보드를 합성된 기대 문자열과 비교합니다. 분해형 자모와 한자 혼용 문장도 반복합니다.

현대 한글 자모는 저장·복사 시 조합됩니다. 일반 NFC/NFKC 전체 정규화나 모든 분해형 검색 질의의 등가성을 보장하지 않습니다. 현재 웹용 폰트 구성에는 한국어 폰트가 없으므로 wasm 빌드 통과는 웹 한글 편집 지원을 의미하지 않습니다. 이 변경을 담은 새 설치 패키지나 릴리즈는 아직 제공하지 않습니다.

## 출처와 라이선스

- **한국어 변경 통합·추가 수정·검증:** [@ai-blink](https://github.com/ai-blink).
- **원본:** [ArtCraft 팀과 기여자들의 PdfCraft](https://github.com/storytold/pdfcraft).
- **선행 한국어 작업:** [PR #6101](https://github.com/storytold/pdfcraft/pull/6101)과 그 작성자들. 선행 코드를 직접 새로 작성한 것처럼 표시하지 않습니다.
- **코드 라이선스:** [MIT](LICENSE-MIT) 또는 [Apache-2.0](LICENSE-APACHE). 원본 저작권, [NOTICE](NOTICE), [제3자 출처](ATTRIBUTION.md)를 보존합니다.

공식 upstream 릴리즈가 아니며 원본 팀의 후원·승인을 뜻하지 않습니다. 원본 상표는 코드 라이선스와 별개이며 [브랜드 조건](docs/brand/LICENSE-brand.txt)이 적용됩니다.
