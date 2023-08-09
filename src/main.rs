/* 8.0.a Global State Management - URL as Global State */

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
// 8.0.a Global State Management - URL as Global State
// --------------------------------------------------------------------

// In many ways, the URL is actually the best way to store global state.
// It can be accessed from any component, anywhere in your tree.
// There are native HTML elements like <form> and <a> that exist solely
// to update the URL.
// And it persists across page reloads and between devices;
// you can share a URL with a friend or send it from your phone to your
// laptop and any state stored in it will be replicated.

// The next few sections of the tutorial will be about the router, and
// we’ll get much more into these topics.

// But for now, we'll just look at options #2 and #3.

use leptos::*;

fn main() {
    leptos::mount_to_body(|cx| {
        view! { cx, <h1>"URL as Global State"</h1> }
    })
}
