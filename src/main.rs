/* 12.3.7 Server Side Rendering - Blocking Resources */

// ------------------
// Blocking Resources
// ------------------

// Any Leptos versions later than 0.2.5 (i.e., git main and 0.3.x or
// later) introduce a new resource primitive with
// `create_blocking_resource`. A blocking resource still loads
// asynchronously like any other async/.await in Rust; it doesn’t
// block a server thread or anything. Instead, reading from a blocking
// resource under a <Suspense/> blocks the HTML stream from returning
// anything, including its initial synchronous shell, until that
// <Suspense/> has resolved.

// Now from a performance perspective, this is not ideal. None of the
// synchronous shell for your page will load until that resource is
// ready. However, rendering nothing means that you can do things like
// set the <title> or <meta> tags in your <head> in actual HTML. This
// sounds a lot like async rendering, but there’s one big difference:
// if you have multiple <Suspense/> sections, you can block on one of
// them but still render a placeholder and then stream in the other.

// For example, think about a blog post. For SEO and for social sharing,
// I definitely want my blog post’s title and metadata in the initial
// HTML <head>. But I really don’t care whether comments have loaded
// yet or not; I’d like to load those as lazily as possible.

// With blocking resources, I can do something like this:

/*
  #[component]
  pub fn BlogPost(cx: Scope) -> impl IntoView {
    let post_data = create_blocking_resource(cx /* load blog post */);
    let comment_data = create_resource(cx /* load blog post */);
    view! { cx,
      <Suspense fallback=|| ()>
      {move || {
        post_data.with(cx, |data| {
          view! { cx,
            <Title text=data.title/>
            <Meta name="description" content=data.excerpt/>
            <article>
            /* render the post content */
            </article>
          }
        })
      }}
      </Suspense>
      <Suspense fallback=|| "Loading comments...">
      /* render comment data here */
      </Suspense>
    }
  }
*/

// The first <Suspense/>, with the body of the blog post, will block my
// HTML stream, because it reads from a blocking resource. The second
// <Suspense/>, with the comments, will not block the stream. Blocking
// resources gave me exactly the power and granularity I needed to
// optimize my page for SEO and user experience.

use leptos::*;
pub fn main() {
    mount_to_body(|cx| {
        view! { cx, <h1>"Server Side Rendering - Blocking Resources"</h1> }
    });
}
