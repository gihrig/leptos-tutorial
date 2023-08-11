/* 9.5 Routing - The <Form/> Component */

// Links and forms sometimes seem completely unrelated. But, in fact,
// they work in very similar ways.

// In plain HTML, there are three ways to navigate to another page:

// 1. An <a> element that links to another page: Navigates to the URL
//    in its href attribute with the GET HTTP method.
// 2. A <form method="GET">: Navigates to the URL in its action attribute
//    with the GET HTTP method and the form data from its inputs encoded
//    in the URL query string.
// 3. A <form method="POST">: Navigates to the URL in its action
//    attribute with the POST HTTP method and the form data from its
//    inputs encoded in the body of the request.

// Since we have a client-side router, we can do client-side link
// navigations without reloading the page, i.e., without a full
// round-trip to the server and back. It makes sense that we can do
// client-side form navigations in the same way.

// The router provides a <Form> component,
// https://docs.rs/leptos_router/latest/leptos_router/fn.Form.html
// which works like the HTML <form> element, but uses client-side
// navigations instead of full page reloads. <Form/> works with both
// GET and POST requests.
// With method="GET", it will navigate to the URL encoded in the form
// data.
// With method="POST" it will make a POST request and handle the server’s
// response.

// <Form/> provides the basis for some components like <ActionForm/>
// and <MultiActionForm/> that we’ll see in later chapters. But it also
// enables some powerful patterns of its own.

// For example, imagine that you want to create a search field that
// updates search results in real time as the user searches, without a
// page reload, but that also stores the search in the URL so a user can
// copy and paste it to share results with someone else.

// It turns out that the patterns we’ve learned so far make this easy to
// implement.

/*
  async fn fetch_results() {
      // some async function to fetch our search results
  }

  #[component]
  pub fn FormExample(cx: Scope) -> impl IntoView {
      // reactive access to URL query strings
      let query = use_query_map(cx);
      // search stored as ?q=
      let search = move || query().get("q").cloned().unwrap_or_default();
      // a resource driven by the search string
      let search_results = create_resource(cx, search, fetch_results);

      view! { cx,
          <Form method="GET" action="">
              <input type="search" name="search" value=search/>
              <input type="submit"/>
          </Form>
          <Transition fallback=move || ()>
              /* render search results */
          </Transition>
      }
  }
*/

// Whenever you click Submit, the <Form/> will “navigate” to ?q={search}.
// But because this navigation is done on the client side, there’s no
// page flicker or reload. The URL query string changes, which triggers
// search to update. Because search is the source signal for the
// `search_results` resource, this triggers search_results to reload its
// resource. The <Transition/> continues displaying the current search
// results until the new ones have loaded. When they are complete, it
// switches to displaying the new result.

// This is a great pattern. The data flow is extremely clear: all data
// flows from the URL to the resource into the UI. The current state of
// the application is stored in the URL, which means you can refresh the
// page or text the link to a friend and it will show exactly what you’re
// expecting. And once we introduce server rendering, this pattern will
// prove to be really fault-tolerant, too: because it uses a <form>
// element and URLs under the hood, it actually works really well without
// even loading your WASM on the client.

// We can actually take it a step further and do something kind of clever:

/*
  view! { cx,
      <Form method="GET" action="">
          <input type="search" name="search" value=search
              oninput="this.form.requestSubmit()"
          />
      </Form>
  }
*/

// You’ll notice that this version drops the Submit button. Instead, we
// add an oninput attribute to the input. Note that this is not `on:input`,
// which would listen for the input event and run some Rust code. Without
// the colon, `oninput` is the plain HTML attribute. So the string is
// actually a JavaScript string. `this.form` gives us the form the input
// is attached to. `requestSubmit()` fires the `submit` event on the
// <form>, which is caught by <Form/> just as if we had clicked a Submit
// button. Now the form will “navigate” on every keystroke or input to
// keep the URL (and therefore the search) perfectly in sync with the
// user’s input as they type.

// See Leptos Router example for more details.
// https://github.com/leptos-rs/leptos/tree/main/examples/router

// -----------------------------------------------------------------
// Router Example
// -----------------------------------------------------------------

use leptos::*;
use leptos_router::*;

#[component]
fn App(cx: Scope) -> impl IntoView {
    view! { cx,
        <Router>
            <h1>"Contact App"</h1>
            // this <nav> will show on every routes,
            // because it's outside the <Routes/>
            // note: we can just use normal <a> tags
            // and the router will use client-side navigation
            <nav>
                <h2>"Navigation"</h2>
                <a href="/">"Home"</a>
                <a href="/contacts">"Contacts"</a>
            </nav>
            <main>
                <Routes>
                    // / just has an un-nested "Home"
                    <Route path="/" view=|cx| view! { cx, <h3>"Home"</h3> }/>
                    // /contacts has nested routes
                    <Route
                        path="/contacts"
                        view=|cx| view! { cx, <ContactList/> }
                    >
                        // if no id specified, fall back
                        <Route path=":id" view=|cx| view! { cx, <ContactInfo/> }>
                            <Route
                                path=""
                                view=|cx| {
                                    view! { cx, <div class="tab">"(Contact Info)"</div> }
                                }
                            />

                            <Route
                                path="conversations"
                                view=|cx| {
                                    view! { cx, <div class="tab">"(Conversations)"</div> }
                                }
                            />

                        </Route>
                        // if no id specified, fall back
                        <Route
                            path=""
                            view=|cx| {
                                view! { cx,
                                    <div class="select-user">
                                        "Select a user to view contact info."
                                    </div>
                                }
                            }
                        />

                    </Route>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn ContactList(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="contact-list">
            // here's our contact list component itself
            <div class="contact-list-contacts">
                <h3>"Contacts"</h3>
                <A href="alice">"Alice"</A>
                <A href="bob">"Bob"</A>
                <A href="steve">"Steve"</A>
            </div>

            // <Outlet/> will show the nested child route.
            // we can position this outlet wherever we want
            // within the layout
            <Outlet/>
        </div>
    }
}

#[component]
fn ContactInfo(cx: Scope) -> impl IntoView {
    // we can access the :id param reactively with `use_params_map`
    let params = use_params_map(cx);
    let id = move || {
        params.with(|params| params.get("id").cloned().unwrap_or_default())
    };

    // imagine we're loading data from an API here
    let name = move || match id().as_str() {
        "alice" => "Alice",
        "bob" => "Bob",
        "steve" => "Steve",
        _ => "User not found.",
    };

    view! { cx,
        <div class="contact-info">
            <h4>{name}</h4>
            <div class="tabs">
                <A href="" exact=true>
                    "Contact Info"
                </A>
                <A href="conversations">"Conversations"</A>
            </div>

            // <Outlet/> here is the tabs that are nested
            // underneath the /contacts/:id route
            <Outlet/>
        </div>
    }
}

fn main() {
    leptos::mount_to_body(|cx| view! { cx, <App/> })
}
