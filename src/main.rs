/* 13.0 Working with the Server */

// The previous section described the process of server-side rendering,
// using the server to generate an HTML version of the page that will
// become interactive in the browser. So far, everything has been
// “isomorphic” or “universal”; in other words, your app has had the
// “same (iso) shape (morphe)” on the client and the server.

// But a server can do a lot more than just render HTML! In fact, a
// server can do a whole bunch of things your browser can’t, like
// reading from and writing to a SQL database.

// If you’re used to building JavaScript frontend apps, you’re probably
// used to calling out to some kind of REST API to do this sort of server
// work. If you’re used to building sites with PHP or Python or Ruby (or
// Java or C# or...), this server-side work is your bread and butter,
// and it’s the client-side interactivity that tends to be an
// afterthought.

// With Leptos, you can do both: not only in the same language, not only
// sharing the same types, but even in the same files!

// This section will talk about how to build the uniquely-server-side
// parts of your application.

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
