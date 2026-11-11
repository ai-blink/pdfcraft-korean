//! Synthetic existing-text regressions. No installed font or external document is used.
use super::*;

const SENTENCE: &str = "안녕하세요. 한글 편집 검증 2026, 계약 금액 1,234,567원.";

fn has_korean() -> bool {
    pdfcraft_fonts::CRAFT_FONTS.iter().any(|f| f.covers("Kore") || f.covers("Hang"))
}

/// Save actual bytes to disk, drop the edited document, then open only those saved bytes.
#[test]
fn korean_acceptance_disk_round_trip() {
    let nfd = "\u{1112}\u{1161}\u{11ab}\u{1100}\u{1173}\u{11af} 편집 2026";
    for (index, (typed, expected)) in
        [(SENTENCE, SENTENCE), (nfd, "한글 편집 2026"), ("한글 편집 契約 金額 勞動 働", "한글 편집 契約 金額 勞動 働")].into_iter().enumerate()
    {
        let mut doc = text_page("BT /F2 12 Tf 72 700 Td (ab) Tj ET");
        let before = write_full(&doc, &SaveOptions::default()).unwrap();
        let modified = doc.modified_objects();
        let result = text::replace_line(&mut doc, 0, 0, typed);
        if !has_korean() {
            let Err(EditError::Invalid(message)) = result else { panic!("expected missing-font refusal") };
            assert!(message.contains("Korean fallback font") && message.contains("CRAFT_FONTS_DIR"), "{message}");
            assert_eq!(doc.modified_objects(), modified);
            assert_eq!(write_full(&doc, &SaveOptions::default()).unwrap(), before);
            assert_eq!(text::text_lines(&doc, 0).unwrap()[0].text, "ab");
            for style in [
                BlockStyle::default(),
                BlockStyle {
                    ranges: vec![RangeStyle {
                        range: 0..typed.len(),
                        family: None,
                        resource: None,
                        size: Some(18.0),
                        color: None,
                        underline: None,
                        char_spacing: None,
                        scale: None,
                    }],
                    ..Default::default()
                },
            ] {
                let result = text::rewrite_block(&mut doc, 0, 0, Some(typed), &style);
                let Err(EditError::Invalid(message)) = result else { panic!("expected missing-font paragraph refusal") };
                assert!(message.contains("Korean fallback font") && message.contains("CRAFT_FONTS_DIR"), "{message}");
                assert_eq!(doc.modified_objects(), modified);
                assert_eq!(write_full(&doc, &SaveOptions::default()).unwrap(), before);
            }
            continue;
        }
        result.unwrap();
        let path = std::env::temp_dir().join(format!("pdfcraft-korean-{}-{index}.pdf", std::process::id()));
        std::fs::write(&path, write_incremental(&doc, &SaveOptions::default()).unwrap()).unwrap();
        drop(doc);
        let bytes = Arc::new(std::fs::read(&path).unwrap());
        let reopened = Document::open(bytes.clone()).unwrap();
        assert_eq!(text::text_lines(&reopened, 0).unwrap()[0].text, expected);
        // The viewer's search/selection text layer must agree with the editor's reader.
        // This is engine coverage; it does not stand in for a GUI/OS clipboard test.
        let mut renderer = pdfcraft_render::PageRenderer::new(bytes, Default::default());
        let rendered = renderer.render(pdfcraft_render::RenderRequest { kind: pdfcraft_render::RequestKind::Text, ..Default::default() });
        assert!(rendered.error.is_none(), "{:?}", rendered.error);
        let layer = rendered.text.unwrap();
        assert_eq!(layer.plain_text(), expected);
        let matches = layer.find("한글 편집");
        assert_eq!(matches.len(), 1);
        let hit = matches[0].clone();
        assert_eq!(layer.text_of(hit.clone()), "한글 편집");
        let rects = layer.line_rects(hit);
        assert_eq!(rects.len(), 1);
        let [x0, y0, x1, y1] = rects[0];
        assert!(x0 >= 72.0 && x1 > x0 && x1 <= 600.0 && y1 > y0 && y0 >= 0.0 && y1 <= 800.0, "{rects:?}");
        std::fs::remove_file(path).unwrap();
    }
}

/// A producer may encode conjoining jamo in ToUnicode. A style-only edit must compose them too.
#[test]
fn korean_style_only_edit_composes_source_jamo() {
    if !has_korean() {
        return;
    }
    let mut doc = text_page("BT /F1 12 Tf 72 700 Td (ab) Tj ET");
    let cmap = b"/CIDInit /ProcSet findresource begin 12 dict begin begincmap /CMapType 2 def \
        1 begincodespacerange <00> <FF> endcodespacerange \
        2 beginbfchar <61> <1112116111AB> <62> <1100117311AF> endbfchar \
        endcmap CMapName currentdict /CMap defineresource pop end end";
    let unicode = doc.add(Object::Stream(Stream::flate(Dict::new(), cmap)));
    doc.update_dict(ObjRef::new(5, 0), |d| d.set(b"ToUnicode".to_vec(), Object::Ref(unicode))).unwrap();
    assert_eq!(text::text_blocks(&doc, 0).unwrap()[0].text, "\u{1112}\u{1161}\u{11ab}\u{1100}\u{1173}\u{11af}");
    text::rewrite_block(&mut doc, 0, 0, None, &BlockStyle { size: Some(18.0), ..Default::default() }).unwrap();
    assert_eq!(text::text_blocks(&reopen(&doc), 0).unwrap()[0].text, "한글");
}

/// Composing an implicit draft must keep the existing writer's reflow and style dispatch.
#[test]
fn korean_style_only_composition_preserves_row_policy() {
    if !has_korean() {
        return;
    }
    let make = |decomposed: bool| {
        let mut doc = text_page("BT /F1 12 Tf 72 700 Td (a) Tj 0 -14 Td (b) Tj ET");
        let (a, b) = if decomposed { ("1112116111AB", "1100117311AF") } else { ("D55C", "AE00") };
        let cmap = format!(
            "/CIDInit /ProcSet findresource begin 12 dict begin begincmap /CMapType 2 def \
            1 begincodespacerange <00> <FF> endcodespacerange 2 beginbfchar <61> <{a}> <62> <{b}> endbfchar \
            endcmap CMapName currentdict /CMap defineresource pop end end"
        );
        let unicode = doc.add(Object::Stream(Stream::flate(Dict::new(), cmap.as_bytes())));
        doc.update_dict(ObjRef::new(5, 0), |d| d.set(b"ToUnicode".to_vec(), Object::Ref(unicode))).unwrap();
        doc
    };
    let style = BlockStyle { size: Some(18.0), width: Some(400.0), ..Default::default() };
    let mut nfd = make(true);
    let mut nfc = make(false);
    text::rewrite_block(&mut nfd, 0, 0, None, &style).unwrap();
    text::rewrite_block(&mut nfc, 0, 0, None, &style).unwrap();
    let lines = |doc: &Document| text::text_lines(&reopen(doc), 0).unwrap().into_iter().map(|l| (l.text, l.size)).collect::<Vec<_>>();
    assert_eq!(lines(&nfd), lines(&nfc));
}

fn invalid_range_style(range: std::ops::Range<usize>) -> BlockStyle {
    BlockStyle {
        ranges: vec![RangeStyle {
            range,
            size: Some(18.0),
            family: None,
            resource: None,
            color: None,
            underline: None,
            char_spacing: None,
            scale: None,
        }],
        ..Default::default()
    }
}

fn assert_refused_without_changes(doc: &mut Document, draft: Option<&str>, style: &BlockStyle, reason: &str) {
    let before = write_full(doc, &SaveOptions::default()).unwrap();
    let modified = doc.modified_objects();
    let result = text::rewrite_block(doc, 0, 0, draft, style);
    let Err(EditError::Invalid(message)) = result else { panic!("expected {reason}, got {result:?}") };
    assert!(message.contains(reason), "expected {reason}, got {message}");
    assert_eq!(doc.modified_objects(), modified);
    assert_eq!(write_full(doc, &SaveOptions::default()).unwrap(), before);
}

#[test]
fn korean_review_original_style_ranges_are_validated_before_composition() {
    let nfd = "\u{1112}\u{1161}\u{11ab}";
    for range in [0..999, 1..nfd.len(), 0..usize::MAX] {
        let mut doc = text_page("BT /F1 12 Tf 72 700 Td (ab) Tj ET");
        assert_refused_without_changes(&mut doc, Some(nfd), &invalid_range_style(range), "disjoint UTF-8 boundaries");
    }
}

#[test]
fn korean_review_style_only_ranges_are_validated_before_composition() {
    for range in [0..999, 1..9] {
        let mut doc = text_page("BT /F1 12 Tf 72 700 Td (a) Tj ET");
        let cmap = b"/CIDInit /ProcSet findresource begin 12 dict begin begincmap /CMapType 2 def \
            1 begincodespacerange <00> <FF> endcodespacerange 1 beginbfchar <61> <1112116111AB> endbfchar \
            endcmap CMapName currentdict /CMap defineresource pop end end";
        let unicode = doc.add(Object::Stream(Stream::flate(Dict::new(), cmap)));
        doc.update_dict(ObjRef::new(5, 0), |d| d.set(b"ToUnicode".to_vec(), Object::Ref(unicode))).unwrap();
        assert_refused_without_changes(&mut doc, None, &invalid_range_style(range), "disjoint UTF-8 boundaries");
    }
}

#[test]
fn korean_review_oversized_original_draft_is_rejected_before_composition() {
    // Normalization shrinks this below the writer's 4,000,000-byte limit. An empty style range
    // keeps the unfixed writer's failure bounded: it must report the ORIGINAL size error first,
    // before normalizing, remapping or preparing any per-character styled layout.
    let nfd = "\u{1100}\u{1161}\u{11a8}".repeat(444_445);
    assert!(nfd.len() > 4_000_000);
    let mut doc = text_page("BT /F1 12 Tf 72 700 Td (ab) Tj ET");
    assert_refused_without_changes(&mut doc, Some(&nfd), &invalid_range_style(0..0), "the replacement text is too large");
}

#[test]
fn korean_review_unchanged_flat_text_keeps_editor_style_offsets() {
    let mut doc = text_page("BT /F1 12 Tf 72 700 Td (ab-) Tj 0 -14 Td (cd) Tj ET");
    let block = text::text_blocks(&doc, 0).unwrap().remove(0);
    assert_eq!(block.text, "abcd");
    assert_eq!(block.editor_text, "ab-\ncd");
    text::rewrite_block(&mut doc, 0, 0, Some(&block.text), &invalid_range_style(4..6)).unwrap();
    assert!(text::text_lines(&reopen(&doc), 0).unwrap().iter().any(|line| line.text == "cd" && (line.size - 18.0).abs() < 0.01));
}
