/** 3.5a Forms and Inputs - Controlled Inputs */
use leptos::*;

// In a "controlled input," the framework controls the state
// of the input element. On every input event, it updates a
// local signal that holds the current state, which in turn
// updates the value prop of the input.

// There are two important things to remember:
//
// 1. The input event fires on (almost) every change to the
//    element, while the change event fires (more or less)
//    when you unfocus the input. You probably want on:input,
//    but we give you the freedom to choose.
// 2. The value _attribute_ only sets the initial value of the
//    input, i.e., it only updates the input up to the point
//    that you begin typing. The value _property_ continues
//    updating the input after that. You usually want to set
//    prop:value for this reason.

#[component]
fn App(cx: Scope) -> impl IntoView {
    view! { cx,
        <h2>"Controlled Component"</h2>
        <ControlledComponent/>
    }
}

#[component]
fn ControlledComponent(cx: Scope) -> impl IntoView {
    // create a signal to hold the value
    let (name, set_name) = create_signal(cx, "Controlled".to_string());

    view! { cx,
        <input type="text"
            on:input=move |ev| {
                // event_target_value is a Leptos helper function
                // it functions the same way as event.target.value
                // in JavaScript, but smooths out some of the typecasting
                // necessary to make this work in Rust
                set_name(event_target_value(&ev));
            }

            // the `prop:` syntax lets you update a DOM property,
            // rather than an attribute.
            prop:value=name
        />
        <p>"Name is: " {name}</p>
    }
}

fn main() {
    leptos::mount_to_body(|cx| view! { cx, <App/> })
}
