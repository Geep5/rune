use super::*;
use crate::edit::MarkRange;
use crate::{Mark, TextEdit};

const URL: &str = "https://example.com/docs";

#[derive(Default)]
struct State {
    edit: TextEdit,
    opened: Vec<String>,
}

fn view(s: &State, _: &mut Cx<State>) -> El {
    col().size(300.0, 100.0).child(
        crate::text_input(&s.edit, crate::TextStyle::new(14.0, 400.0))
            .key("field")
            .w(280.0)
            .on_edit(|s: &mut State| &mut s.edit)
            .on_link(|s: &mut State, url, _| s.opened.push(url.into())),
    )
}

fn field() -> TextEdit {
    let mut edit = TextEdit::new("see the docs").rich();
    edit.set_marks(vec![MarkRange { start: 8, end: 12, mark: Mark::Link(URL.into()) }]);
    edit
}

/// A link in an editable rich field opens on a plain click, as one in read-only text does;
/// before, the field took the press and its link never fired.
#[test]
fn a_click_on_a_link_in_a_text_field_opens_it() {
    let mut rt = Runtime::new(State { edit: field(), ..State::default() }, view, (300.0, 100.0), Env::new(true, 1.0));
    rt.frame(0.0);
    let (x, y) = rt.find(URL).expect("the link is drawn");
    let (x, y) = (x + 2.0, y + 2.0);
    rt.pointer_move(x, y);
    rt.pointer_down(x, y);
    rt.pointer_up(x, y);
    rt.frame(0.0);
    assert_eq!(rt.state.opened, [URL]);
}

/// Dragging across the link selects its text: an edit, not a click.
#[test]
fn a_drag_across_a_link_in_a_text_field_selects_without_opening() {
    let mut rt = Runtime::new(State { edit: field(), ..State::default() }, view, (300.0, 100.0), Env::new(true, 1.0));
    rt.frame(0.0);
    let (x, y) = rt.find(URL).expect("the link is drawn");
    let (x, y) = (x + 2.0, y + 2.0);
    rt.pointer_move(x, y);
    rt.pointer_down(x, y);
    rt.pointer_move(x + 20.0, y);
    rt.frame(0.0);
    rt.pointer_up(x + 20.0, y);
    rt.frame(0.0);
    assert!(rt.state.opened.is_empty(), "{:?}", rt.state.opened);
}
