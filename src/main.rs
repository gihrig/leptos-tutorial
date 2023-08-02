/* 4.2a Reactivity - Responding to Changes with create_effect */
use leptos::*;

// We’ve made it this far without having mentioned half of the reactive
// system: effects.

// Reactivity works in two halves: updating individual reactive values
// (“signals”) notifies the pieces of code that depend on them (“effects”)
// that they need to run again. These two halves of the reactive system
// are inter-dependent. Without effects, signals can change within the
// reactive system but never be observed in a way that interacts with the
// outside world. Without signals, effects run once but never again, as
// there’s no observable value to subscribe to. Effects are quite literally
// “side effects” of the reactive system: they exist to synchronize the
// reactive system with the non-reactive world outside it.

// Hidden behind the whole reactive DOM renderer that we’ve seen so far is
// a function called create_effect.

// create_effect takes a function as its argument. It immediately runs the
// function. If you access any reactive signal inside that function, it
// registers the fact that the effect depends on that signal with the
// reactive runtime. Whenever one of the signals that the effect depends on
// changes, the effect runs again.

/*
  let (a, set_a) = create_signal(cx, 0);
  let (b, set_b) = create_signal(cx, 0);


  create_effect(cx, move |_| {
    // immediately prints "Value: 0" and subscribes to `a`
    log::debug!("Value: {}", a());
  });
*/

// The effect function is called with an argument containing whatever
// value it returned the last time it ran. On the initial run, this is
// None.

// By default, effects do not run on the server. This means you can call
// browser-specific APIs within the effect function without causing issues.
// If you need an effect to run on the server, use create_isomorphic_effect.

fn main() {
    leptos::mount_to_body(|cx| {
        view! { cx,
        <h1>{"Reactivity - Responding to Changes with create_effect"}</h1>}
    })
}
