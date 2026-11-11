//! Real inline TextEdit regressions for committed decomposed-Hangul input.
use super::*;
use egui::Event;
use egui_kittest::Harness;

const NFD: &str = "\u{1112}\u{1161}\u{11ab}\u{1100}\u{1173}\u{11af}";

fn editor(text: &str) -> Harness<'static, crate::PdfCraftApp> {
    let text = text.to_owned();
    let mut h = Harness::builder().with_size(egui::vec2(1400.0, 900.0)).build_eframe(move |_cc| {
        let (mut app, ed) = super::tests::preview_app_with_text(&text);
        app.set_option("language", "en").unwrap();
        assert!(app.execute("edit.edit_text"));
        app.views[0].line_editor = Some(ed);
        app
    });
    h.run_steps(6);
    assert!(h.ctx.memory(|memory| memory.has_focus(input_id(crate::panes::Place::Main))), "the real inline editor owns input");
    h
}

fn cursor(h: &Harness<'_, crate::PdfCraftApp>) -> egui::text::CCursorRange {
    egui::TextEdit::load_state(&h.ctx, input_id(crate::panes::Place::Main)).unwrap().cursor.char_range().unwrap()
}

fn select(h: &mut Harness<'_, crate::PdfCraftApp>, from: usize, to: usize) {
    let id = input_id(crate::panes::Place::Main);
    let mut state = egui::TextEdit::load_state(&h.ctx, id).unwrap();
    state.cursor.set_char_range(Some(egui::text::CCursorRange::two(egui::text::CCursor::new(from), egui::text::CCursor::new(to))));
    egui::TextEdit::store_state(&h.ctx, id, state);
    h.run_steps(1);
}

#[test]
fn hangul_committed_text_replaces_selection_as_syllables_and_keeps_caret() {
    let mut h = editor("Replace me");
    select(&mut h, 0, "Replace me".chars().count());
    h.event(Event::Text(format!("{NFD} 편집")));
    h.run_steps(2);
    assert_eq!(h.state().views[0].line_editor.as_ref().unwrap().text, "한글 편집");
    let caret = cursor(&h);
    assert_eq!(usize::from(caret.primary.index), 5);
    assert_eq!(usize::from(caret.secondary.index), 5);
    h.event(Event::Text("!".into()));
    h.run_steps(2);
    assert_eq!(h.state().views[0].line_editor.as_ref().unwrap().text, "한글 편집!");
}

#[test]
fn hangul_paste_before_suffix_keeps_cursor_and_style_byte_range() {
    let mut h = editor("AB suffix");
    select(&mut h, 3, 3);
    h.event(Event::Paste(format!("{NFD} ")));
    h.run_steps(2);
    assert_eq!(h.state().views[0].line_editor.as_ref().unwrap().text, "AB 한글 suffix");
    assert_eq!(usize::from(cursor(&h).primary.index), 6);
    h.event(Event::Text("X".into()));
    h.run_steps(2);
    assert_eq!(h.state().views[0].line_editor.as_ref().unwrap().text, "AB 한글 Xsuffix");
    select(&mut h, 7, 13);
    let ed = h.state_mut().views[0].line_editor.as_mut().unwrap();
    ed.choose_size(18.0);
    let style = ed.style();
    assert_eq!(style.ranges.len(), 1);
    assert_eq!(ed.text.get(style.ranges[0].range.clone()), Some("suffix"));
    assert_eq!(style.ranges[0].range, "AB 한글 X".len().."AB 한글 Xsuffix".len());
}

#[test]
fn hangul_active_ime_preedit_remains_decomposed_across_frames() {
    let mut h = editor("AB suffix");
    // The bundled syllable font intentionally lacks raw conjoining-jamo glyphs. This test checks
    // the IME protocol, which must retain its preedit buffer even when that font cannot draw it.
    h.ctx.set_missing_glyph_policy(egui::epaint::text::MissingGlyphPolicy::Tofu);
    select(&mut h, 3, 3);
    let event = Event::Ime(egui::ImeEvent::Preedit { text: NFD.into(), active_range_chars: Some(0..6) });
    h.event(event.clone());
    h.run_steps(1);
    assert!(h.ctx.input(|input| input.events.contains(&event)), "the IME event and its range are unchanged");
    assert_eq!(h.state().views[0].line_editor.as_ref().unwrap().text, format!("AB {NFD}suffix"));
    h.run_steps(2);
    assert_eq!(h.state().views[0].line_editor.as_ref().unwrap().text, format!("AB {NFD}suffix"));
    h.event(Event::Ime(egui::ImeEvent::Commit("한글".into())));
    h.run_steps(2);
    assert_eq!(h.state().views[0].line_editor.as_ref().unwrap().text, "AB 한글suffix");
    assert_eq!(usize::from(cursor(&h).primary.index), 5);
}

#[test]
fn hangul_unfocused_editor_leaves_other_field_events_unchanged() {
    let ctx = egui::Context::default();
    let editor = input_id(crate::panes::Place::Main);
    let other = egui::Id::unique("another-field");
    let events = vec![Event::Text(NFD.into()), Event::Paste(NFD.into())];
    let mut output = ctx.run_ui(egui::RawInput { events: events.clone(), ..Default::default() }, |ui| {
        ui.memory_mut(|memory| memory.request_focus(other));
        compose_committed_input(ui.ctx(), editor);
        assert_eq!(ui.input(|input| input.events.clone()), events);
    });
    output.textures_delta.clear();
}

#[test]
fn hangul_focused_editor_preserves_ime_and_noncomposing_events() {
    let ctx = egui::Context::default();
    let editor = input_id(crate::panes::Place::Main);
    let events = vec![
        Event::Ime(egui::ImeEvent::Preedit { text: NFD.into(), active_range_chars: Some(1..4) }),
        Event::Ime(egui::ImeEvent::Commit(NFD.into())),
        Event::Ime(egui::ImeEvent::DeleteSurrounding { before_chars: 1, after_chars: 2 }),
        Event::Ime(egui::ImeEvent::DeleteSurroundingBytes { before_bytes: 3, after_bytes: 6 }),
        Event::Text("한글 漢字".into()),
        Event::Paste("\u{1112}".into()),
        Event::Copy,
    ];
    let mut output = ctx.run_ui(egui::RawInput { events: events.clone(), ..Default::default() }, |ui| {
        ui.memory_mut(|memory| memory.request_focus(editor));
        compose_committed_input(ui.ctx(), editor);
        assert_eq!(ui.input(|input| input.events.clone()), events);
    });
    output.textures_delta.clear();
}
