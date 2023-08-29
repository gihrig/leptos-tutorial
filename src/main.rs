/* 12.3.4 Server Side Rendering - Out-of-Order Streaming */

// For example page see https://leptos-rs.github.io/leptos/ssr/23_ssr_modes.html#out-of-order-streaming

// ----------------------
// Out-of-Order Streaming
// ----------------------

// Out-of-order streaming: Like synchronous rendering, serve an HTML
// shell that includes fallback for any <Suspense/>. But load data on
// the server, streaming it down to the client as it resolves, and
// streaming down HTML for <Suspense/> nodes, which is swapped in to
// replace the fallback.

// Pros: Combines the best of synchronous and async.
//    Fast initial response/TTFB because it immediately sends the whole
//      synchronous shell
//    Fast total time because resources begin loading on the server.
//    Able to show the fallback loading state and dynamically replace
//      it, instead of showing blank sections for un-loaded data.

// Cons: Requires JavaScript to be enabled for suspended fragments to
//    appear in correct order. (This small chunk of JS is streamed down
//    in a <script> tag alongside the <template> tag that contains the
//    rendered <Suspense/> fragment, so it does not need to load any
//    additional JS files.)

use leptos::*;
pub fn main() {
    mount_to_body(|cx| {
        view! { cx, <h1>"Server Side Rendering - Out-of-Order Streaming"</h1> }
    });
}
