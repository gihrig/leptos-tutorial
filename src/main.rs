/* 10.1 Styling - TailwindCSS */

// Anyone creating a website or application soon runs into the question
// of styling. For a small app, a single CSS file is probably plenty to
// style your user interface. But as an application grows, many developers
// find that plain CSS becomes increasingly hard to manage.

// Some frontend frameworks (like Angular, Vue, and Svelte) provide
// built-in ways to scope your CSS to particular components, making it
// easier to manage styles across a whole application without styles
// meant to modify one small component having a global effect. Other
// frameworks (like React or Solid) don’t provide built-in CSS scoping,
// but rely on libraries in the ecosystem to do it for them. Leptos is
// in this latter camp: the framework itself has no opinions about CSS
// at all, but provides a few tools and primitives that allow others to
// build styling libraries.

// Here are a few different approaches to styling your Leptos app, other
// than plain CSS.

// --------------------------------------------------------------------
// TailwindCSS: Utility-first CSS
// --------------------------------------------------------------------

// TailwindCSS is a popular utility-first CSS library. It allows you to
// style your application by using inline utility classes, with a custom
// CLI tool that scans your files for Tailwind class names and bundles
// the necessary CSS.

// This allows you to write components like this:

/*
  #[component]
  fn Home(cx: Scope) -> impl IntoView {
      let (count, set_count) = create_signal(cx, 0);

      view! { cx,
          <main class="my-0 mx-auto max-w-3xl text-center">
              <h2 class="p-6 text-4xl">"Welcome to Leptos with Tailwind"</h2>
              <p class="px-10 pb-10 text-left">"Tailwind will scan your Rust files for Tailwind class names and compile them into a CSS file."</p>
              <button
                  class="bg-sky-600 hover:bg-sky-700 px-5 py-3 text-white rounded-lg"
                  on:click=move |_| set_count.update(|count| *count += 1)
              >
                  {move || if count() == 0 {
                      "Click me!".to_string()
                  } else {
                      count().to_string()
                  }}
              </button>
          </main>
      }
  }
*/

// It can be a little complicated to set up the Tailwind integration at
// first, but you can check out our two examples of how to use Tailwind
// with a client-side-rendered trunk application
// https://github.com/leptos-rs/leptos/tree/main/examples/tailwind_csr_trunk

// or with a server-rendered cargo-leptos application.
// https://github.com/leptos-rs/leptos/tree/main/examples/tailwind

// cargo-leptos also has some built-in Tailwind support that you can
// use as an alternative to Tailwind’s CLI.
// https://github.com/leptos-rs/cargo-leptos#site-parameters

// ----------------------------------------------------------------------
// TailwindCSS Example - from leptos examples tailwind_csr_trunk
// ----------------------------------------------------------------------

mod app;

use app::*;
use leptos::*;

pub fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    log!("csr mode - mounting to body");

    mount_to_body(|cx| {
        view! { cx, <App/> }
    });
}
