/** 3.3f Components and Props - into Props */
use leptos::*;

// There’s one more way we could implement this, and it
// would be to use #[prop(into)]. This attribute
// automatically calls .into() on the values you pass as
// props, which allows you to easily pass props with
// different values.

// In this case, it’s helpful to know about the Signal
// type. Signal is an enumerated type that represents
// any kind of readable reactive signal. It can be useful
// when defining APIs for components you’ll want to reuse
// while passing different sorts of signals.
// The MaybeSignal type is useful when you want to be able
// to take either a static or reactive value.

#[component]
fn ProgressBar(
    cx: Scope,
    #[prop(default = 100)] max: u16,
    #[prop(into)] progress: Signal<i32>,
) -> impl IntoView {
    view! { cx,
      <progress
        max=max
        value=progress
      />
    }
}

#[component]
fn App(cx: Scope) -> impl IntoView {
    let (count, set_count) = create_signal(cx, 0);
    let double_count = move || count() * 2;

    view! { cx,
        <button
            on:click=move |_| {
                set_count.update(|n| *n += 1);
            }
        >
          "Click me"
        </button>
        <br />
        <br />
        // .into() converts `ReadSignal` to `Signal`
        <ProgressBar progress=count/>
        <br />
        // use `Signal::derive()` to wrap a derived signal
        <ProgressBar progress=Signal::derive(cx, double_count)/>
    }
}

fn main() {
    leptos::mount_to_body(|cx| view! { cx, <App/> })
}
