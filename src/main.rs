/* 4.2d Reactivity - To create_effect, or not to create_effect? */
use leptos::*;

// Effects are intended to run side-effects of the system, not to
// synchronize state within the system. In other words:

// Don’t write to signals within effects.

// If you need to define a signal that depends on the value of other
// signals, use a derived signal or create_memo.

// If you need to synchronize some reactive value with the non-reactive
// world outside—like a web API, the console, the filesystem, or the
// DOM—create an effect.

// If you’re curious for more information about when you should and
// shouldn’t use create_effect, check out this video for a more in-depth
// consideration! https://youtu.be/aQOFJQ2JkvQ

fn main() {
    leptos::mount_to_body(|cx| {
        view! { cx,
        <h1>{"Reactivity - To create_effect, or not to create_effect?"}</h1>}
    })
}
