/* 12.3.1 Server Side Rendering - Synchronous Rendering */

// ---------------------
// Synchronous Rendering
// ---------------------

//
// Synchronous: Serve an HTML shell that includes fallback for any
// <Suspense/>. Load data on the client using create_local_resource,
// replacing fallback once resources are loaded.

// Pros:
//  App shell appears very quickly: great TTFB (time to first byte).
// Cons:
//  Resources load relatively slowly; you need to wait for JS + WASM to
//    load before even making a request.
//  No ability to include data from async resources in the <title> or
//    other <meta> tags, hurting SEO and things like social media link
//    previews.

// If you’re using server-side rendering, the synchronous mode is almost
// never what you actually want, from a performance perspective. This is
// because it misses out on an important optimization. If you’re loading
// async resources during server rendering, you can actually begin
// loading the data on the server. Rather than waiting for the client to
// receive the HTML response, then loading its JS + WASM, then realize it
// needs the resources and begin loading them, server rendering can
// actually begin loading the resources when the client first makes the
// response. In this sense, during server rendering an async resource is
// like a Future that begins loading on the server and resolves on the
// client. As long as the resources are actually serializable, this will
// always lead to a faster total load time.

// This is why create_resource
// https://docs.rs/leptos/latest/leptos/fn.create_resource.html
// requires resources data to be serializable by default, and why you
// need to explicitly use create_local_resource
// https://docs.rs/leptos/latest/leptos/fn.create_local_resource.html
// for any async data that is not serializable and should therefore only
// be loaded in the browser itself. Creating a local resource when you
// could create a serializable resource is always a deoptimization.

use leptos::*;
pub fn main() {
    mount_to_body(|cx| {
        view! { cx, <h1>"Server Side Rendering - Synchronous Rendering"</h1> }
    });
}
