/* 3.6g Control Flow - Final Example */

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
// Final Example
// ----------------------------------------------------------------
use leptos::*;

#[component]
fn App(cx: Scope) -> impl IntoView {
    let (value, set_value) = create_signal(cx, 0);
    let is_odd = move || value() & 1 == 1;
    let odd_text = move || if is_odd() { Some("How odd!") } else { None };

    view! { cx,
        <h1>"Control Flow - Final Example"</h1>

        // Simple UI to update and show a value
        <button on:click=move |_| set_value.update(|n| *n += 1)>
            "+1"
        </button>
        <p>"Value is: " {value}</p>

        <hr/>

        <h2><code>"Option<T>"</code></h2>
        // For any `T` that implements `IntoView`,
        // so does `Option<T>`

        <p>": "{odd_text}</p>
        // This means you can use `Option` methods on it
        <p>": "{move || odd_text().map(|text| text.len())}</p>

        <h2>"Conditional Logic"</h2>
        // You can do dynamic conditional if-then-else
        // logic in several ways
        //
        // a. An "if" expression in a function
        //    This will simply re-render every time the value
        //    changes, which makes it good for lightweight UI
        <p>
            {move || if is_odd() {
                "Odd"
            } else {
                "Even"
            }}
        </p>

        // b. Toggling some kind of class
        //    This is smart for an element that's going
        //    to be toggled often, because it isn't destroyed
        //    between states (you can find the `red` class
        //    in `index.html`)
        <p class:red=is_odd>"Red if odd."</p>

        // c. The <Show/> component
        //    This only renders the fallback and the child
        //    once, lazily, and toggles between them when
        //    needed. This makes it more efficient in many cases
        //    than a {move || if ...} block
        <Show when=is_odd
            fallback=|cx| view! { cx, <p>"Even steven"</p> }
        >
            <p>"Oddment"</p>
        </Show>

        // d. Because `bool::then()` converts a `bool` to
        //    `Option`, you can use it to create a show/hide
        //    or other toggle
        {move || is_odd().then(|| view! { cx, <p>"Oddity!"</p> })
        .unwrap_or_else(|| view! { cx, <p>"Evenity!"</p> })}

        <h2>"Converting between Types"</h2>
        // e. Note: if branches return different types,
        //    you can convert between them with
        //    `.into_any()` (for different HTML element types)
        //    or `.into_view(cx)` (for all view types)
        {move || match is_odd() {
            true if value() == 1 => {
                // <pre> returns HtmlElement<Pre>
                view! { cx, <pre>"The One"</pre> }.into_any()
            },
            false if value() == 2 => {
                // <p> returns HtmlElement<P>
                // so we convert into a more generic type
                view! { cx, <p>"The Two"</p> }.into_any()
            }
            _ => view! { cx, <textarea>{value()}</textarea> }.into_any()
        }}
    }
}

fn main() {
    leptos::mount_to_body(|cx| view! { cx, <App/> })
}
