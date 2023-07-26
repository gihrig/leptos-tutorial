/** 3.3h Components and Props - Documenting Components */
use leptos::*;

// This is one of the least essential but most important
// sections of this book. It’s not strictly necessary to
// document your components and their props. It may be very
// important, depending on the size of your team and your app.
// But it’s very easy, and bears immediate fruit.

// To document a component and its props, you can simply add
// doc comments on the component function, and each one of
// the props:

/// Display a progress bar.
#[component]
fn ProgressBar(
    /// Leptos components require a `cx` scope.
    cx: Scope,
    /// The maximum value for the progress bar.
    #[prop(default = 100)]
    max: u16,
    /// The current progress toward the max value.
    #[prop(into)]
    progress: Signal<i32>,
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
        <button on:click=move |_| { set_count.update(|n| *n += 1); }>
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

// That’s all you need to do. These behave like ordinary Rust
// doc comments, except that you can document individual
// component props, which can’t be done with Rust function
// arguments.

// This will automatically generate documentation for your
// component, its Props type, and each of the fields used to
// add props. It can be a little hard to understand how
// powerful this is until you hover over the component name
// or props and see the power of the #[component] macro
// combined with rust-analyzer here.

fn main() {
    leptos::mount_to_body(|cx| view! { cx, <App/> })
}
