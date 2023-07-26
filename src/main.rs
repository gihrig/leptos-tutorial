/** 3.3a Components and Props - Component Props */
use leptos::*;

// In Leptos, you define props by giving additional
// arguments (e.g. `progress`) to the component function.
// Components use PascalCase
#[component]
fn ProgressBar(cx: Scope, progress: ReadSignal<i32>) -> impl IntoView {
    view! { cx,
      <progress
        max="50"
        value=progress
      />
    }
}

#[component]
fn App(cx: Scope) -> impl IntoView {
    let (count, set_count) = create_signal(cx, 0);
    view! { cx,
        <button
            on:click=move |_| {
                set_count.update(|n| *n += 1);
            }
        >
          "Click me"
        </button>

        // Use ProgressBar
        <ProgressBar progress=count />
    }
}

fn main() {
    leptos::mount_to_body(|cx| view! { cx, <App/> })
}
