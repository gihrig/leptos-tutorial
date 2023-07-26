/** 3.3c Components and Props - Optional Props */
use leptos::*;

// Previously the max setting was hard-coded. Let’s take
// that as a prop too. But let’s add a catch: let’s make
// this prop optional by annotating the particular
// argument to the component function with
// #[prop(optional)].

#[component]
fn ProgressBar(
    cx: Scope,
    // mark this prop optional
    // you can specify it or not when you use <ProgressBar/>
    #[prop(optional)] max: u16,
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
// can omit max to use the default value (i.e.,
// <ProgressBar value=count/>). The default value on an
// optional is its Default::default() value, which for a u16
// is going to be 0. In the case of a progress bar, a max
// value of 0 is not very useful.

// In the next section we'll give it a particular default
// value instead.

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
