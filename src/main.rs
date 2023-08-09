/* 8.0.d Global State Management - Final Example */

// So far, we've only been working with local state in components,
// and we’ve seen how to coordinate state between parent and child
// components. On occasion, there are times where people look for a
// more general solution for global state management that can work
// throughout an application.

// In general, you do not need this chapter. The typical pattern is
// to compose your application out of components, each of which manages
// its own local state, not to store all state in a global structure.
// However, there are some cases (like theming, saving user settings,
// or sharing data between components in different parts of your UI)
// in which you may want to use some kind of global state management.

// The three best approaches to global state are:

// 1. Using the router to drive global state via the URL
// 2. Passing signals through context
// 3. Creating a global state struct and creating lenses into it with
//    `create_slice`

// --------------------------------------------------------------------
// 8.0.d Global State Management - Final Example
// --------------------------------------------------------------------

use leptos::*;

// In virtual DOM libraries like React, using the Context API to manage
// global state is a bad idea: because the entire app exists in a tree,
// changing some value provided high up in the tree can cause the whole
// app to render.
//
// In fine-grained reactive libraries like Leptos, this is simply not
// the case. You can create a signal in the root of your app and pass
// it down to other components using provide_context(). Changing it
// will only cause rerendering in the specific places it is actually
// used, not the whole app.

#[component]
fn Option2(cx: Scope) -> impl IntoView {
    // here we create a signal in the root that can be consumed
    // anywhere in the app.
    let (count, set_count) = create_signal(cx, 0);
    // we'll pass the setter to specific components,
    // but provide the count itself to the whole app via context
    provide_context(cx, count);

    view! { cx,
        <h1>"Option 2: Passing Signals"</h1>
        // SetterButton is allowed to modify the count
        <SetterButton set_count/>
        // These consumers can only read from it
        // But we could give them write access by passing `set_count`
        // if we wanted
        <div style="display: flex">
            <FancyMath/>
            <ListItems/>
        </div>
    }
}

/// A button that increments our global counter.
#[component]
fn SetterButton(cx: Scope, set_count: WriteSignal<u32>) -> impl IntoView {
    view! { cx,
        <div class="provider red">
            <button on:click=move |_| {
                set_count.update(|count| *count += 1)
            }>"Increment Global Count"</button>
        </div>
    }
}

/// A component that does some "fancy" math with the global count
#[component]
fn FancyMath(cx: Scope) -> impl IntoView {
    // here we consume the global count signal with `use_context`
    let count = use_context::<ReadSignal<u32>>(cx)
        // we know we just provided this in the parent component
        .expect("there to be a `count` signal provided");
    let is_even = move || count() & 1 == 0;

    view! { cx,
        <div class="consumer blue">
            "The number " <strong>{count}</strong>
            {move || if is_even() { " is" } else { " is not" }} " even."
        </div>
    }
}

/// A component that shows a list of items generated from the global count.
#[component]
fn ListItems(cx: Scope) -> impl IntoView {
    // again, consume the global count signal with `use_context`
    let count = use_context::<ReadSignal<u32>>(cx)
        .expect("there to be a `count` signal provided");

    let squares = move || {
        (0..count())
            .map(|n| view! { cx, <li>{n} <sup>"2"</sup> " is " {n * n}</li> })
            .collect::<Vec<_>>()
    };

    view! { cx,
        <div class="consumer green">
            <ul>{squares}</ul>
        </div>
    }
}

// Option #3: Create a Global State Struct
//
// You can use this approach to build a single global data structure
// that holds the state for your whole app, and then access it by
// taking fine-grained slices using `create_slice` or `create_memo`,
// so that changing one part of the state doesn't cause parts of your
// app that depend on other parts of the state to change.

#[derive(Default, Clone, Debug)]
struct GlobalState {
    count: u32,
    name: String,
}

#[component]
fn Option3(cx: Scope) -> impl IntoView {
    // We'll provide a single signal that holds the whole state.
    // Each component will be responsible for creating its own "lens"
    // into that signal.
    let state = create_rw_signal(cx, GlobalState::default());
    provide_context(cx, state);

    view! { cx,
        <h1>"Option 3: Passing Global State Struct"</h1>
        <div class="red consumer" style="width: 100%">
            <h2>"Current Global State"</h2>
            <pre>{move || { format!("{:#?}", state.get()) }}</pre>
        </div>
        <div style="display: flex">
            <GlobalStateCounter/>
            <GlobalStateInput/>
        </div>
    }
}

/// A component that updates the count in the global state.
#[component]
fn GlobalStateCounter(cx: Scope) -> impl IntoView {
    let state = use_context::<RwSignal<GlobalState>>(cx)
        .expect("state to have been provided");

    // `create_slice` lets us create a "lens" into the data
    let (count, set_count) = create_slice(
        cx,
        // we take a slice *from* `state`
        state,
        // our getter returns a "slice" of the data
        |state| state.count,
        // our setter describes how to mutate that slice, given a new value
        |state, n| state.count = n,
    );

    view! { cx,
        <div class="consumer blue">
            <button on:click=move |_| {
                set_count(count() + 1);
            }>

                "Increment Global Count"
            </button>
            <br/>
            <span>"Count is: " {count}</span>
        </div>
    }
}

/// A component that updates the count in the global state.
#[component]
fn GlobalStateInput(cx: Scope) -> impl IntoView {
    let state = use_context::<RwSignal<GlobalState>>(cx)
        .expect("state to have been provided");

    // this slice is completely independent of the `count` slice
    // that we created in the other component
    // neither of them will cause the other to rerun
    let (name, set_name) = create_slice(
        cx,
        // we take a slice *from* `state`
        state,
        // our getter returns a "slice" of the data
        |state| state.name.clone(),
        // our setter describes how to mutate that slice, given a new value
        |state, n| state.name = n,
    );

    view! { cx,
        <div class="consumer green">
            <input
                type="text"
                prop:value=name
                on:input=move |ev| {
                    set_name(event_target_value(&ev));
                }
            />

            <br/>
            <span>"Name is: " {name}</span>
        </div>
    }
}
// This `main` function is the entry point into the app
// It just mounts our component to the <body>
// Because we defined it as `fn App`, we can now use it in a
// template as <App/>

fn main() {
    leptos::mount_to_body(|cx| {
        view! { cx,
            <Option2/>
            <Option3/>
        }
    })
}
