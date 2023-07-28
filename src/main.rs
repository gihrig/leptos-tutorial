/** 3.5b Forms and Inputs - Uncontrolled Inputs */
use leptos::{ev::SubmitEvent, *};

// In an "uncontrolled input," the browser controls the state
// of the input element. Rather than continuously updating a
// signal to hold its value, we use a NodeRef to access the
// input once when we want to get its value.

// In this example, we only notify the framework when the <form>
// fires a submit event.

/*
let (name, set_name) = create_signal(cx, "Uncontrolled".to_string());

let input_element: NodeRef<Input> = create_node_ref(cx);
*/

// NodeRef is a kind of reactive smart pointer: we can use it to
// access the underlying DOM node. Its value will be set when the
// element is rendered.

/*
let on_submit = move |ev: SubmitEvent| {
    // stop the page from reloading!
    ev.prevent_default();

    // here, we'll extract the value from the input
    let value = input_element()
        // event handlers can only fire after the view
        // is mounted to the DOM, so the `NodeRef` will be `Some`
        .expect("<input> to exist")
        // `NodeRef` implements `Deref` for the DOM element type
        // this means we can call`HtmlInputElement::value()`
        // to get the current value of the input
        .value();
    set_name(value);
};
*/

// Our on_submit handler will access the input’s value and use it
// to call set_name. To access the DOM node stored in the NodeRef,
// we can simply call it as a function (or using .get()). This will
// return Option<web_sys::HtmlInputElement>, but we know it will
// already have been filled when we rendered the view, so it’s safe
// to unwrap here.

// We can then call .value() to get the value out of the input,
// because NodeRef gives us access to a correctly-typed HTML
// element.

/*
view! { cx,
    <form on:submit=on_submit>
        <input type="text"
            value=name
            node_ref=input_element
        />
        <input type="submit" value="Submit"/>
    </form>
    <p>"Name is: " {name}</p>
}
*/

// The view should be pretty self-explanatory by now. Note two
// things:
//
// 1. Unlike in the controlled input example, we use value
//    (not prop:value). This is because we’re just setting the
//    initial value of the input, and letting the browser control
//    its state. (We could use prop:value instead.)
// 2. We use node_ref to fill the NodeRef. (Older examples
//    sometimes use _ref. They are the same thing, but node_ref
//    has better rust-analyzer support.)

#[component]
fn App(cx: Scope) -> impl IntoView {
    view! { cx,
        <h2>"Uncontrolled Component"</h2>
        <UncontrolledComponent/>
    }
}

#[component]
fn UncontrolledComponent(cx: Scope) -> impl IntoView {
    // import the type for <input>
    use leptos::html::Input;

    let (name, set_name) = create_signal(cx, "Uncontrolled".to_string());

    // we'll use a NodeRef to store a reference to the input element
    // this will be filled when the element is created
    let input_element: NodeRef<Input> = create_node_ref(cx);

    // fires when the form `submit` event happens
    // this will store the value of the <input> in our signal
    let on_submit = move |ev: SubmitEvent| {
        // stop the page from reloading!
        ev.prevent_default();

        // here, we'll extract the value from the input
        let value = input_element()
            // event handlers can only fire after the view
            // is mounted to the DOM, so the `NodeRef` will be `Some`
            .expect("<input> to exist")
            // `NodeRef` implements `Deref` for the DOM element type
            // this means we can call`HtmlInputElement::value()`
            // to get the current value of the input
            .value();
        set_name(value);
    };

    view! { cx,
        <form on:submit=on_submit>
            <input type="text"
                // here, we use the `value` *attribute* to set only
                // the initial value, letting the browser maintain
                // the state after that
                value=name

                // store a reference to this input in `input_element`
                node_ref=input_element
            />
            <input type="submit" value="Submit"/>
        </form>
        <p>"Name is: " {name}</p>
    }
}

fn main() {
    leptos::mount_to_body(|cx| view! { cx, <App/> })
}
