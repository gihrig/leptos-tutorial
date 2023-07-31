/* 3.6f Control Flow - Type Conversions */
use leptos::*;
// A Few Tips

// When thinking about control flow with Leptos, it’s important
// to remember a few things:

// 1. Rust is an expression-oriented language: control-flow
//    expressions like if x() { y } else { z } and
//    match x() { ... } return their values. This makes them
//    very useful for declarative user interfaces.
// 2. For any T that implements IntoView—in other words, for
//    any type that Leptos knows how to render—Option<T> and
//    Result<T, impl Error> also implement IntoView. And just
//    as Fn() -> T renders a reactive T, Fn() -> Option<T>
//    and Fn() -> Result<T, impl Error> are reactive.
// 3. Rust has lots of handy helpers like Option::map,
//    Option::and_then, Option::ok_or, Result::map, Result::ok,
//    and bool::then that allow you to convert, in a declarative
//    way, between a few different standard types, all of which
//    can be rendered.
//    Spending time in the Option and Result docs in particular
//    is one of the best ways to level up your Rust game.
// 4. And always remember: to be reactive, values must be functions.
//    You’ll see me constantly wrap things in a move || closure,
//    below. This is to ensure that they actually rerun when the
//    signal they depend on changes, keeping the UI reactive.

// To connect the dots a little: this means that you can actually
// implement most of your control flow with native Rust code,
// without any control-flow components or special knowledge.

// For example, let’s start with a simple signal and derived signal:

/*
  let (value, set_value) = create_signal(cx, 0);
  let is_odd = move || value() & 1 == 1;

  If you don’t recognize what’s going on with is_odd, don’t
  worry about it too much. It’s just a simple way to test
  whether an integer is odd by doing a bitwise AND with 1.
*/

// We can use these signals and ordinary Rust to build most
// control flow.

// ----------------------------------------------------------------
// Type Conversions
// ----------------------------------------------------------------

// The view macro doesn’t return the most-generic wrapping type `View`.
// Instead, it returns things with types like `Fragment` or
// `HtmlElement<Input>`. This can be a little annoying if you’re
// returning different HTML elements from different branches of a
// conditional:
/*
  <main>
      {move || match is_odd() {
          true if value() == 1 => {
              // returns HtmlElement<Pre>
              view! { cx, <pre>"One"</pre> }
          },
          false if value() == 2 => {
              // returns HtmlElement<P>
              view! { cx, <p>"Two"</p> }
              ^^^^^^^^^^^^^^^^^^^^^^^^^^
              `match` arms have incompatible types
              expected struct `HtmlElement<Pre>`
              found struct `HtmlElement<P>`
          }
          // returns HtmlElement<Textarea>
          _ => view! { cx, <textarea>{value()}</textarea> }
      }}
  </main>
*/

// This strong typing is actually very powerful, because HtmlElement
// is, among other things, a smart pointer: each HtmlElement<T> type
// implements Deref for the appropriate underlying web_sys type. In
// other words, in the browser your view returns real DOM elements,
// and you can access native DOM methods on them.
//
// But it can be a little annoying in conditional logic like this,
// because you can’t return different types from different branches
// of a condition in Rust. There are two ways to get yourself out of
// this situation:

// 1. If you have multiple HtmlElement types, convert them to
//    HtmlElement<AnyElement> with .into_any()
// 2. If you have a variety of view types that are not all HtmlElement,
// convert them to Views with .into_view(cx).

#[component]
fn App(cx: Scope) -> impl IntoView {
    let (value, set_value) = create_signal(cx, 0);
    let is_odd = move || value() & 1 == 1;

    view! { cx,
        <h1>"Control Flow - Type Conversion"</h1>

        // Simple UI to update and show a value
        <button on:click=move |_| set_value.update(|n| *n += 1)>
            "+1"
        </button>
        <p>"Value is: " {value}</p>

        <hr/>

        // Here’s the above example, with the `.into_any`
        // conversion added:
        <main>
          {move || match is_odd() {
              true if value() == 1 => {
                  // returns HtmlElement<Pre>
                  view! { cx, <pre>"One"</pre> }.into_any()
              },
              false if value() == 2 => {
                  // returns HtmlElement<P>
                  view! { cx, <p>"Two"</p> }.into_any()
              }
              // returns HtmlElement<Textarea>
              _ => view! { cx, <textarea>{value()}</textarea> }.into_any()
          }}
        </main>

    }
}

fn main() {
    leptos::mount_to_body(|cx| view! { cx, <App/> })
}
