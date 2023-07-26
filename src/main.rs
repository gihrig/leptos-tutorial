/** 3.3g Components and Props - Optional Generic Props */
use leptos::*;

// Note that you can’t specify optional generic props for
// a component. Let’s see what would happen if you try:

/*
  #[component]
  fn ProgressBar<F: Fn() -> i32 + 'static>(
      cx: Scope,
      #[prop(optional)] progress: Option<F>,
  ) -> impl IntoView {
      progress.map(|progress| {
          view! { cx,
              <progress
                  max=100
                  value=progress
              />
          }
      })
  }

  #[component]
  pub fn App(cx: Scope) -> impl IntoView {
      view! { cx,
          <ProgressBar/>
      }
}
*/

// Rust helpfully gives the error

// xx |         <ProgressBar/>
//    |          ^^^^^^^^^^^ cannot infer type of the type parameter `F` declared on the function `ProgressBar`
//    |
// help: consider specifying the generic argument
//    |
// xx |         <ProgressBar::<F>/>
//    |                     +++++

// There are just two problems:

// Leptos’s view macro doesn’t support specifying a generic
// on a component with this turbofish syntax.

// Even if you could, specifying the correct type here is not
// possible; closures and functions in general are un-nameable
// types. The compiler can display them with a shorthand, but
// you can’t specify them.

// However, you can get around this by providing a concrete
// type using Box<dyn _> or &dyn _:

// >>> This example doesn't do anything - What's the point ??? <<<

#[component]
fn ProgressBar(
    cx: Scope,
    #[prop(optional)] progress: Option<Box<dyn Fn() -> i32>>,
) -> impl IntoView {
    progress.map(|progress| {
        view! { cx,
            <progress
                max=100
                value=progress
            />
        }
    })
}

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    view! { cx,
        <ProgressBar/>
    }
}

// Because the Rust compiler now knows the concrete type of
// the prop, and therefore its size in memory even in the
// None case (as above), this compiles fine.

// In this particular case, &dyn Fn() -> i32 will cause
// lifetime issues,

// >>> Is that why the example does nothing? <<<

// but in other cases, it may be a possibility.

fn main() {
    leptos::mount_to_body(|cx| view! { cx, <App/> })
}
