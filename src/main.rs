/* 4.2e Reactivity - Effects and Rendering */
use leptos::*;

// We’ve managed to get this far without mentioning effects because
// they’re built into the Leptos DOM renderer. We’ve seen that you can
// create a signal and pass it into the view macro, and it will update
// the relevant DOM node whenever the signal changes:

/*
  let (count, set_count) = create_signal(cx, 0);

  view! { cx,
        <p>{count}</p>
    }
*/

// This works because the framework essentially creates an effect
// wrapping this update. You can imagine Leptos translating this view
// into something like this:

/*
  let (count, set_count) = create_signal(cx, 0);

  // create a DOM element
  let p = create_element("p");

  // create an effect to reactively update the text
  create_effect(cx, move |prev_value| {
    // first, access the signal’s value and convert it to a string
    let text = count().to_string();

    // if this is different from the previous value, update the node
    if prev_value != Some(text) {
      p.set_text_content(&text);
    }

    // return this value so we can memoize the next update
    text
  });
*/

// Every time count is updated, this effect wil rerun. This is what
// allows reactive, fine-grained updates to the DOM.

fn main() {
    leptos::mount_to_body(|cx| {
        view! { cx,
        <h1>{"Reactivity - Effects and Rendering"}</h1>}
    })
}
