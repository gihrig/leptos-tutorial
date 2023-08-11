/* 9.1 Routing - Defining Routes */

// Getting Started

// It’s easy to get started with the router.

// First things first, make sure you’ve added the leptos_router package
// to your dependencies.

// It’s important that the router is a separate package from leptos
// itself. This means that everything in the router can be defined in
// user-land code. If you want to create your own router, or use no
// router, you’re completely free to do that!

// And import the relevant types from the router, either with something
// like

/*
  use leptos_router::{
    Route,
    RouteProps,
    Router,
    RouterProps,
    Routes,
    RoutesProps
  };
*/

// or simply

/*
  use leptos_router::*;
*/

// Providing the <Router/>

// Routing behavior is provided by the <Router/> component.
// https://docs.rs/leptos_router/latest/leptos_router/fn.Router.html
// This should usually be somewhere near the root of your application.

// You shouldn’t try to use multiple <Router/>s in your app. Remember
// that the router drives global state: if you have multiple routers,
// which one decides what to do when the URL changes?

// Let’s start with a simple <App/> component using the router:

/*
  use leptos::*;
  use leptos_router::*;

  #[component]
  pub fn App(cx: Scope) -> impl IntoView {
    view! { cx,
      <Router>
        <nav>
          /* ... */
        </nav>
        <main>
          /* ... */
        </main>
      </Router>
    }
  }
*/

// Defining <Routes/>

// The <Routes/> component
// https://docs.rs/leptos_router/latest/leptos_router/fn.Routes.html
// is where you define all the routes to which a user can navigate in
// your application. Each possible route is defined by a
// <Route/> component.
// https://docs.rs/leptos_router/latest/leptos_router/fn.Route.html

// You should place the <Routes/> component at the location within your
// app where you want routes to be rendered. Everything outside <Routes/>
// will be present on every page, so you can leave things like a
// navigation bar or menu outside the <Routes/>.

/*
  use leptos::*;
  use leptos_router::*;

  #[component]
  pub fn App(cx: Scope) -> impl IntoView {
    view! { cx,
      <Router>
        <nav>
          /* ... */
        </nav>
        <main>
          // all our routes will appear inside <main>
          <Routes>
            /* ... */
          </Routes>
        </main>
      </Router>
    }
  }
*/

// Individual routes are defined by providing children to <Routes/>
// with the <Route/> component. <Route/> takes a path and a view. When
// the current location matches path, the view will be created and
// displayed.

// The path can include

// a static path (/users),
// dynamic, named parameters beginning with a colon (/:id),
// and/or a wildcard beginning with an asterisk (/user/*any)

// The view is a function that takes a Scope and returns a view.

/*
  <Routes>
    <Route path="/" view=Home/>
    <Route path="/users" view=Users/>
    <Route path="/users/:id" view=UserProfile/>
    <Route path="/`*any" view=NotFound/> <--- remove `
  </Routes>
*/

// view takes a Fn(Scope) -> impl IntoView. If a component has no props,
// it is a function that takes Scope and returns impl IntoView, so it can
// be passed directly into the view. In this case, view=Home is just a
// shorthand for |cx| view! { cx, <Home/> }.

// Now if you navigate to / or to /users you’ll get the home page or the
// <Users/>. If you go to /users/3 or /blahblah you’ll get a user profile
// or your 404 page (<NotFound/>). On every navigation, the router
// determines which <Route/> should be matched, and therefore what
// content should be displayed where the <Routes/> component is defined.

// Note that you can define your routes in any order. The router scores
// each route to see how good a match it is, rather than simply trying
// to match them top to bottom.

// Simple enough?

use leptos::*;

fn main() {
    leptos::mount_to_body(|cx| {
        view! { cx, <h1>"Routing - Defining Routes"</h1> }
    })
}
