/* 12.3.2 Server Side Rendering - Async Rendering */

// For example page see https://leptos-rs.github.io/leptos/ssr/23_ssr_modes.html#async-rendering

// ---------------
// Async Rendering
// ---------------

// Async: load all resources on the server. Wait until all data are loaded, and
// render HTML in one sweep.

// Pros:
//    Better handling for meta tags (because you know async data even
//    before you render the <head>). Faster complete load than
//    synchronous because async resources begin loading on server.
// Cons:
//    Slower load time/TTFB: you need to wait for all async resources to
//    load before displaying anything on the client. The page is totally
//    blank until everything is loaded.

use leptos::*;
pub fn main() {
    mount_to_body(|cx| {
        view! { cx, <h1>"Server Side Rendering - Async Rendering"</h1> }
    });
}
