use cfg_if::cfg_if;
pub mod app;
pub mod error_template;
pub mod fileserve;
pub mod todo;
/*
Seems odd that `mod todo` must be defined here, in lib.rs
to avoid the following error:
  error[E0432]: unresolved import `crate::todo`
  --> src/app.rs:1:12
    |
  1 | use crate::todo::*;
    |            ^^^^ could not find `todo` in the crate root

  error[E0425]: cannot find value `BusyButton` in this scope
    --> src/app.rs:10:8
    |
 10 |       <BusyButton/>
    |        ^^^^^^^^^^ not found in this scope
Defining `pub mod todo` in todo.rs or src/mod.rs does not work.
Neither does defining in main.rs, as in `modules` demo, work.
See: https://doc.rust-lang.org/stable/reference/items/modules.html#module-source-filenames
    */

cfg_if! { if #[cfg(feature = "hydrate")] {
    use leptos::*;
    use wasm_bindgen::prelude::wasm_bindgen;
    use crate::app::*;

    #[wasm_bindgen]
    pub fn hydrate() {
        // initializes logging using the `log` crate
        _ = console_log::init_with_level(log::Level::Info);
        console_error_panic_hook::set_once();

        leptos::mount_to_body(move |cx| {
            view! { cx, <App/> }
        });
    }
}}
