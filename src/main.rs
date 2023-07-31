/* 3.7a Error Handling - Result<T, E> */

use leptos::*;

// In the last chapter, we saw that you can render Option<T>:
// in the None case, it will render nothing, and in the T case,
// it will render T (that is, if T implements IntoView). You
// can actually do something very similar with a Result<T, E>.
// In the Err(_) case, it will render nothing. In the Ok(T)
// case, it will render the T.

// Let’s start with a simple component to capture a number input.

#[component]
fn NumericInput(cx: Scope) -> impl IntoView {
    let (value, set_value) = create_signal(cx, Ok(0));

    // When input changes, try to parse a number from the input.
    let on_input = move |ev| set_value(event_target_value(&ev).parse::<i32>());

    view! {cx,
      <label>
        "Type a number (or not!)"
        <input on:input=on_input/>
        <p>
          "You entered "
          <strong>{value}</strong>
        </p>
      </label>
    }
}

fn main() {
    leptos::mount_to_body(|cx| view! { cx, <NumericInput/> })
}

// Every time you change the input, on_input will attempt to parse
// its value into a 32-bit integer (i32), and store it in our value
// signal, which is a Result<i32, _>. If you type the number 42,
// the UI will display

/*
  You entered 42
*/

// But if you type the string "foo", it will display

/*
  You entered
*/

// This is not great. It saves us using .unwrap_or_default()
// or something, but it would be much nicer if we could catch
// the error and do something with it.

// You can do that, with the <ErrorBoundary/> component.
