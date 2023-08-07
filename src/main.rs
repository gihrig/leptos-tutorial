/* 6.2 Async - Suspense */

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

// <Suspense>

// In the previous chapter, we showed how you can create a simple loading
// screen to show some fallback while a resource is loading.

/*
  let (count, set_count) = create_signal(cx, 0);
  let a = create_resource(cx, count, |count| async move { load_a(count).await });

  view! { cx,
    <h1>"My Data"</h1>
    {move || match once.read(cx) {
      None => view! { cx, <p>"Loading..."</p> }.into_view(cx),
      Some(data) => view! { cx, <ShowData data/> }.into_view(cx)
    }}
  }
*/

// But what if we have two resources, and want to wait for both of them?

/*
  let (count, set_count) = create_signal(cx, 0);
  let (count2, set_count2) = create_signal(cx, 0);
  let a = create_resource(cx, count, |count| async move { load_a(count).await });
  let b = create_resource(cx, count2, |count| async move { load_b(count).await });

  view! { cx,
    <h1>"My Data"</h1>
    {move || match (a.read(cx), b.read(cx)) {
      (Some(a), Some(b)) => view! { cx,
        <ShowA a/>
        <ShowA b/>
      }.into_view(cx),
      _ => view! { cx, <p>"Loading..."</p> }.into_view(cx)
    }}
  }
*/

// That’s not so bad, but it’s kind of annoying. What if we could invert
// the flow of control?

// The <Suspense/> component lets us do exactly that. You give it a
// fallback prop and children, one or more of which usually involves
// reading from a resource. Reading from a resource “under” a <Suspense/>
// (i.e., in one of its children) registers that resource with the
// <Suspense/>. If it’s still waiting for resources to load, it shows
// the fallback. When they’ve all loaded, it shows the children.

/*
  let (count, set_count) = create_signal(cx, 0);
  let (count2, set_count2) = create_signal(cx, 0);
  let a = create_resource(cx, count, |count| async move { load_a(count).await });
  let b = create_resource(cx, count2, |count| async move { load_b(count).await });

  view! { cx,
    <h1>"My Data"</h1>
    <Suspense
    fallback=move || view! { cx, <p>"Loading..."</p> }
    >
    <h2>"My Data"</h2>
    <h3>"A"</h3>
    {move || {
      a.read(cx)
      .map(|a| view! { cx, <ShowA a/> })
    }}
    <h3>"B"</h3>
    {move || {
      b.read(cx)
      .map(|b| view! { cx, <ShowB b/> })
    }}
    </Suspense>
  }
*/

// Every time one of the resources is reloading, the "Loading..."
// fallback will show again.

// This inversion of the flow of control makes it easier to add or
// remove individual resources, as you don’t need to handle the matching
// yourself. It also unlocks some massive performance improvements
// during server-side rendering, which we’ll talk about during a later
// chapter.

// ----------------------------------------------------------------
// 6.2 Async - Suspense - Example
// ----------------------------------------------------------------

use gloo_timers::future::TimeoutFuture;
use leptos::*;

async fn ucase_api_call(name: String, delay: u32) -> String {
    TimeoutFuture::new(delay).await;
    name.to_ascii_uppercase()
}

async fn lcase_api_call(name: String, delay: u32) -> String {
    TimeoutFuture::new(delay).await;
    name.to_ascii_lowercase()
}

#[component]
fn App(cx: Scope) -> impl IntoView {
    let (name, set_name) = create_signal(cx, "Bill".to_string());

    // this will reload every time `name` changes
    let async_udata = create_resource(cx, name, |name| async move {
        ucase_api_call(name, 1000).await
    });
    let async_ldata = create_resource(cx, name, |name| async move {
        lcase_api_call(name, 1500).await
    });

    view! { cx,
        <input
            on:input=move |ev| {
                set_name(event_target_value(&ev));
            }

            prop:value=name
        />
        <p>
            <code>"name:"</code>
            {name}
        </p>
        // the fallback will show whenever a resource
        // read "under" the suspense is loading
        <Suspense fallback=move || view! { cx, <p>"Loading..."</p> }>
            // the children will be rendered once initially,
            // and then whenever all resources have been resolved
            <p>"Your shouting name is " {move || async_udata.read(cx)}</p>
            <p>"Your quiet name is " {move || async_ldata.read(cx)}</p>
        </Suspense>
    }
}

fn main() {
    leptos::mount_to_body(|cx| view! { cx, <App/> })
}
