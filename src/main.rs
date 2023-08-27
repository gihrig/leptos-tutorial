/* 10.4 Styling - Styled - Longer */

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
// Styled: Runtime CSS Scoping - Longer example
// https://github.com/eboody/styled#longer-example
// ----------------------------------------------------------------

use leptos::*;

pub mod components {
    pub mod button;
    pub mod theme;
}

use crate::components::button::{Button, Variant};

#[component]
pub fn HomePage(cx: Scope) -> impl IntoView {
    view! { cx,
      // TODO: css generated for first button only
      // Variant not the issue, only the first
      // button in code is styled
        <Button variant=Variant::PRIMARY/>
        <Button variant=Variant::SECONDARY/>
        <Button variant=Variant::ALERT/>
    }
}

pub fn main() {
    println!["Hello, styles!"];
    mount_to_body(|cx| {
        view! { cx, <HomePage/> }
    });
}
