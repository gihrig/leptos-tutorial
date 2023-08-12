/* 10.3 Styling - Styled */

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
// Styled: Runtime CSS Scoping
// --------------------------------------------------------------------

// Styled
// https://github.com/eboody/styled
// is a runtime scoped CSS library that integrates well with Leptos. It lets you
// declare scoped CSS in the body of your component function, and then applies those
// styles at runtime.

/*
use styled::style;

#[component]
pub fn MyComponent(cx: Scope) -> impl IntoView {
    let styles = style!(
      div {
        background-color: red;
        color: white;
      }
    );

    styled::view! { cx, styles,
        <div>"This text should be red with white text."</div>
    }
}
*/

// ----------------------------------------------------------------
// Styled Example from
// https://github.com/eboody/styled#usage
// Seems broken:
// Compiling leptos-tutorial v0.1.0 (/Users/glen/Documents/Development/Study/Rust/Leptos/leptos-tutorial)
// error[E0277]: the trait bound `&fn(leptos_reactive::scope::Scope, StyleProps) -> impl leptos_dom::IntoView {Style}: ComponentConstructor<_>` is not satisfied
//    --> src/main.rs:65:5
//     |
// 65  | /     styled::view! {
// 66  | |         cx,
// 67  | |         styles,
// 68  | |         <div>"This text should be red with white text."</div>
// 69  | |     }
//     | |_____^ the trait `ComponentConstructor<_>` is not implemented for `&fn(leptos_reactive::scope::Scope, StyleProps) -> impl leptos_dom::IntoView {Style}`
//     |
// note: required by a bound in `component_view`
//    --> /Users/glen/.cargo/registry/src/index.crates.io-6f17d22bba15001f/leptos-0.4.8/src/lib.rs:290:13
//     |
// 289 | pub fn component_view<P>(
//     |        -------------- required by a bound in this function
// 290 |     f: impl ComponentConstructor<P>,
//     |             ^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `component_view`
//     = note: this error originates in the macro `view` which comes from the expansion of the macro `styled::view` (in Nightly builds, run with -Z macro-backtrace for more info)
// ----------------------------------------------------------------
use leptos::*;
use styled::style;

#[component]
pub fn MyComponent(cx: Scope) -> impl IntoView {
    let styles = style!(
      div {
        background-color: red;
        color: white;
      }
    );

    styled::view! {
        cx,
        styles,
        <div>"This text should be red with white text."</div>
    }
}

#[component]
pub fn AnotherComponent(cx: Scope) -> impl IntoView {
    // note were using a plain div selector and it wont clash with MyComponent's div style!
    let styles = style!(
      div {
        background-color: blue;
        color: gray;
      }
    );

    styled::view! {
        cx,
        styles,
        <div>"This text should be blue with gray text."</div>
    }
}

pub fn main() {
    println!["Hello, stylers!"];
    mount_to_body(|cx| {
        view! { cx,
          <MyComponent/>
          <AnotherComponent/>
        }
    });
}
