/** 3.3d Components and Props - Default Props */
use leptos::*;

// You can specify a default value other than
// Default::default() pretty simply with #[prop(default = ...).

#[component]
fn ProgressBar(
    cx: Scope,
    // optional prop with a default value
    #[prop(default = 100)] max: u16,
    progress: ReadSignal<i32>,
) -> impl IntoView {
    view! { cx,
      <progress
        max=max
        value=progress
      />
    }
}

// Now, we can use <ProgressBar max=50 value=count/>, or we
// can omit max to use the default value of 100 (i.e.,
// <ProgressBar value=count/>).

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
        <ProgressBar max=50 progress=count />
    }
}

fn main() {
    leptos::mount_to_body(|cx| view! { cx, <App/> })
}
