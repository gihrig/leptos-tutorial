/* 3.8b Parent-Child Communication - Callback */
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

// 2. Callback

// Another approach would be to pass a callback to the child: say,
// on_click.

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    let (toggled, set_toggled) = create_signal(cx, false);
    view! { cx,
      <h1>"Parent-Child Communication - Callback"</h1>
      <p>"Parent: Toggled? " {toggled}</p>
      // `*value = !*value` is a simple invert of the boolean `*value`
      <ButtonB on_click=move |_| set_toggled.update(|value| *value = !*value)/>
    }
}

#[component]
pub fn ButtonB<F>(cx: Scope, on_click: F) -> impl IntoView
where
    F: Fn(MouseEvent) + 'static,
{
    view! { cx,
      <span>"Child: "</span>
      <button on:click=on_click>"Toggle"</button>
    }
}
fn main() {
    leptos::mount_to_body(|cx| view! { cx, <App/> })
}

// You’ll notice that whereas <ButtonA/> was given a WriteSignal and
// decided how to mutate it, <ButtonB/> simply fires an event: the
// mutation happens back in <App/>. This has the advantage of keeping
// local state local, preventing the problem of spaghetti mutation.
// But it also means the logic to mutate that signal needs to exist
// up in <App/>, not down in <ButtonB/>. These are real trade-offs,
// not a simple right-or-wrong choice.

// Note the way we declare the generic type F here for the callback.
// If you’re confused, look back at the generic props section of the
// chapter on components.
