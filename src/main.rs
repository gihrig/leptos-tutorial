/* 6.1 Async - Loading Data with Resources */

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

// Loading Data with Resources

// A Resource is a reactive data structure that reflects the current
// state of an asynchronous task, allowing you to integrate asynchronous
// Futures into the synchronous reactive system. Rather than waiting
// for its data to load with .await, you transform the Future into a
// signal that returns Some(T) if it has resolved, and None if it’s
// still pending.
// See: https://docs.rs/leptos/latest/leptos/struct.Resource.html

// You do this by using the `create_resource` function. This takes two
// arguments (other than the ubiquitous cx):

// 1. a source signal, which will generate a new Future whenever it changes
// 2. a fetcher function, which takes the data from that signal and
//    returns a Future

// Here’s an example
/*
  // our source signal: some synchronous, local state
  let (count, set_count) = create_signal(cx, 0);

  // our resource
  let async_data = create_resource(cx,
    count,
    // every time `count` changes, this will run
    |value| async move {
      log!("loading data from API");
      load_data(value).await
    },
  );
*/

// To create a resource that simply runs once, you can pass a
// non-reactive, empty source signal:
/*
  let once = create_resource(cx, || (), |_| async move {
    load_data().await
  });
*/

// To access the value you can use `.read(cx)` or
// `.with(cx, |data| /* */)`.
// These work just like `.get()` and `.with()` on a signal. `read` clones
// the value and returns it, `with` applies a closure to it.
// But there are two differences:

// 1. For any `Resource<_, T>`, they always return `Option<T>`, not `T:`
//    because it’s always possible that your resource is still loading.
// 2. They take a Scope argument. You’ll see why in the next chapter, on
//    `<Suspense/>`.

// So, you can show the current state of a resource in your view:
/*
  let once = create_resource(cx, || (), |_| async move {
    load_data().await
  });
  view! { cx,
      <h1>"My Data"</h1>
      {move || match once.read(cx) {
          None => view! { cx, <p>"Loading..."</p> }.into_view(cx),
          Some(data) => view! { cx, <ShowData data/> }.into_view(cx)
      }}
  }
*/

// Resources also provide a `refetch()` method that allows you to
// manually reload the data (for example, in response to a button
// click) and a `loading()` method that returns a `ReadSignal<bool>`
// indicating whether the resource is currently loading or not.

// -----------------------------------------------------------------
// 6.1 Async - Loading Data with Resources - Example
// -----------------------------------------------------------------
use gloo_timers::future::TimeoutFuture;
use leptos::*;

// Here we define an async function
// This could be anything: a network request, database read, etc.
// Here, we just multiply a number by 10
async fn load_data(value: i32) -> i32 {
    // fake a one-second delay
    TimeoutFuture::new(1_000).await;
    value * 10
}

#[component]
fn App(cx: Scope) -> impl IntoView {
    // this count is our synchronous, local state
    let (count, set_count) = create_signal(cx, 0);

    // create_resource takes two arguments after its scope
    let async_data = create_resource(
        cx,
        // the first is the "source signal"
        count,
        // the second is the loader
        // it takes the source signal's value as its argument
        // and does some async work
        |value| async move { load_data(value).await },
    );
    // whenever the source signal changes, the loader reloads

    // you can also create resources that only load once
    // just return the unit type () from the source signal
    // that doesn't depend on anything: we just load it once
    let stable =
        create_resource(cx, || (), |_| async move { load_data(1).await });

    // we can access the resource values with .read()
    // this will reactively return None before the Future has resolved
    // and update to Some(T) when it has resolved
    let async_result = move || {
        async_data
            .read(cx)
            .map(|value| format!("Server returned {value:?}"))
            // This loading state will only show before the first load
            .unwrap_or_else(|| "Loading...".into())
    };

    // the resource's loading() method gives us a
    // signal to indicate whether it's currently loading
    let loading = async_data.loading();
    let is_loading = move || if loading() { "Loading..." } else { "Idle." };

    view! { cx,
        <button on:click=move |_| {
            set_count.update(|n| *n += 1);
        }>

            "Click me"
        </button>
        <p>
            <code>"stable"</code>
            ": "
            {move || stable.read(cx)}
        </p>
        <p>
            <code>"count"</code>
            ": "
            {count}
        </p>
        <p>
            <code>"async_value"</code>
            ": "
            {async_result}
            <br/>
            {is_loading}
        </p>
    }
}

fn main() {
    leptos::mount_to_body(|cx| view! { cx, <App/> })
}
