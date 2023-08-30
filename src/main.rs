/* 12.4.0 Server Side Rendering - Hydration Bugs A Thought Experiment */

// -------------------------------------
// Hydration Bugs - A Thought Experiment
// -------------------------------------

// Let’s try an experiment to test your intuitions. Open up an app
// you’re server-rendering with cargo-leptos. (If you’ve just been
// using trunk so far to play with examples, go clone a cargo-leptos
// template just for the sake of this exercise.)

// Put a log somewhere in your root component. (I usually call mine
// <App/>, but anything will do.)

/*
  #[component]
  pub fn App(cx: Scope) -> impl IntoView {
    leptos::log!("where do I run?");
    // ... whatever
  }
*/

// And let’s fire it up

/*
  cargo leptos watch
*/

// Where do you expect "where do I run?"" to log?

//  In the command line where you’re running the server?
//  In the browser console when you load the page?
//  Neither?
//  Both?

// `trunk serve --open` logs in the browser console
//    Requires simple trunk compatible app as shown below
// `cargo leptos watch` logs in the command line and the Browser
//    Requires more complex trunk compatible app as shown below
// Try it out.

// Trunk compatible app
// --------------------
// #[component]
// pub fn App(cx: Scope) -> impl IntoView {
//     leptos::log!("where do I run?");
// }

// use leptos::*;
// pub fn main() {
//     mount_to_body(|cx| {
//         view! { cx,
//           <h1>"Server Side Rendering - Hydration Bugs"</h1>
//           <h2>"A Thought Experiment"</h2>
//           <App/>
//         }
//     });
// }

// cargo leptos compatible app - based on leptos-start-axum
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

// Discussion continued
// ----------------------------------------------------------------

// Okay, consider the spoiler alerted.

// You’ll notice of course that it logs in both places, assuming
// everything goes according to plan. In fact on the server it logs
// twice—first during the initial server startup, when Leptos renders
// your app once to extract the route tree, then a second time when
// you make a request. Each time you reload the page, "where do I run?"
// should log once on the server and once on the client.

// If you think about the description in the last couple sections,
// hopefully this makes sense. Your application runs once on the server,
// where it builds up a tree of HTML which is sent to the client. During
// this initial render, "where do I run?" logs on the server.

// Once the WASM binary has loaded in the browser, your application
// runs a second time, walking over the same user interface tree and
// adding interactivity.

// Does that sound like a waste? It is, in a sense. But reducing that
// waste is a genuinely hard problem. It’s what some JS frameworks like
// Qwik are intended to solve, although it’s probably too early to tell
// whether it’s a net performance gain as opposed to other approaches.
