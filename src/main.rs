/* 8.0.b Global State Management - Passing Signals through Context */

// So far, we've only been working with local state in components,
// and we’ve seen how to coordinate state between parent and child
// components. On occasion, there are times where people look for a
// more general solution for global state management that can work
// throughout an application.

// In general, you do not need this chapter. The typical pattern is
// to compose your application out of components, each of which manages
// its own local state, not to store all state in a global structure.
// However, there are some cases (like theming, saving user settings,
// or sharing data between components in different parts of your UI)
// in which you may want to use some kind of global state management.

// The three best approaches to global state are:

// 1. Using the router to drive global state via the URL
// 2. Passing signals through context
// 3. Creating a global state struct and creating lenses into it with
//    `create_slice`

// --------------------------------------------------------------------
// 8.0.b Global State Management - Passing Signals through Context
// --------------------------------------------------------------------

// In the section on parent-child communication, we saw that you can use
// `provide_context` to pass signals from a parent component to a child,
// and `use_context` to read it in the child. But `provide_context` works
// across any distance. If you want to create a global signal that holds
// some piece of state, you can provide it and access it via context
// anywhere in the descendants of the component where you provide it.

// A signal provided via context only causes reactive updates where it
// is read, not in any of the components in between, so it maintains the
// power of fine-grained reactive updates, even at a distance.

// We start by creating a signal in the root of the app and providing it
// to all its children and descendants using `provide_context`.

/*
  #[component]
  fn App(cx: Scope) -> impl IntoView {
      // here we create a signal in the root that can be consumed
      // anywhere in the app.
      let (count, set_count) = create_signal(cx, 0);
      // we'll pass the setter to specific components,
      // but provide the count itself to the whole app via context
      provide_context(cx, count);

      view! { cx,
          // SetterButton is allowed to modify the count
          <SetterButton set_count/>
          // These consumers can only read from it
          // But we could give them write access by passing `set_count`
          // if we wanted
          <FancyMath/>
          <ListItems/>
      }
  }
*/

// <SetterButton/> is the kind of counter we’ve written several times
// now. (See the sandbox below if you don’t understand what I mean.)

// <FancyMath/> and <ListItems/> both consume the signal we’re providing
// via use_context and do something with it.

/*
  /// A component that does some "fancy" math with the global count
  #[component]
  fn FancyMath(cx: Scope) -> impl IntoView {
      // here we consume the global count signal with `use_context`
      let count = use_context::<ReadSignal<u32>>(cx)
          // we know we just provided this in the parent component
          .expect("there to be a `count` signal provided");
      let is_even = move || count() & 1 == 0;

      view! { cx,
          <div class="consumer blue">
              "The number "
              <strong>{count}</strong>
              {move || if is_even() {
                  " is"
              } else {
                  " is not"
              }}
              " even."
          </div>
      }
  }
*/

// Note that this same pattern can be applied to more complex state.
// If you have multiple fields you want to update independently, you
// can do that by providing some struct of signals:

/*
  #[derive(Copy, Clone, Debug)]
  struct GlobalState {
      count: RwSignal<i32>,
      name: RwSignal<String>
  }

  impl GlobalState {
      pub fn new(cx: Scope) -> Self {
          Self {
              count: create_rw_signal(cx, 0),
              name: create_rw_signal(cx, "Bob".to_string())
          }
      }
  }

  #[component]
  fn App(cx: Scope) -> impl IntoView {
      provide_context(cx, GlobalState::new(cx));

      // etc.
  }
*/

use leptos::*;

fn main() {
    leptos::mount_to_body(|cx| {
        view! { cx, <h1>"URL as Global State"</h1> }
    })
}
