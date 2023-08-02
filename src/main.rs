/* 4.2c Reactivity - Effects as Zero-Cost-ish Abstraction */
use leptos::*;

// While they’re not a “zero-cost abstraction” in the most technical
// sense—they require some additional memory use, exist at runtime,
// etc.—at a higher level, from the perspective of whatever expensive
// API calls or other work you’re doing within them, effects are a
// zero-cost abstraction. They rerun the absolute minimum number of
// times necessary, given how you’ve described them.

// Imagine that I’m creating some kind of chat software, and I want
// people to be able to display their full name, or just their first
// name, and to notify the server whenever their name changes:

/*
  let (first, set_first) = create_signal(cx, String::new());
  let (last, set_last) = create_signal(cx, String::new());
  let (use_last, set_use_last) = create_signal(cx, true);

  // this will add the name to the log
  // any time one of the source signals changes
  create_effect(cx, move |_| {
      log(
          cx,
          if use_last() {
              format!("{} {}", first(), last())
          } else {
              first()
          },
      )
  });
*/

// If use_last is true, effect should rerun whenever first, last, or
// use_last changes. But if I toggle use_last to false, a change in last
// will never cause the full name to change. In fact, last will be
// removed from the dependency list until use_last toggles again. This
// saves us from sending multiple unnecessary requests to the API if I
// change last multiple times while use_last is still false.

fn main() {
    leptos::mount_to_body(|cx| {
        view! { cx,
        <h1>{"Reactivity - Effects as Zero-Cost-ish Abstraction"}</h1>}
    })
}
