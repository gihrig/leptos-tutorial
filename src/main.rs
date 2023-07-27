/** 3.4a Iteration - Static Views with Vec<_> */
use leptos::*;

// Sometimes you need to show an item repeatedly, but the
// list you’re drawing from does not often change. In this
// case, it’s important to know that you can insert any
// Vec<IV> where IV: IntoView into your view. In other
// words, if you can render T, you can render Vec<T>.

/*
  let values = vec![0, 1, 2];
  view! { cx,
      // this will just render "012"
      <p>{values.clone()}</p>
      // or we can wrap them in <li>
      <ul>
          {values.into_iter()
              .map(|n| view! { cx, <li>{n}</li>})
              .collect::<Vec<_>>()}
      </ul>
  }
*/

// Leptos also provides a .collect_view(cx) helper function
// that allows you to collect any iterator of T: IntoView
// into Vec<View>.

/*
  let values = vec![0, 1, 2];
  view! { cx,
      // this will just render "012"
      <p>{values.clone()}</p>
      // or we can wrap them in <li>
      <ul>
          {values.into_iter()
              .map(|n| view! { cx, <li>{n}</li>})
              .collect_view(cx)}
      </ul>
}
*/

// The fact that the list is static doesn’t mean the interface
// needs to be static. You can render dynamic items as part of
// a static list.

/*
  // create a list of N signals
  let counters = (1..=length).map(|idx| create_signal(cx, idx));

  // each item manages a reactive view
  // but the list itself will never change
  let counter_buttons = counters
      .map(|(count, set_count)| {
          view! { cx,
              <li>
                  <button
                      on:click=move |_| set_count.update(|n| *n += 1)
                  >
                      {count}
                  </button>
              </li>
          }
      })
      .collect_view(cx);

  view! { cx,
      <ul>{counter_buttons}</ul>
  }
*/

// You can render a Fn() -> Vec<_> reactively as well. But
// note that every time it changes, this will re-render every
// item in the list. This is quite inefficient!

// This example will show you a method for mostly-static lists,
// using Rust iterators

#[component]
fn App(cx: Scope) -> impl IntoView {
    view! { cx,
        <h1>"Iteration"</h1>
        <h2>"Static List"</h2>
        <p>"Use this pattern if the list itself is static."</p>
        <StaticList length=5/>
    }
}

/// A list of counters, without the ability
/// to add or remove any.
#[component]
fn StaticList(
    cx: Scope,
    /// How many counters to include in this list.
    length: usize,
) -> impl IntoView {
    // create counter signals that start at incrementing numbers
    let counters = (1..=length).map(|idx| create_signal(cx, idx));

    // when you have a list that doesn't change, you can
    // manipulate it using ordinary Rust iterators
    // and collect it into a Vec<_> to insert it into the DOM
    let counter_buttons = counters
        .map(|(count, set_count)| {
            view! { cx,
                <li>
                    <button
                        on:click=move |_| set_count.update(|n| *n += 1)
                    >
                        {count}
                    </button>
                </li>
            }
        })
        .collect::<Vec<_>>();

    // Note that if `counter_buttons` were a reactive list
    // and its value changed, this would be very inefficient:
    // it would rerender every row every time the list changed.
    view! { cx,
        <ul>{counter_buttons}</ul>
    }
}

fn main() {
    leptos::mount_to_body(|cx| view! { cx, <App/> })
}
