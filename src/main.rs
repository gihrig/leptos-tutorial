/* 6.4 Async - Actions */

// Working with async

// So far we’ve only been working with synchronous user interfaces:
// You provide some input, the app immediately processes it and updates
// the interface. This is great, but is a tiny subset of what web
// applications do. In particular, most web apps have to deal with some
// kind of asynchronous data loading, usually loading something from an
// API.

// Asynchronous data is notoriously hard to integrate with the
// synchronous parts of your code. Leptos provides a cross-platform
// `spawn_local` function that makes it easy to run a Future, but
// there’s much more to it than that.
// See: https://docs.rs/leptos/latest/leptos/fn.spawn_local.html

// In this chapter, we’ll see how Leptos helps smooth out that process
// for you.

// ------------------------------------------------------------------

// Mutating Data with Actions

// We’ve talked about how to load async data with resources. Resources
// immediately load data and work closely with <Suspense/> and
// <Transition/> components to show whether data is loading in your app.
// But what if you just want to call some arbitrary async function and
// keep track of what it’s doing?

// Well, you could always use spawn_local.
// https://docs.rs/leptos/latest/leptos/fn.spawn_local.html
// This allows you to just spawn an async task in a synchronous
// environment by handing the Future off to the browser (or, on the
// server, Tokio or whatever other runtime you’re using). But how do
// you know if it’s still pending? Well, you could just set a signal
// to show whether it’s loading, and another one to show the result...

// All of this is true. Or you could use the final async primitive:
// create_action.
// https://docs.rs/leptos/latest/leptos/fn.create_action.html

// Actions and resources seem similar, but they represent fundamentally
// different things. If you’re trying to load data by running an async
// function, either once or when some other value changes, you probably
// want to use `create_resource`. If you’re trying to occasionally run
// an async function in response to something like a user clicking a
// button, you probably want to use `create_action`.

// Say we have some async function we want to run.

/*
  async fn add_todo_request(new_title: &str) -> Uuid {
      /* do some stuff on the server to add a new todo */
  }
*/

// create_action takes a reactive Scope and an async function that
// takes a reference to a single argument, which you could think of
// as its “input type.”

// The input is always a single type. If you want to pass in multiple
// arguments, you can do it with a struct or tuple.

/*
  // if there's a single argument, just use that
  let action1 = create_action(cx, |input: &String| {
    let input = input.clone();
    async move { todo!() }
  });

  // if there are no arguments, use the unit type `()`
  let action2 = create_action(cx, |input: &()| async { todo!() });

  // if there are multiple arguments, use a tuple
  let action3 = create_action(cx,
    |input: &(usize, String)| async { todo!() }
  );
*/

// Because the action function takes a reference but the Future needs
// to have a 'static lifetime, you’ll usually need to clone the value
// to pass it into the Future. This is admittedly awkward but it
// unlocks some powerful features like optimistic UI. We’ll see a
// little more about that in future chapters.

// So in this case, all we need to do to create an action is

/*
  let add_todo_action = create_action(cx, |input: &String| {
    let input = input.to_owned();
    async move { add_todo_request(&input).await }
  });
*/

// Rather than calling add_todo_action directly, we’ll call it with
// .dispatch(), as in

/*
  add_todo_action.dispatch("Some value".to_string());
*/

// You can do this from an event listener, a timeout, or anywhere;
// because .dispatch() isn’t an async function, it can be called from
// a synchronous context.

// Actions provide access to a few signals that synchronize between
// the asynchronous action you’re calling and the synchronous reactive
// system:

/*
  let submitted = add_todo_action.input(); // RwSignal<Option<String>>
  let pending = add_todo_action.pending(); // ReadSignal<bool>
  let todo_id = add_todo_action.value(); // RwSignal<Option<Uuid>>
*/

// This makes it easy to track the current state of your request, show
// a loading indicator, or do “optimistic UI” based on the assumption
// that the submission will succeed.

/*
  let input_ref = create_node_ref::<Input>(cx);

  view! { cx,
      <form
          on:submit=move |ev| {
              ev.prevent_default(); // don't reload the page...
              let input = input_ref.get().expect("input to exist");
              add_todo_action.dispatch(input.value());
          }
      >
          <label>
              "What do you need to do?"
              <input type="text"
                  node_ref=input_ref
              />
          </label>
          <button type="submit">"Add Todo"</button>
      </form>
      // use our loading state
      <p>{move || pending().then("Loading...")}</p>
  }
*/

// Now, there’s a chance this all seems a little over-complicated, or
// maybe too restricted. I wanted to include actions here, alongside
// resources, as the missing piece of the puzzle. In a real Leptos app,
// you’ll actually most often use actions alongside server functions,
// create_server_action,
// https://docs.rs/leptos/latest/leptos/fn.create_server_action.html
// and the <ActionForm/> component
// https://docs.rs/leptos_router/latest/leptos_router/fn.ActionForm.html
// to create really powerful progressively-enhanced forms.
// So if this primitive seems useless to you... Don’t worry! Maybe it
// will make sense later.
// (Or check out our todo_app_sqlite example now.)
// https://github.com/leptos-rs/leptos/blob/main/examples/todo_app_sqlite/src/todo.rs

// --------------------------------------------------------------------
// 6.4 Async - Actions - Final Example
// --------------------------------------------------------------------

use gloo_timers::future::TimeoutFuture;
use leptos::{html::Input, *};
use uuid::Uuid;

// Here we define an async function
// This could be anything: a network request, database read, etc.
// Think of it as a mutation: some imperative async action you run,
// whereas a resource would be some async data you load
async fn add_todo(text: &str) -> Uuid {
    _ = text;
    // fake a one-second delay
    TimeoutFuture::new(1_000).await;
    // pretend this is a post ID or something
    Uuid::new_v4()
}

#[component]
fn App(cx: Scope) -> impl IntoView {
    // an action takes an async function with a single argument
    // it can be a simple type, a struct, or ()
    let add_todo = create_action(cx, |input: &String| {
        // the input is a reference, but we need the Future to own it
        // this is important: we need to clone and move into the Future
        // so it has a 'static lifetime
        let input = input.to_owned();
        async move { add_todo(&input).await }
    });

    // actions provide a bunch of synchronous, reactive variables
    // that tell us different things about the state of the action
    let submitted = add_todo.input();
    let pending = add_todo.pending();
    let todo_id = add_todo.value();

    let input_ref = create_node_ref::<Input>(cx);

    view! { cx,
        <form on:submit=move |ev| {
            ev.prevent_default();
            let input = input_ref.get().expect("input to exist");
            add_todo.dispatch(input.value());
        }>

            <label>
                "What do you need to do?"
                <input type="text" node_ref=input_ref/>
            </label>
            <button type="submit">"Add Todo"</button>
        </form>
        <p style="height: 10px;">{move || pending().then(|| "Loading...")}</p>
        <p>
            "Submitted: " <code>{move || format!("{:#?}", submitted())}</code>
        </p>
        <p>"Pending: " <code>{move || format!("{:#?}", pending())}</code></p>
        <p>"Todo ID: " <code>{move || format!("{:#?}", todo_id())}</code></p>
    }
}

fn main() {
    leptos::mount_to_body(|cx| view! { cx, <App/> })
}
