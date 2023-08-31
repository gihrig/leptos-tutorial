use leptos::*;

// Demonstrate Client Code Can't Run on Server error
#[component]
pub fn App(cx: Scope) -> impl IntoView {
    view! { cx,
      <h1>"Hello, World, it works!"</h1>
      <h2>"This code is from src/app.rs"</h2>
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
