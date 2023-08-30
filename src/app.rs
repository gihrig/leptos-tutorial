use leptos::*;

// Demonstrate DOM Mutation During Rendering error
// Does not panic as stated in the text (main.rs) but
// always displays the "Loading..." if branch.
#[component]
pub fn App(cx: Scope) -> impl IntoView {
    let (loaded, set_loaded) = create_signal(cx, false);

    // create_effect only runs on the client
    create_effect(cx, move |_| {
        // do something like reading from localStorage
        // set_loaded(true); <--- Error: Stuck in "loading...
        // Solution: use requestAnimationFrame to update state
        request_animation_frame(move || set_loaded(true));
    });

    move || {
        if loaded() {
            view! { cx, <p>"Hello, world!"</p> }.into_any()
        } else {
            view! { cx, <div class="loading">"Loading..."</div> }.into_any()
        }
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
