/* 12.4.3 Server Side Rendering - Hydration Bugs Client Code on Server */

// ----------------------------------------------
// Hydration Bugs Client Code Can't Run on Server
// ----------------------------------------------

// The Potential for Bugs

// Hopefully the previous section thought experiment (12.4.0) made sense.
// But what does it have to do with the title of this chapter, which is
// “Hydration bugs (and how to avoid them)”?

// Remember that the application needs to run on both the server and the
// client. This generates a few different sets of potential issues you
// need to know how to avoid.

// --------------------------------------------------------

// Some Client Code Can't Run on Server

// Imagine you happily import a dependency like gloo-net that you’ve
// been used to using to make requests in the browser, and use it in a
// create_resource in a server-rendered app.

// You’ll probably instantly see the dreaded message

/*
  panicked at 'cannot call wasm-bindgen imported functions on non-wasm
  targets'
*/

// Uh-oh.

// But of course this makes sense. We’ve just said that your app needs
// to run on the client and the server.

// Solution

// There are a few ways to avoid this:

// 1. Only use libraries that can run on both the server and the client.
//    `reqwest`, for example, works for making HTTP requests in both
//    settings.
// 2. Use different libraries on the server and the client, and gate them
//    using the #[cfg] macro. Example here:
//    https://github.com/leptos-rs/leptos/blob/main/examples/hackernews/src/api.rs
// 3. Wrap client-only code in create_effect. Because create_effect only
//    runs on the client, this can be an effective way to access browser
//    APIs that are not needed for initial rendering.

// For example, say that I want to store something in the browser’s
// localStorage whenever a signal changes.

/*
  #[component]
  pub fn App(cx: Scope) -> impl IntoView {
    use gloo_storage::Storage;
    let storage = gloo_storage::LocalStorage::raw();
    leptos::log!("{storage:?}");
  }
*/

// This panics because I can’t access LocalStorage during server
// rendering.

// But if I wrap it in create_effect...

/*
  #[component]
  pub fn App(cx: Scope) -> impl IntoView {
    use gloo_storage::Storage;
    create_effect(cx, move |_| {
      let storage = gloo_storage::LocalStorage::raw();
      leptos::log!("{storage:?}");
    });
  }
*/

// It’s fine! This will render appropriately on the server, ignoring
// the client-only code, and then access the storage and log a message
// on the browser.

// See src/app.rs for demonstration of this error
// ----------------------------------------------

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::{routing::post, Router};
    use leptos::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use leptos_tutorial::{app::*, fileserve::file_and_error_handler};

    leptos::log!("Running with feature = 'ssr'");
    simple_logger::init_with_level(log::Level::Info)
        .expect("couldn't initialize logging");

    // Setting get_configuration(None) means we'll be using cargo-leptos's env values
    // For deployment these variables are:
    // <https://github.com/leptos-rs/start-axum#executing-a-server-on-a-remote-machine-without-the-toolchain>
    // Alternately a file can be specified such as Some("Cargo.toml")
    // The file would need to be included with the executable when moved to deployment
    let conf = get_configuration(None).await.unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(|cx| view! { cx, <App/> }).await;

    // build our application with a route
    let app = Router::new()
        .route("/api/*fn_name", post(leptos_axum::handle_server_fns))
        .leptos_routes(&leptos_options, routes, |cx| view! { cx, <App/> })
        .fallback(file_and_error_handler)
        .with_state(leptos_options);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    log!("listening on http://{}", &addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // This log never runs, even with feature = 'csr'
    leptos::log!("Running with feature = 'csr'");
    // no client-side main function
    // unless we want this to work with e.g., Trunk for a purely
    // client-side app
    // see lib.rs for hydration function instead
}
