/* 4.2f Reactivity - Final Example */

use leptos::html::Input;
use leptos::*;

#[component]
fn App(cx: Scope) -> impl IntoView {
    // Just making a visible log here
    // You can ignore this...
    let log = create_rw_signal::<Vec<String>>(cx, vec![]);
    let logged = move || log().join("\n");
    provide_context(cx, log);

    view! { cx,
        <CreateAnEffect/>
        <pre>{logged}</pre>
    }
}

#[component]
fn CreateAnEffect(cx: Scope) -> impl IntoView {
    let (first, set_first) = create_signal(cx, String::new());
    let (last, set_last) = create_signal(cx, String::new());
    let (use_last, set_use_last) = create_signal(cx, true);

    // this will add the name to the log
    // any time one of the source signals changes
    create_effect(cx, move |_| {
        log(
            cx,
            if use_last() {
                format!("{}  {}", first(), last())
            } else {
                first()
            },
        )
    });

    view! { cx,
        <h1><code>"create_effect"</code> " Version"</h1>
        <form>
            <label>
                "First Name"
                <input type="text" name="first" prop:value=first
                    on:change=move |ev| set_first(event_target_value(&ev))
                />
            </label>
            <label>
                "Last Name"
                <input type="text" name="last" prop:value=last
                    on:change=move |ev| set_last(event_target_value(&ev))
                />
            </label>
            <label>
                "Show Last Name"
                <input type="checkbox" name="use_last" prop:checked=use_last
                    on:change=move |ev| set_use_last(event_target_checked(&ev))
                />
            </label>
        </form>
    }
}

#[component]
fn ManualVersion(cx: Scope) -> impl IntoView {
    let first = create_node_ref::<Input>(cx);
    let last = create_node_ref::<Input>(cx);
    let use_last = create_node_ref::<Input>(cx);

    let mut prev_name = String::new();
    let on_change = move |_| {
        log(cx, "      listener");
        let first = first.get().unwrap();
        let last = last.get().unwrap();
        let use_last = use_last.get().unwrap();
        let this_one = if use_last.checked() {
            format!("{} {}", first.value(), last.value())
        } else {
            first.value()
        };

        if this_one != prev_name {
            log(cx, &this_one);
            prev_name = this_one;
        }
    };

    view! { cx,
        <h1>"Manual Version"</h1>
        <form on:change=on_change>
            <label>
                "First Name"
                <input type="text" name="first"
                    node_ref=first
                />
            </label>
            <label>
                "Last Name"
                <input type="text" name="last"
                    node_ref=last
                />
            </label>
            <label>
                "Show Last Name"
                <input type="checkbox" name="use_last"
                    checked
                    node_ref=use_last
                />
            </label>
        </form>
    }
}

#[component]
fn EffectVsDerivedSignal(cx: Scope) -> impl IntoView {
    let (my_value, set_my_value) = create_signal(cx, String::new());
    // Don't do this.
    /*let (my_optional_value, set_optional_my_value) = create_signal(cx, Option::<String>::None);

    create_effect(cx, move |_| {
        if !my_value.get().is_empty() {
            set_optional_my_value(Some(my_value.get()));
        } else {
            set_optional_my_value(None);
        }
    });*/

    // Do this
    let my_optional_value =
        move || (!my_value.with(String::is_empty)).then(|| Some(my_value.get()));

    view! { cx,
        <input
            prop:value=my_value
            on:input= move |ev| set_my_value(event_target_value(&ev))
        />

        <p>
            <code>"my_optional_value"</code>
            " is "
            <code>
                <Show
                    when=move || my_optional_value().is_some()
                    fallback=|_cx| view! { cx, "None" }
                >
                    "Some(\"" {my_optional_value().unwrap()} "\")"
                </Show>
            </code>
        </p>
    }
}

/*
#[component]
pub fn Show<F, W, IV>(
    /// The scope the component is running in
    cx: Scope,
    /// The components Show wraps
    children: Box<dyn Fn(Scope) -> Fragment>,
    /// A closure that returns a bool that determines whether this thing runs
    when: W,
    /// A closure that returns what gets rendered if the when statement is false
    fallback: F,
) -> impl IntoView
where
    W: Fn() -> bool + 'static,
    F: Fn(Scope) -> IV + 'static,
    IV: IntoView,
{
    let memoized_when = create_memo(cx, move |_| when());

    move || match memoized_when.get() {
        true => children(cx).into_view(cx),
        false => fallback(cx).into_view(cx),
    }
}
*/

fn log(cx: Scope, msg: impl std::fmt::Display) {
    let log = use_context::<RwSignal<Vec<String>>>(cx).unwrap();
    log.update(|log| log.push(msg.to_string()));
}

fn main() {
    leptos::mount_to_body(|cx| view! { cx, <App/> })
}
