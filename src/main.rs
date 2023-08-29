/* 12.3.6 Server Side Rendering - Using SSR Modes */

// ---------------
// Using SSR Modes
// ---------------

// Because it offers the best blend of performance characteristics,
// Leptos defaults to out-of-order streaming. But it’s really simple
// to opt into these different modes. You do it by adding an ssr
// property onto one or more of your <Route/> components, like in the
// ssr_modes example.
// https://github.com/leptos-rs/leptos/blob/main/examples/ssr_modes/src/app.rs

/*
  <Routes>
    // We’ll load the home page with out-of-order streaming and <Suspense/>
    <Route path="" view=HomePage/>

    // We'll load the posts with async rendering, so they can set
    // the title and metadata *after* loading the data
    <Route
      path="/post/:id"
      view=Post
      ssr=SsrMode::Async
    />
  </Routes>
*/

// For a path that includes multiple nested routes, the most restrictive
// mode will be used: i.e., if even a single nested route asks for async
// rendering, the whole initial request will be rendered async. async is
// the most restricted requirement, followed by in-order, and then
// out-of-order. (This probably makes sense if you think about it for a
// few minutes.)

use leptos::*;
pub fn main() {
    mount_to_body(|cx| {
        view! { cx, <h1>"Server Side Rendering - Using SSR Modes"</h1> }
    });
}
