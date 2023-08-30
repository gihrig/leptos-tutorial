/* 12.4.2 Server Side Rendering - Hydration Bugs DOM Mutation */

// ----------------------------------------------
// Hydration Bugs - DOM Mutation During Rendering
// ----------------------------------------------

// The Potential for Bugs

// Hopefully the previous section thought experiment (12.4.0) made sense.
// But what does it have to do with the title of this chapter, which is
// “Hydration bugs (and how to avoid them)”?

// Remember that the application needs to run on both the server and the
// client. This generates a few different sets of potential issues you
// need to know how to avoid.

// --------------------------------------------------------

// DOM Mutation During Rendering

// This is a slightly more common way to create a client/server mismatch:
// updating a signal during rendering in a way that mutates the view.

/*
  #[component]
  pub fn App(cx: Scope) -> impl IntoView {
    let (loaded, set_loaded) = create_signal(cx, false);

    // create_effect only runs on the client
    create_effect(cx, move |_| {
      // do something like reading from localStorage
      set_loaded(true);
    });

      move || {
        if loaded() {
          view! { cx, <p>"Hello, world!"</p> }.into_any()
        } else {
          view! { cx, <div class="loading">"Loading..."</div> }.into_any()
        }
      }
    }
*/

// This one gives us the scary panic <--- not true! see app.rs
/*
  panicked at 'assertion failed: `(left == right)`
    left: `"DIV"`,
  right: `"P"`: SSR and CSR elements have the same hydration key but different node kinds.
*/

// The problem here is that create_effect runs immediately and
// synchronously, but only in the browser. As a result, on the server,
// loaded is false, and a <div> is rendered. But on the browser, by the
// time the view is being rendered, loaded has already been set to true,
// and the browser is expecting to find a <p>.

// Solution

// You can simply tell the effect to wait a tick before updating the
// signal, by using something like request_animation_frame, which will
// set a short timeout and then update the signal before the next frame.

/*
  create_effect(cx, move |_| {
    // do something like reading from localStorage
    request_animation_frame(move || set_loaded(true));
  });
*/

// This allows the browser to hydrate with the correct, matching state
// (loaded is false when it reaches the view), then immediately update
// it to true once hydration is complete.

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
