/* 12.4.1 Server Side Rendering - Hydration Bugs Client/Serve Mismatch */

// ----------------------------------------------
// Hydration Bugs - Client Server Code Mismatches
// ----------------------------------------------

// The Potential for Bugs

// Hopefully the previous section thought experiment (12.4.0) made sense.
// But what does it have to do with the title of this chapter, which is
// “Hydration bugs (and how to avoid them)”?

// Remember that the application needs to run on both the server and the
// client. This generates a few different sets of potential issues you
// need to know how to avoid.

// --------------------------------------------------------

// Mismatches between server and client code

// One way to create a bug is by creating a mismatch between the HTML
// that’s sent down by the server and what’s rendered on the client.
// It’s actually fairly hard to do this unintentionally, I think (at
// least judging by the bug reports I get from people.) But imagine I
// do something like this

/*
  #[component]
  pub fn App(cx: Scope) -> impl IntoView {
    let data = if cfg!(target_arch = "wasm32") {
      vec![0, 1, 2]
    } else {
      vec![]
    };
    data.into_iter()
    .map(|value| view! { cx, <span>{value}</span> })
    .collect_view(cx)
  }
*/

// In other words, if this is being compiled to WASM, it has three items;
// otherwise it’s empty.

// When I load the page in the browser, I see nothing. If I open the
// console I see a bunch of warnings:

/*
  element with id 0-3 not found, ignoring it for hydration
  element with id 0-4 not found, ignoring it for hydration
  element with id 0-5 not found, ignoring it for hydration
  component with id _0-6c not found, ignoring it for hydration
  component with id _0-6o not found, ignoring it for hydration
*/

// The WASM version of your app, running in the browser, expects to find
// three items; but the HTML has none.

// Solution

// It’s pretty rare that you do this intentionally, but it could happen
// from somehow running different logic on the server and in the browser.
// If you’re seeing warnings like this and you don’t think it’s your
// fault, it’s much more likely that it’s a bug with <Suspense/> or
// something. Feel free to go ahead and open an issue or discussion on
// GitHub for help.

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
