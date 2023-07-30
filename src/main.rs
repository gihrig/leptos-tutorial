/* 3.6e Control Flow - Preventing Over-Rendering <Show/> */
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
// Preventing Over-Rendering <Show/>
// ----------------------------------------------------------------

#[component]
fn Big(_cx: Scope) -> impl IntoView {
    log!("rendering Big");
    "Big"
}

#[component]
fn Small(_cx: Scope) -> impl IntoView {
    log!("rendering Small");
    "Small"
}

// The <Show/> component is the answer to over-rendering. You pass
// it a `when` condition function, a fallback to be shown if the
// when function returns false, and children to be rendered if
// `when` returns true.

// <Show/> memoizes the when condition, so it only renders its
// <Small/> once, continuing to show the same component until value
// is greater than five; then it renders <Big/> once, continuing to
// show it indefinitely.

// This is a helpful tool to avoid rerendering when using dynamic if
// expressions. As always, there's some overhead: for a very simple
// node (like updating a single text node, or updating a class or
// attribute), a move || if ... will be more efficient. But if it’s
// at all expensive to render either branch, reach for <Show/>.

#[component]
fn App(cx: Scope) -> impl IntoView {
    let (value, set_value) = create_signal(cx, 0);

    view! { cx,
        <h1>"Control Flow - Preventing Over-Rendering <Show/>"</h1>
        <h2>"<Big/> and <Small/> components only render as needed"</h2>
        <p>"See browser console"</p>

        // Simple UI to update and show a value
        <button on:click=move |_| set_value.update(|n| *n += 1)>
            "+1"
        </button>
        <p>"Value is: " {value}</p>

        <hr/>

        // See discussion above
        <Show
          when=move || { value() > 5 }
          fallback=|cx| view! { cx, <Small/> }
        >
          <Big/>
        </Show>
    }
}

fn main() {
    leptos::mount_to_body(|cx| view! { cx, <App/> })
}
