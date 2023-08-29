/* 12.3.0 Server Side Rendering - Async Rendering and SSR “Modes” */

// Server-rendering a page that uses only synchronous data is pretty
// simple: You just walk down the component tree, rendering each element
// to an HTML string. But this is a pretty big caveat: it doesn’t answer
// the question of what we should do with pages that includes
// asynchronous data, i.e., the sort of stuff that would be rendered
// under a <Suspense/> node on the client.

// When a page loads async data that it needs to render, what should
// we do? Should we wait for all the async data to load, and then render
// everything at once? (Let’s call this “async” rendering) Should we go
// all the way in the opposite direction, just sending the HTML we have
// immediately down to the client and letting the client load the
// resources and fill them in? (Let’s call this “synchronous” rendering)
// Or is there some middle-ground solution that somehow beats them both?
// (Hint: There is.)

// If you’ve ever listened to streaming music or watched a video online,
// I’m sure you realize that HTTP supports streaming, allowing a single
// connection to send chunks of data one after another without waiting
// for the full content to load. You may not realize that browsers are
// also really good at rendering partial HTML pages. Taken together,
// this means that you can actually enhance your users’ experience by
// streaming HTML: and this is something that Leptos supports out of the
// box, with no configuration at all. And there’s actually more than one
// way to stream HTML: you can stream the chunks of HTML that make up
// your page in order, like frames of a video, or you can stream them...
// well, out of order.

// Let me say a little more about what I mean.

// Leptos supports all four different modes of rendering HTML that
// includes asynchronous data:

// 1. Synchronous Rendering
//    https://leptos-rs.github.io/leptos/ssr/23_ssr_modes.html#synchronous-rendering
// 2. Async Rendering
//    https://leptos-rs.github.io/leptos/ssr/23_ssr_modes.html#async-rendering
// 3. In-Order streaming
//    https://leptos-rs.github.io/leptos/ssr/23_ssr_modes.html#in-order-streaming
// 4. Out-of-Order Streaming
//    https://leptos-rs.github.io/leptos/ssr/23_ssr_modes.html#out-of-order-streaming

use leptos::*;
pub fn main() {
    mount_to_body(|cx| {
        view! { cx, <h1>"Server Side Rendering - Async Rendering and SSR 'Modes'"</h1> }
    });
}
