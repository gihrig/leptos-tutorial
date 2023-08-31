use leptos::*;

// Demonstrate Client Code Can't Run on Server error
#[component]
pub fn App(cx: Scope) -> impl IntoView {
    use gloo_storage::Storage;
    // panicked at 'cannot call wasm-bindgen imported functions on non-wasm targets'
    // let storage = gloo_storage::LocalStorage::raw();
    // leptos::log!("{storage:?}");

    // Solution wrap `storage...` in create_effect
    create_effect(cx, move |_| {
        let storage = gloo_storage::LocalStorage::raw();
        leptos::log!("{storage:?}");
    });

    view! { cx,
      <h1>"Hello, World, it works!"</h1>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage(cx: Scope) -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(cx, 0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! { cx,
        <h1>"Welcome to Leptos!"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
    }
}
