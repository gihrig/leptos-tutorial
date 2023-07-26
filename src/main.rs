/** 3.3b Components and Props - Reactive and Static Props */
use leptos::*;

// You’ll notice that throughout this example, `progress`
// takes a reactive ReadSignal<i32>, and not a plain i32.
// This is very important.

// Component props have no special meaning attached to them.
// A component is simply a function that runs once to set up
// the user interface. The only way to tell the interface to
// respond to changing is to pass it a signal type. So if you
// have a component property that will change over time, like
// our progress, it should be a signal.

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
