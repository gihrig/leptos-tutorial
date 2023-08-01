/* 3.8a Parent-Child Communication - WriteSignal */
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

// 1. WriteSignal

// One approach is simply to pass a WriteSignal from the parent down
// to the child, and update it in the child. This lets you manipulate
// the state of the parent from the child.

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    let (toggled, set_toggled) = create_signal(cx, false);
    view! { cx,
      <h1>"Parent-Child Communication - WriteSignal"</h1>
      <p>"Parent: Toggled? " {toggled}</p>
      <ButtonA setter=set_toggled/>
    }
}

#[component]
pub fn ButtonA(cx: Scope, setter: WriteSignal<bool>) -> impl IntoView {
    view! { cx,
      <span>"Child: "</span>
      // `*value = !*value` is a simple invert of the boolean `*value`
      <button on:click=move |_| setter.update(|value| *value = !*value)>"Toggle"</button>
    }
}
fn main() {
    leptos::mount_to_body(|cx| view! { cx, <App/> })
}

// This pattern is simple, but you should be careful with it:
// passing around a WriteSignal can make it hard to reason about
// your code. In this example, it’s pretty clear when you read
// <App/> that you are handing off the ability to mutate `toggled`,
// but it’s not at all clear when or how it will change. In this
// small, local example it’s easy to understand, but if you find
// yourself passing around WriteSignals like this throughout your
// code, you should really consider whether this is making it too
// easy to write spaghetti code.
