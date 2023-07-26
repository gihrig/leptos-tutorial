/** 3.2d Dynamic Attributes - Derived Signals */
use leptos::*;

/**
* Derived signals let you create reactive computed
* values that can be used in multiple places in your
* application with minimal overhead.

   Note: Using a derived signal like this means that
   the calculation runs once per signal change per
   place we access double_count; in other words,
   twice. This is a very cheap calculation, so that’s
   fine. We’ll look at memos in a later chapter, which
   are designed to solve this problem for expensive
   calculations.
*/

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

    // a "derived signal" is a function that accesses other signals
    // we can use this to create reactive values that depend on the
    // values of one or more other signals
    let double_count = move || count() * 2;

    // the `view` macro is how we define the user interface
    // it uses an HTML-like format that can accept certain Rust values
    view! { cx,
        <button
            // the class: syntax reactively updates a single class
            // here, we'll set the `red` class when `count` is odd
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
        // NOTE: self-closing tags like <br> need an explicit `/`
        <br/>
        <br/>

        // 3.2d Derived Signals
        <progress
            max="50"
            // signals are functions, this is equal to
            // `move || count.get()`
            value=count
        />
        <br/>

        <progress
            max="50"
            // derived signals are functions, so they can also
            // reactively update the DOM
            // First use of `double_count
            value=double_count
        />
        <p>
            "Count: "
            {count}
        </p>
        <p>
            "Double Count: "
            // Second use of `double_count
            {double_count}
        </p>
    }
}

// This `main` function is the entry point into the app
// It just mounts our component to the <body>
// Because we defined it as `fn App`, we can now use it in a
// template as <App/>
fn main() {
    leptos::mount_to_body(|cx| view! { cx, <App/> })
}
