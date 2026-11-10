use super::*;
use crate::edit::{Mark, MarkRange};

struct State {
    edit: crate::TextEdit,
    opened: Vec<String>,
}

fn view(s: &State, _: &mut Cx<State>) -> El {
    col().size(300.0, 100.0).child(
        crate::text_input(&s.edit, crate::TextStyle::new(14.0, 400.0))
            .key("field")
            .size(300.0, 40.0)
            .on_edit(|s: &mut State| &mut s.edit)
            .on_link(|s: &mut State, url: &str, _| s.opened.push(url.to_owned())),
    )
}

/// A field whose text is all one link, laid out.
fn linked(focused: bool) -> Runtime<State> {
    let mut edit = crate::TextEdit::new("docs docs docs").rich();
    edit.set_marks(vec![MarkRange::new(0..14, Mark::Link("https://example.com".into()))]);
    edit.focused = focused;
    let mut rt = Runtime::new(State { edit, opened: Vec::new() }, view, (300.0, 100.0), Env::new(true, 1.0));
    rt.frame(0.0);
    rt
}

#[test]
fn a_click_on_a_link_in_a_field_opens_it_whether_or_not_the_field_is_being_edited() {
    for focused in [false, true] {
        let mut rt = linked(focused);
        rt.pointer_down(20.0, 10.0);
        rt.pointer_up(20.0, 10.0);
        rt.frame(0.1);
        assert_eq!(rt.state.opened, ["https://example.com"], "focused: {focused}");
    }
}

#[test]
fn a_drag_across_a_link_in_a_field_selects_and_opens_nothing() {
    let mut rt = linked(true);
    rt.pointer_down(10.0, 10.0);
    rt.pointer_move(60.0, 10.0);
    rt.pointer_up(60.0, 10.0);
    rt.frame(0.1);
    assert!(rt.state.opened.is_empty());
}
