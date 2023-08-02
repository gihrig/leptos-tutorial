/* 4.2b Reactivity - Autotracking and Dynamic Dependencies */
use leptos::*;

// If you’re familiar with a framework like React, you might notice one
// key difference. React and similar frameworks typically require you to
// pass a “dependency array,” an explicit set of variables that determine
// when the effect should rerun.

// Because Leptos comes from the tradition of synchronous reactive
// programming, we don’t need this explicit dependency list. Instead, we
// automatically track dependencies depending on which signals are accessed
// within the effect.

// This has two effects (no pun intended). Dependencies are:

// 1. Automatic: You don’t need to maintain a dependency list, or worry about
//    what should or shouldn’t be included. The framework simply tracks
//    which signals might cause the effect to rerun, and handles it for you.
// 2. Dynamic: The dependency list is cleared and updated every time the
//    effect runs. If your effect contains a conditional (for example),
//    only signals that are used in the current branch are tracked. This
//    means that effects rerun the absolute minimum number of times.

// If this sounds like magic, and if you want a deep dive into how
// automatic dependency tracking works, check out this video.
// (Apologies for the low volume!)

fn main() {
    leptos::mount_to_body(|cx| {
        view! { cx,
        <h1>{"Reactivity - Autotracking and Dynamic Dependencies"}</h1>}
    })
}
