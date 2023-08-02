/* 3.9b Component Children - Manipulating Children */
use leptos::*;

// The Fragment type is basically a way of wrapping a Vec<View>. You can
// insert it anywhere into your view.

// But you can also access those inner views directly to manipulate them.
// For example, here’s a component that takes its children and turns them
// into an unordered list.

// ------------------------------------------------------------------

// Wraps Children

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    view! { cx,
      <h1>"Wraps Children"</h1>
      <p>"Creates an unordered list of its children"</p>
      <WrapsChildren>
        "A"
        "B"
        "C"
      </WrapsChildren>
    }
}

#[component]
pub fn WrapsChildren(cx: Scope, children: Children) -> impl IntoView {
    // Fragment has `nodes` field that contains a Vec<View>
    let children = children(cx)
        .nodes
        .into_iter()
        .map(|child| view! { cx, <li>{child}</li> })
        .collect_view(cx);

    view! { cx,
        <ul>{children}</ul>
    }
}

fn main() {
    leptos::mount_to_body(|cx| view! { cx, <App/> })
}
