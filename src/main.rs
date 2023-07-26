/** 3.3e Components and Props - Generic Props */
use leptos::*;

// We began with two counters, one driven by count, and
// one by the derived signal double_count. Let’s recreate
// that by using double_count as the progress prop on
// another <ProgressBar/>.

/*
  // This is the same as the previous example.
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

  #[component]
  fn App(cx: Scope) -> impl IntoView {
      let (count, set_count) = create_signal(cx, 0);
      let double_count = move || count() * 2;

      view! { cx,
        <button on:click=move |_| { set_count.update(|n| *n += 1); }>
            "Click me"
        </button>
        <ProgressBar progress=count/>
        // add a second progress bar
        <ProgressBar progress=double_count/>
                              ^^^^^^^^^^^^ <-- mismatched types expected leptos::ReadSignal<i32> found i32
    }
  }
*/

// This won’t compile. It should be pretty easy to understand
// why: we’ve declared that the progress prop takes ReadSignal<i32>,
// and double_count is not ReadSignal<i32>. As rust-analyzer will
// tell you, its type is || -> i32, i.e., it’s a closure that
// returns an i32.

// There are a couple ways to handle this. One would be to say:
// “Well, I know that a ReadSignal is a function, and I know that
// a closure is a function; maybe I could just take any function?”
// If you’re savvy, you may know that both these implement the
// trait Fn() -> i32. So you could use a generic component:

// Note that generic component props can’t be specified with an
// impl yet (progress: impl Fn() -> i32 + 'static,), in part
// because they’re actually used to generate a struct
// ProgressBarProps, and struct fields cannot be impl types.
// The #[component] macro may be further improved in the future
// to allow inline impl generic props.

#[component]
/*
fn ProgressBar<F>(cx: Scope, #[prop(default = 100)] max: u16, progress: F) -> impl IntoView
where
    F: Fn() -> i32 + 'static,
{
*/
//This generic can also be specified inline:
fn ProgressBar<F: Fn() -> i32 + 'static>(
    cx: Scope,
    #[prop(default = 100)] max: u16,
    progress: F,
) -> impl IntoView {
    view! { cx,
      <progress
        max=max
        value=progress
      />
    }
}

#[component]
fn App(cx: Scope) -> impl IntoView {
    let (count, set_count) = create_signal(cx, 0);
    let double_count = move || count() * 2;

    view! { cx,
        <button
            on:click=move |_| {
                set_count.update(|n| *n += 1);
            }
        >
          "Click me"
        </button>
        <br />
        <br />
        <ProgressBar progress=count/>
        <br />
        // add a second progress bar
        <ProgressBar progress=double_count/>
    }
}

// This is a perfectly reasonable way to write this
// component: progress now takes any value that implements
// this Fn() trait.

fn main() {
    leptos::mount_to_body(|cx| view! { cx, <App/> })
}
