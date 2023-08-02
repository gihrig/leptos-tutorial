/* 3.9a Component Children - Takes Children */
use leptos::*;

// It’s pretty common to want to pass children into a component, just as
// you can pass children into an HTML element. For example, imagine I
// have a <FancyForm/> component that enhances an HTML <form>. I need
// some way to pass all its inputs.

/*
  view! { cx,
      <Form>
          <fieldset>
              <label>
                  "Some Input"
                  <input type="text" name="something"/>
              </label>
          </fieldset>
          <button>"Submit"</button>
      </Form>
  }
*/

// How can you do this in Leptos? There are basically two ways to pass
// components to other components:

// 1. `render props`: properties that are functions that return a view
// 2. the `children` prop: a special component property that includes
//    anything you pass as a child to the component.

// In fact, you’ve already seen these both in action in the <Show/>
// component:

/*
  view! { cx,
    <Show
      // `when` is a normal prop
      when=move || value() > 5
      // `fallback` is a "render prop": a function that returns a view
      fallback=|cx| view! { cx, <Small/> }
    >
      // `<Big/>` (and anything else here)
      // will be given to the `children` prop
      <Big/>
    </Show>
  }
*/

// ------------------------------------------------------------------

// Takes Children

// Let’s define a component that takes some `children` and a `render prop`.

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    view! { cx,
      <TakesChildren render_prop=|| view! { cx, <p>"Hi, there!"</p> }>
        // these get passed to `children`
        <p>"Some text"</p>
        <span>"A span"</span>
      </TakesChildren>
    }
}

#[component]
pub fn TakesChildren<F, IV>(
    cx: Scope,
    /// Takes a function (type F) that returns anything that can be
    /// converted into a View (type IV)
    render_prop: F,
    /// `children` takes the `Children` type
    children: Children,
) -> impl IntoView
where
    F: Fn() -> IV,
    IV: IntoView,
{
    view! { cx,
      <h2>"Render Prop"</h2>
      {render_prop()}

      <h2>"Children"</h2>
      {children(cx)}
    }
}

// `render_prop` and `children` are both functions, so we can call them to
// generate the appropriate views. `children`, in particular, is an alias
// for Box<dyn FnOnce(Scope) -> Fragment>. (Aren't you glad we named it
// Children instead?)

// If you need a Fn or FnMut here because you need to call children more
// than once, we also provide ChildrenFn and ChildrenMut aliases.

fn main() {
    leptos::mount_to_body(|cx| view! { cx, <App/> })
}
