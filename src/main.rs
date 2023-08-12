/* 10.2 Styling - Stylers */

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
// Stylers - Compile-time CSS Extraction
// --------------------------------------------------------------------

// Stylers
// https://github.com/abishekatp/stylers
// is a compile-time scoped CSS library that lets you declare scoped
// CSS in the body of your component. Stylers will extract this CSS
// at compile time into CSS files that you can then import into your
// app, which means that it doesn’t add anything to the WASM binary
// size of your application.

// This allows you to write components like this:

/*
use stylers::style;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    let styler_class = style! { "App",
        two{
            color: blue;
        }
        div.one{
            color: red;
            content: raw_str(r#"\hello"#);
            font: "1.3em/1.2" Arial, Helvetica, sans-serif;
        }
        div {
            border: 1px solid black;
            margin: 25px 50px 75px 100px;
            background-color: lightblue;
        }
        h2 {
            color: purple;
        }
        @media only screen and (max-width: 1000px) {
            h3 {
                background-color: lightblue;
                color: blue
            }
        }
    };

    view! { cx, class = styler_class,
        <div class="one">
            <h1 id="two">"Hello"</h1>
            <h2>"World"</h2>
            <h2>"and"</h2>
            <h3>"friends!"</h3>
        </div>
    }
}
*/

// ----------------------------------------------------------------
// Stylers Example from
// https://github.com/abishekatp/stylers/tree/main/examples/style
// Seems broken:
// Compiling stylers v0.3.1
// error[E0615]: attempted to take value of method `column` on type `proc_macro::Span`
//   --> /Users/glen/.cargo/registry/src/index.crates.io-6f17d22bba15001f/stylers-0.3.1/src/style/css_style_rule.rs:74:47
//    |
// 74 | ...                   pre_col = end.column;
//    |                                     ^^^^^^ method, not a field
//    |
// help: use parentheses to call the method
//    |
// 74 |                                 pre_col = end.column();
// Plus 5 more
// ----------------------------------------------------------------
use leptos::*;
use style_macro::*;

pub fn main() {
    println!["Hello, stylers!"];
    mount_to_body(|cx| view! { cx,  <Abi/> });
}
