/* 6.3 Async - <Transition> */

// Working with async

// So far we’ve only been working with synchronous user interfaces:
// You provide some input, the app immediately processes it and updates
// the interface. This is great, but is a tiny subset of what web
// applications do. In particular, most web apps have to deal with some
// kind of asynchronous data loading, usually loading something from an
// API.

// Asynchronous data is notoriously hard to integrate with the
// synchronous parts of your code. Leptos provides a cross-platform
// `spawn_local` function that makes it easy to run a Future, but
// there’s much more to it than that.
// See: https://docs.rs/leptos/latest/leptos/fn.spawn_local.html

// In this chapter, we’ll see how Leptos helps smooth out that process
// for you.

// ------------------------------------------------------------------

// <Transition>

// You’ll notice in the <Suspense/> example that if you keep reloading
// the data, it keeps flickering back to "Loading...". Sometimes this
// is fine. For other times, there’s <Transition/>.

// <Transition/> behaves exactly the same as <Suspense/>, but instead
// of falling back every time, it only shows the fallback the first
// time. On all subsequent loads, it continues showing the old data
// until the new data are ready. This can be really handy to prevent
// the flickering effect, and to allow users to continue interacting
// with your application.

// This example shows how you can create a simple tabbed contact list
// with <Transition/>. When you select a new tab, it continues showing
// the current contact until the new data loads. This can be a much
// better user experience than constantly falling back to a loading
// message.

// --------------------------------------------------------------------
// 6.3 Async - <Transition> - Final Example
// --------------------------------------------------------------------

use gloo_timers::future::TimeoutFuture;
use leptos::*;

async fn important_api_call(id: usize) -> String {
    TimeoutFuture::new(1_000).await;
    match id {
        0 => "Alice",
        1 => "Bob",
        2 => "Carol",
        _ => "User not found",
    }
    .to_string()
}

#[component]
fn App(cx: Scope) -> impl IntoView {
    let (tab, set_tab) = create_signal(cx, 0);

    // this will reload every time `tab` changes
    let user_data =
        create_resource(
            cx,
            tab,
            |tab| async move { important_api_call(tab).await },
        );

    view! { cx,
        <div class="buttons">
            <button
                on:click=move |_| set_tab(0)
                class:selected=move || tab() == 0
            >
                "Tab A"
            </button>
            <button
                on:click=move |_| set_tab(1)
                class:selected=move || tab() == 1
            >
                "Tab B"
            </button>
            <button
                on:click=move |_| set_tab(2)
                class:selected=move || tab() == 2
            >
                "Tab C"
            </button>
            // Shows "Loading..." on page load and transition
            // Seems to defeat the stated purpose of <Transition/>
            // {move || if user_data.loading().get() { "Loading..." } else { "" }}
        </div>
        // Transition seems to be broken in this example.
        // It works as expected in `hackernews` and `hackernews_axum`
        // examples

        // The <Transition fallback never shows ???
        // Does not perform as discussed above. It does not "show
        // initially".
        // The second feature "subsequent reloads, the current child will
        // continue showing" does function as expected.
        // fallback seems to be useless. Might as well be disabled.
        // `fallback=move || {}`
        <Transition
        fallback=move || {view! { cx, <p>"Loading..."</p> }}>
            <p>{move || user_data.read(cx)}</p>
        </Transition>
    }
}

fn main() {
    leptos::mount_to_body(|cx| view! { cx, <App/> })
}
