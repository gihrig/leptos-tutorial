/* 13.1.0 Working with the Server - Server Functions */

// If you’re creating anything beyond a toy app, you’ll need to run
// code on the server all the time: reading from or writing to a
// database that only runs on the server, running expensive
// computations using libraries you don’t want to ship down to the
// client, accessing APIs that need to be called from the server
// rather than the client for CORS reasons or because you need a
// secret API key that’s stored on the server and definitely
// shouldn’t be shipped down to a user’s browser.

// Traditionally, this is done by separating your server and client
// code, and by setting up something like a REST API or GraphQL API
// to allow your client to fetch and mutate data on the server. This
// is fine, but it requires you to write and maintain your code in
// multiple separate places (client-side code for fetching, server-side
// functions to run), as well as creating a third thing to manage,
// which is the API contract between the two.

// Leptos is one of a number of modern frameworks that introduce the
// concept of server functions. Server functions have two key
// characteristics:

// 1. Server functions are co-located with your component code, so that
//    you can organize your work by feature, not by technology. For
//    example, you might have a “dark mode” feature that should persist
//    a user’s dark/light mode preference across sessions, and be
//    applied during server rendering so there’s no flicker. This
//    requires a component that needs to be interactive on the client,
//    and some work to be done on the server (setting a cookie, maybe
//    even storing a user in a database.) Traditionally, this feature
//    might end up being split between two different locations in your
//    code, one in your “frontend” and one in your “backend.” With
//    server functions, you’ll probably just write them both in one
//    dark_mode.rs and forget about it.
// 2. Server functions are isomorphic, i.e., they can be called either
//    from the server or the browser. This is done by generating code
//    differently for the two platforms. On the server, a server
//    function simply runs. In the browser, the server function’s body
//    is replaced with a stub that actually makes a fetch request to
//    the server, serializing the arguments into the request and
//    deserializing the return value from the response. But on either
//    end, the function can simply be called: you can create an add_todo
//    function that writes to your database, and simply call it from a
//    click handler on a button in the browser!

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
