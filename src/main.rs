/* 12.4.4 Server Side Rendering - Hydration Bugs Server Code on Client */

// ----------------------------------------------
// Hydration Bugs Server Code Can't Run on Client
// ----------------------------------------------

// The Potential for Bugs

// Hopefully the previous section thought experiment (12.4.0) made sense.
// But what does it have to do with the title of this chapter, which is
// “Hydration bugs (and how to avoid them)”?

// Remember that the application needs to run on both the server and the
// client. This generates a few different sets of potential issues you
// need to know how to avoid.

// --------------------------------------------------------

// Some Server Code Can't Run on Client

// WebAssembly running in the browser is a pretty limited environment.
// You don’t have access to a file-system or to many of the other things
// the standard library may be used to having. Not every crate can even
// be compiled to WASM, let alone run in a WASM environment.

// In particular, you’ll sometimes see errors about the crate mio or
// missing things from core. This is generally a sign that you are
// trying to compile something to WASM that can’t be compiled to WASM.
// If you’re adding server-only dependencies, you’ll want to mark them
// optional = true in your Cargo.toml and then enable them in the ssr
// feature definition e.g. `axum = { version = "0.6.4", optional = true }`

// You can use create_effect to specify that something should only run
// on the client, and not in the server. Is there a way to specify that
// something should run only on the server, and not the client?

// In fact, there is. The next chapter will cover the topic of server
// functions in some detail. Leptos Server Functions are documented here:
// https://docs.rs/leptos_server/latest/leptos_server/index.html

// Example app not used in this chapter.
// --------------------------------------------------------

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
