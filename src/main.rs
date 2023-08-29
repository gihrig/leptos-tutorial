/* 12.3.3 Server Side Rendering - In-Order Streaming */

// For example page see https://leptos-rs.github.io/leptos/ssr/23_ssr_modes.html#in-order-streaming

// ------------------
// In-Order Streaming
// ------------------

// In-order streaming: Walk through the component tree, rendering HTML
// until you hit a <Suspense/>. Send down all the HTML you’ve got so far
// as a chunk in the stream, wait for all the resources accessed under
// the <Suspense/> to load, then render it to HTML and keep walking until
// you hit another <Suspense/> or the end of the page.

// Pros:
//    Rather than a blank screen, shows at least something before the
//      data are ready.
// Cons
//    Loads the shell more slowly than synchronous rendering (or
//      out-of-order streaming) because it needs to pause at every
//      <Suspense/>.
//    Unable to show fallback states for <Suspense/>.
//    Can’t begin hydration until the entire page has loaded, so earlier
//      pieces of the page will not be interactive until the suspended
//      chunks have loaded.

use leptos::*;
pub fn main() {
    mount_to_body(|cx| {
        view! { cx, <h1>"Server Side Rendering - In-Order Streaming"</h1> }
    });
}
