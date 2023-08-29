/* 12.3.5 Server Side Rendering - Partially-blocked Streaming */

// ---------------------------
// Partially-blocked Streaming
// ---------------------------

// Partially-blocked streaming: “Partially-blocked” streaming is useful
// when you have multiple separate <Suspense/> components on the page.
// If one of them reads from one or more “blocking resources” (see below),
// the fallback will not be sent; rather, the server will wait until that
// <Suspense/> has resolved and then replace the fallback with the
// resolved fragment on the server, which means that it is included in
// the initial HTML response and appears even if JavaScript is disabled
// or not supported. Other <Suspense/> stream in out of order as usual.

// This is useful when you have multiple <Suspense/> on the page, and one
// is more important than the other: think of a blog post and comments,
// or product information and reviews. It is not useful if there’s only
// one <Suspense/>, or if every <Suspense/> reads from blocking resources.
// In those cases it is a slower form of async rendering.

// Pros:
//    Works if JavaScript is disabled or not supported on the user’s
//    device.
// Cons:
//    Slower initial response time than out-of-order.
//    Marginally slower overall response due to additional work on the
//      server.
//    No fallback state shown.

use leptos::*;
pub fn main() {
    mount_to_body(|cx| {
        view! { cx, <h1>"Server Side Rendering - Partially-blocked Streaming"</h1> }
    });
}
