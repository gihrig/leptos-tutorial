/* 3.7b Error Handling - <ErrorBoundary/> */

use leptos::*;

// An <ErrorBoundary/> is a little like the <Show/> component
// we saw in the last chapter. If everything’s okay—which is
// to say, if everything is `Ok(_)` it renders its children. But
// if there’s an `Err(_)` rendered among those children, it will
// trigger the <ErrorBoundary/>’s fallback.

// Let’s add an <ErrorBoundary/> to this example.

#[component]
fn NumericInput(cx: Scope) -> impl IntoView {
    let (value, set_value) = create_signal(cx, Ok(0));

    // When input changes, try to parse a number from the input.
    let on_input = move |ev| set_value(event_target_value(&ev).parse::<i32>());

    view! {cx,
      <h1>"Error Handling"</h1>
      <label>
        "Type a number (or something that's not a number!) "
        <input on:input=on_input/>
        <ErrorBoundary
          // The fallback receives a signal containing an error
          fallback=|cx, errors| view! { cx,
            <div class="error">
              <p>"Not a number! Errors:"</p>
              // We can render a list of errors as strings, if we'd like
              <ul>
                {move || errors.get()
                  .into_iter()
                  .map(|(_, e)| view! { cx, <li>{e.to_string()}</li>})
                  .collect_view(cx)
                }
              </ul>
            </div>
          }
        >
        <p>
          "You entered " <strong>{value}</strong>
        </p>
        </ErrorBoundary>
      </label>
    }
}

fn main() {
    leptos::mount_to_body(|cx| view! { cx, <NumericInput/> })
}

// Now, if you type 42, value is Ok(42) and you’ll see

/*
  You entered 42
*/

// If you type "foo", value is Err(_) and the fallback
// will render. We’ve chosen to render the list of
// errors as a String, so you’ll see something like

/*
  Not a number! Errors:
  - cannot parse integer from empty string
*/

// If you fix the error, the error message will disappear
// and the content you’re wrapping in an <ErrorBoundary/>
// will appear again.
