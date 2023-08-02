/* 3.8c Parent-Child Communication - Event Listener */
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

// 3. Event Listener

// You can actually write Option 2 (a Callback) in a slightly different
// way. If the callback maps directly onto a native DOM event, you can
// add an `on:` listener directly to the place you use the component in
// your view macro in <App/>.

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    let (toggled, set_toggled) = create_signal(cx, false);
    view! { cx,
      <h1>"Parent-Child Communication - Event Listener"</h1>
      <p>"Parent: Toggled? " {toggled}</p>
      // note the on:click instead of on_click
      // this is the same syntax as an HTML element event listener
      // `*value = !*value` is a simple invert of the boolean `*value`
      <ButtonC on:click=move |_| set_toggled.update(|value| *value = !*value)/>
    }
}

#[component]
pub fn ButtonC(cx: Scope) -> impl IntoView {
    view! { cx,
      <span>"Child: "</span>
      <button>"Toggle"</button>
    }
}
fn main() {
    leptos::mount_to_body(|cx| view! { cx, <App/> })
}

// This lets you write way less code in <ButtonC/> than you did for
// <ButtonB/>, and still gives a correctly-typed event to the listener.
// This works by adding an on: event listener to each element that
// <ButtonC/> returns: in this case, just the one <button>.

// Of course, this only works for actual DOM events that you’re passing
// directly through to the elements you’re rendering in the component.
// For more complex logic that doesn’t map directly onto an element
// (say you create <ValidatedForm/> and want an on_valid_form_submit
// callback) you should use Option 2.
