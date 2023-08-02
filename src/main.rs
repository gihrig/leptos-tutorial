/* 3.8e Parent-Child Communication - Final Example */
use leptos::{ev::MouseEvent, *};

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
// These are covered in the previous commits.

// ----------------------------------------------------------------

// This highlights four different ways that child components can communicate
// with their parent:
// 1) <ButtonA/>: passing a WriteSignal as one of the child component props,
//    for the child component to write into and the parent to read
// 2) <ButtonB/>: passing a closure as one of the child component props, for
//    the child component to call
// 3) <ButtonC/>: adding an `on:` event listener to a component
// 4) <ButtonD/>: providing a context that is used in the component (rather than prop drilling)

#[derive(Copy, Clone)]
struct SmallcapsContext(WriteSignal<bool>);

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // just some signals to toggle three classes on our <p>
    let (red, set_red) = create_signal(cx, false);
    let (right, set_right) = create_signal(cx, false);
    let (italics, set_italics) = create_signal(cx, false);
    let (smallcaps, set_smallcaps) = create_signal(cx, false);

    // the newtype pattern isn't *necessary* here but is a good practice
    // it avoids confusion with other possible future `WriteSignal<bool>` contexts
    // and makes it easier to refer to it in ButtonC
    provide_context(cx, SmallcapsContext(set_smallcaps));

    view! { cx,
      <main>
        <h1>"Parent-Child Communication - Final Example"</h1>
        <p
          // class: attributes take F: Fn() => bool, and these signals all implement Fn()
          class:red=red
          class:right=right
          class:italics=italics
          class:smallcaps=smallcaps
        >
          "Lorem ipsum sit dolor amet."
        </p>

        // Button A: pass the signal setter
        <ButtonA setter=set_red/>

        // Button B: pass a closure
        <ButtonB on_click=move |_| set_right.update(|value| *value = !*value)/>

        // Button B: use a regular event listener
        // setting an event listener on a component like this applies it
        // to each of the top-level elements the component returns
        <ButtonC on:click=move |_| set_italics.update(|value| *value = !*value)/>

        // Button D gets its setter from context rather than props
        <ButtonD/>
      </main>
    }
}

/// Button A receives a signal setter and updates the signal itself
#[component]
pub fn ButtonA(
    cx: Scope,
    /// Signal that will be toggled when the button is clicked.
    setter: WriteSignal<bool>,
) -> impl IntoView {
    view! { cx,
      <button on:click=move |_| setter.update(|value| *value = !*value)>"Toggle Red"</button>
    }
}

/// Button B receives a closure
#[component]
pub fn ButtonB<F>(
    cx: Scope,
    /// Callback that will be invoked when the button is clicked.
    on_click: F,
) -> impl IntoView
where
    F: Fn(MouseEvent) + 'static,
{
    view! { cx, <button on:click=on_click>"Toggle Right"</button> }

    // just a note: in an ordinary function ButtonB could take on_click: impl Fn(MouseEvent)
    // + 'static and save you from typing out the generic.
    // The component macro actually expands to define a struct:
    //
    // struct ButtonBProps<F> where F: Fn(MouseEvent) + 'static {
    //   on_click: F
    // }
    //
    // this is what allows us to have named props in our component invocation,
    // instead of an ordered list of function arguments
    // if Rust ever gets named function arguments we could drop this requirement
}

/// Button C is a dummy: it renders a button but doesn't handle
/// its click. Instead, the parent component adds an event listener.
#[component]
pub fn ButtonC(cx: Scope) -> impl IntoView {
    view! { cx, <button>"Toggle Italics"</button> }
}

/// Button D is very similar to Button A, but instead of passing the setter as a prop
/// we get it from the context
#[component]
pub fn ButtonD(cx: Scope) -> impl IntoView {
    let setter = use_context::<SmallcapsContext>(cx).unwrap().0;

    view! { cx,
      <button on:click=move |_| setter.update(|value| *value = !*value)>"Toggle Small Caps"</button>
    }
}

fn main() {
    leptos::mount_to_body(|cx| view! { cx, <App/> })
}
