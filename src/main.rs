/* 3.8d Parent-Child Communication - Providing a Context */
use leptos::*;

// You can think of your application as a nested tree of components.
// Each component handles its own local state and manages a section
// of the user interface, so components tend to be relatively
// self-contained.

// Sometimes, though, you’ll want to communicate between a parent
// component and its child. For example, imagine you’ve defined a
// <FancyButton/> component that adds some styling, logging, or
// something else to a <button/>. You want to use a <FancyButton/>
// in your <App/> component. But how can you communicate between
// the two?

// It’s easy to communicate state from a parent component to a child
// component. We covered some of this in the material on components
// and props. Basically if you want the parent to communicate to the
// child, you can pass a ReadSignal, a Signal, or even a MaybeSignal
// as a prop.

// But what about the other direction? How can a child send
// notifications about events or state changes back up to the parent?

// There are four basic patterns of parent-child communication in Leptos.

// 4. Providing a Context

// This version is actually a variant on Option 1. Say you have a
// deeply-nested component tree:

/*
  #[component]
  pub fn App(cx: Scope) -> impl IntoView {
      let (toggled, set_toggled) = create_signal(cx, false);
      view! { cx,
          <p>"Toggled? " {toggled}</p>
          <Layout/>
      }
  }

  #[component]
  pub fn Layout(cx: Scope) -> impl IntoView {
      view! { cx,
          <header>
              <h1>"My Page"</h1>
          </header>
          <main>
              <Content/>
          </main>
      }
  }

  #[component]
  pub fn Content(cx: Scope) -> impl IntoView {
      view! { cx,
          <div class="content">
              <ButtonD/>
          </div>
      }
  }

  #[component]
  pub fn ButtonD<F>(cx: Scope) -> impl IntoView {
      todo!()
  }
*/

// Now <ButtonD/> is no longer a direct child of <App/>, so you can’t
// simply pass your WriteSignal to its props. You could do what’s
// sometimes called “prop drilling,” adding a prop to each layer between
// the two:

/*
  #[component]
  pub fn App(cx: Scope) -> impl IntoView {
      let (toggled, set_toggled) = create_signal(cx, false);
      view! { cx,
          <p>"Toggled? " {toggled}</p>
          <Layout set_toggled/>
      }
  }

  #[component]
  pub fn Layout(cx: Scope, set_toggled: WriteSignal<bool>) -> impl IntoView {
      view! { cx,
          <header>
              <h1>"My Page"</h1>
          </header>
          <main>
              <Content set_toggled/>
          </main>
      }
  }

  #[component]
  pub fn Content(cx: Scope, set_toggled: WriteSignal<bool>) -> impl IntoView {
      view! { cx,
          <div class="content">
              <ButtonD set_toggled/>
          </div>
      }
  }

  #[component]
  pub fn ButtonD<F>(cx: Scope, set_toggled: WriteSignal<bool>) -> impl IntoView {
      todo!()
  }
*/

// This is a mess! <Layout/> and <Content/> don’t need set_toggled; they
// just pass it through to <ButtonD/>. But I need to declare the prop in
// triplicate. This is not only annoying but hard to maintain: imagine
// we add a “half-toggled” option and the type of set_toggled needs to
// change to an enum. We have to change it in three places!

// Isn’t there some way to skip levels?

// There is!

// The Context API

// You can provide data that skips levels by using provide_context and
// use_context. Contexts are identified by the type of the data you
// provide (in this example, WriteSignal<bool>), and they exist in a
// top-down tree that follows the contours of your UI tree. In this
// example, we can use context to skip the unnecessary prop drilling.

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    let (toggled, set_toggled) = create_signal(cx, false);

    // share `set_toggled` with all children of this component
    provide_context(cx, set_toggled);

    view! { cx,
        <h1>"Parent-Child Communication - Providing a Context"</h1>
        <strong>"Parent: "</strong>
        <span>"Toggled? " {toggled}</span>
        <Layout/>
    }
}

// <Layout/> and <Content/> no longer pass `set_toggled`
#[component]
pub fn Layout(cx: Scope) -> impl IntoView {
    view! { cx,
        <header>
            <h4>"Layout..."</h4>
        </header>
        <main>
            <Content/>
        </main>
    }
}

#[component]
pub fn Content(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="content">
          <h4>"Content..."</h4>
          <ButtonD/>
        </div>
    }
}

#[component]
pub fn ButtonD(cx: Scope) -> impl IntoView {
    // use_context searches up the context tree, hoping to
    // find a `WriteSignal<bool>`
    // in this case, I .expect() because I know I provided it
    let setter = use_context::<WriteSignal<bool>>(cx).expect("to have found the setter provided");

    view! { cx,
        <button
            on:click=move |_| setter.update(|value| *value = !*value)
        >
          <span>{"Child: "}</span>
          "Toggle"
        </button>
    }
}

fn main() {
    leptos::mount_to_body(|cx| view! { cx, <App/> })
}

// The same caveats apply to this as to <ButtonA/>: passing a WriteSignal
// around should be done with caution, as it allows you to mutate state
// from arbitrary parts of your code. But when done carefully, this can
// be one of the most effective techniques for global state management in
// Leptos: simply provide the state at the highest level you’ll need it,
// and use it wherever you need it lower down.

// Note that there are no performance downsides to this approach. Because
// you are passing a fine-grained reactive signal, nothing happens in the
// intervening components (<Layout/> and <Content/>) when you update it.
// You are communicating directly between <ButtonD/> and <App/>.
// In fact—and this is the power of fine-grained reactivity—you are
// communicating directly between a button click in <ButtonD/> and a
// single text node in <App/>. It’s as if the components themselves don’t
// exist at all. And, well... at runtime, they don’t. It’s just signals
// and effects, all the way down.
