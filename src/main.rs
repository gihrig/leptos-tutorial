/* 13.1.1 Working with the Server - Using Server Functions */

// Actually, I kind of like that todo example. What would it look like?
// It’s pretty simple, actually.

// todo.rs
/*
  #[server(AddTodo, "/api")]
  pub async fn add_todo(title: String) -> Result<(), ServerFnError> {
    let mut conn = db().await?;

    match sqlx::query("INSERT INTO todos (title, completed) VALUES ($1, false)")
    .bind(title)
    .execute(&mut conn)
    .await
    {
      Ok(_row) => Ok(()),
      Err(e) => Err(ServerFnError::ServerError(e.to_string())),
      }
  }

  #[component]
  pub fn BusyButton(cx: Scope) -> impl IntoView {
    view! {
      cx,
      <button on:click=move |_| {
        spawn_local(async {
          add_todo("So much to do!".to_string()).await;
        });
      }>
      "Add Todo"
      </button>
      }
  }
*/

// You’ll notice a few things here right away:

// 1. Server functions can use server-only dependencies, like sqlx, and
//    can access server-only resources, like our database.
// 2. Server functions are async. Even if they only did synchronous work
//    on the server, the function signature would still need to be async,
//    because calling them from the browser must be asynchronous.
// 3. Server functions return Result<T, ServerFnError>. Again, even if
//    they only do infallible work on the server, this is true, because
//    ServerFnError’s variants include the various things that can be
//    wrong during the process of making a  network request.
// 4. Server functions can be called from the client. Take a look at our
//    click handler. This is code that will only ever run on the client.
//    But it can call the function add_todo (using spawn_local to run
//    the Future) as if it were an ordinary async function:

/*
        move |_| {
          spawn_local(async {
            add_todo("So much to do!".to_string()).await;
          });
        }
*/

// 5. Server functions are top-level functions defined with fn. Unlike
//    event listeners, derived signals, and most everything else in
//    Leptos, they are not closures! As fn calls, they have no access to
//    the reactive state of your app or anything else that is not passed
//    in as an argument. And again, this makes perfect sense: When you
//    make a request to the server, the server doesn’t have access to
//    client state unless you send it explicitly. (Otherwise we’d have
//    to serialize the whole reactive system and send it across the wire
//    with every request, which—while it served classic ASP for a while—
//    is a really bad idea.)
// 6. Server function arguments and return values both need to be

// There are a few things to note about the way you define a server
// function, too.

// 1. Server functions are created by using the #[server] macro to
//    annotate a top-level function, which can be defined anywhere.
//    https://docs.rs/leptos_server/latest/leptos_server/index.html#server
// 2. We provide the macro a `type name`. The type name is used
//    internally as a container to hold, serialize, and deserialize
//    the arguments.
// 3. We provide the macro a path. This is a prefix for the path at
//    which we’ll mount a server function handler on our server. See
//    examples:
//    Actix: https://github.com/leptos-rs/leptos/blob/main/examples/todo_app_sqlite/src/main.rs#L44
//    Axum:  https://github.com/leptos-rs/leptos/blob/main/examples/todo_app_sqlite_axum/src/main.rs#L55
// 4. You’ll need to have serde as a dependency with the derive
//    feature enabled for the macro to work properly. You can easily
//    add it to Cargo.toml with cargo add serde --features=derive.

// See /src/todo.rs for an example of a server function.
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
