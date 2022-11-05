use sycamore::prelude::*;

use crate::state::EntryData;

use super::modal::{EditModal, Modal};

#[component(inline_props)]
fn EntryLine<G: Html>(cx: Scope, line: String) -> View<G> {
    let mut line = line;
    if line.starts_with("# ") {
        line = line.replace("# ", "");
        view! { cx,
            h3 { (line.clone()) }
        }
    } else {
        view! {
            cx,
            p{ (line.clone()) }
        }
    }
}

#[component(inline_props)]
pub fn Entry<G: Html>(cx: Scope, entry_data: EntryData) -> View<G> {
    let entry_data = create_ref(cx, entry_data);
    let lines = entry_data
        .value
        .map(cx, |x| x.lines().map(|l| l.to_string()).collect());
    let time = format!("at {}", entry_data.time.format("%R"));
    let editing = create_signal(cx, false);
    let edit = |_| editing.set(true);
    view! { cx,
        div(class="entry"){
            p(class="entry-time") { (time)}
            div(on:dblclick = edit, class="entry-content"){
                Indexed(iterable = lines, view = |cx, line|
                    view!{cx,
                    EntryLine(line = line)
                    })
            }
            Modal(visibility=editing) {
                EditModal(value=entry_data.value.clone())
            }
        }
    }
}
