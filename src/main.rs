/** 3.2a Dynamic Attributes - Dynamic Classes */
use leptos::*;

// The #[component] macro marks a function as a reusable component
// Components are the building blocks of your user interface
// They define a reusable unit of behavior
#[component]
fn App(cx: Scope) -> impl IntoView {
    // here we create a reactive signal
    // and get a (getter, setter) pair
    // signals are the basic unit of change in the framework
    // we'll talk more about them later
    let (count, set_count) = create_signal(cx, 0);

    // the `view` macro is how we define the user interface
    // it uses an HTML-like format that can accept certain Rust values
    view! { cx,
        <button
            class:red-20=move || count() % 2 == 1
            // Docs say tuple syntax required when class name contains
            // dashes, number, etc., e.g.
            // class=("red-20", move || count() % 2 == 1)
            // This does not seem to be true as of Rust 1.73 nightly
            // with Leptos 1.4.6

            // on:click will run whenever the `click` event fires
            // every event handler is defined as `on:{eventname}`

            // we're able to move `set_count` into the closure
            // because signals are Copy and 'static
            on:click=move |_| {
                set_count.update(|n| *n += 1);
            }
        >
            // text nodes in RSX should be wrapped in quotes,
            // like a normal Rust string
            "Click me: "
            {move || count()}
        </button>
    }
}

// This `main` function is the entry point into the app
// It just mounts our component to the <body>
// Because we defined it as `fn App`, we can now use it in a
// template as <App/>
fn main() {
    leptos::mount_to_body(|cx| view! { cx, <App/> })
}
