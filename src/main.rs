/* 8.0.c Global State Management - Create a Global State Struct and Slices */

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
// 8.0.c Global State Management - Create a Global State Struct and Slices
// --------------------------------------------------------------------

// You may find it cumbersome to wrap each field of a structure in a
// separate signal like this. In some cases, it can be useful to create
// a plain struct with non-reactive fields, and then wrap that in a
// signal.

/*
  #[derive(Copy, Clone, Debug, Default)]
  struct GlobalState {
      count: i32,
      name: String
  }

  #[component]
  fn App(cx: Scope) -> impl IntoView {
      provide_context(cx, create_rw_signal(GlobalState::default()));

      // etc.
  }
*/

// But there’s a problem: because our whole state is wrapped in one
// signal, updating the value of one field will cause reactive updates
// in parts of the UI that only depend on the other.

/*
  let state = expect_context::<RwSignal<GlobalState>>(cx);
  view! { cx,
      <button on:click=move |_| state.update(|n| *n += 1)>"+1"</button>
      <p>{move || state.with(|state| state.name.clone())}</p>
  }
*/

// In this example, clicking the button will cause the text inside <p> to be updated, cloning state.name again! Because signals are the atomic unit of reactivity, updating any field of the signal triggers updates to everything that depends on the signal.

// There’s a better way. You can take fine-grained, reactive slices by
// using `create_memo`
// https://docs.rs/leptos/latest/leptos/fn.create_memo.html
// or `create_slice`
// https://docs.rs/leptos/latest/leptos/fn.create_slice.html
// (which uses create_memo but also provides a setter).
// “Memoizing” a value means creating a new reactive value which will
// only update when it changes. “Memoizing a slice” means creating a
// new reactive value which will only update when some field of the
// state struct updates.

// Here, instead of reading from the state signal directly, we create
// “slices” of that state with fine-grained updates via create_slice.
// Each slice signal only updates when the particular piece of the
// larger struct it accesses updates. This means you can create a
// single root signal, and then take independent, fine-grained slices
// of it in different components, each of which can update without
// notifying the others of changes.

/*
  /// A component that updates the count in the global state.
  #[component]
  fn GlobalStateCounter(cx: Scope) -> impl IntoView {
      let state = expect_context::<RwSignal<GlobalState>>(cx);

      // `create_slice` lets us create a "lens" into the data
      let (count, set_count) = create_slice(
          cx,
          // we take a slice *from* `state`
          state,
          // our getter returns a "slice" of the data
          |state| state.count,
          // our setter describes how to mutate that slice, given
          // a new value
          |state, n| state.count = n,
      );

      view! { cx,
          <div class="consumer blue">
              <button
                  on:click=move |_| {
                      set_count(count() + 1);
                  }
              >
                  "Increment Global Count"
              </button>
              <br/>
              <span>"Count is: " {count}</span>
          </div>
      }
  }
*/

// Clicking this button only updates state.count, so if we create
// another slice somewhere else that only takes state.name, clicking
// the button won’t cause that other slice to update. This allows you
// to combine the benefits of a top-down data flow and of fine-grained
// reactive updates.

// Note: There are some significant drawbacks to this approach. Both
// signals and memos need to own their values, so a memo will need to
// clone the field’s value on every change. The most natural way to
// manage state in a framework like Leptos is always to provide signals
// that are as locally-scoped and fine-grained as they can be, not to
// hoist everything up into global state. But when you do need some
// kind of global state, `create_slice` can be a useful tool.

use leptos::*;

fn main() {
    leptos::mount_to_body(|cx| {
        view! { cx, <h1>"URL as Global State"</h1> }
    })
}
