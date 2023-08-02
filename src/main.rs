/* 4.1 Reactivity - Working with Signals */
use leptos::*;

// Reactivity

// Leptos is built on top of a fine-grained reactive system, designed
// to run expensive side effects (like rendering something in a browser,
// or making a network request) as infrequently as possible in response
// to change, reactive values.

// So far we’ve seen signals in action. These chapters will go into a
// bit more depth, and look at effects, which are the other half of the
// story.

// ------------------------------------------------------------------

// Working with Signals

// There are four basic signal operations:

// 1. .get() clones the current value of the signal and tracks any future
//    changes to the value reactively.
// 2. .with() takes a function, which receives the current value of the
//    signal by reference (&T), and tracks any future changes.
// 3. .set() replaces the current value of the signal and notifies any
//    subscribers that they need to update.
// 4. .update() takes a function, which receives a mutable reference to
//    the current value of the signal (&mut T), and notifies any
//    subscribers that they need to update. (.update() doesn’t return
//    the value returned by the closure, but you can use .try_update()
//    if you need to. For example, if you’re removing an item from a
//    Vec<_> and want the removed item.)

// Calling a ReadSignal as a function is syntax sugar for .get().
// Calling a WriteSignal as a function is syntax sugar for .set(). So:

/*
  let (count, set_count) = create_signal(cx, 0);
  set_count(1);
  log!(count());
*/

// is the same as:

/*
  let (count, set_count) = create_signal(cx, 0);
  set_count.set(1);
  log!(count.get());
*/

// You might notice that .get() and .set() can be implemented in terms
// of .with() and .update(). In other words, count.get() is identical
// with count.with(|n| n.clone()), and count.set(1) is implemented by
// doing count.update(|n| *n = 1).

// But of course, .get() and .set() (or the plain function-call forms!)
// are much nicer syntax.

// However, there are some very good use cases for .with() and .update().

// For example, consider a signal that holds a Vec<String>.

/*
  let (names, set_names) = create_signal(cx, Vec::new());
  if names().is_empty() {
      set_names(vec!["Alice".to_string()]);
  }
*/

// In terms of logic, this is simple enough, but it’s hiding some
// significant inefficiencies. Remember that names().is_empty() is sugar
// for names.get().is_empty(), which clones the value
// (it’s names.with(|n| n.clone()).is_empty()). This means we clone the
// whole Vec<String>, run is_empty(), and then immediately throw away the
// clone.

// Likewise, set_names replaces the value with a whole new Vec<_>. This
// is fine, but we might as well just mutate the original Vec<_> in place.

/*
  let (names, set_names) = create_signal(cx, Vec::new());
  if names.with(|names| names.is_empty()) {
      set_names.update(|names| names.push("Alice".to_string()));
  }
*/

// Now our function simply takes names by reference to run is_empty(),
// avoiding that clone.

// And if you have Clippy on, or if you have sharp eyes, you may notice
// we can make this even neater:

/*
  if names.with(Vec::is_empty) {
      // ...
  }
*/

// After all, .with() simply takes a function that takes the value by
// reference. Since Vec::is_empty takes &self, we can pass it in directly
// and avoid the unnecessary closure.

// ------------------------------------------------------------------

// Making signals depend on each other

// Often people ask about situations in which some signal needs to
// change based on some other signal’s value. There are three good ways
// to do this, and one that’s less than ideal but okay under controlled
// circumstances.

// Good Options

// 1. `B` is a function of `A`. Create a signal for `A` and a derived
//    signal or memo for `B`.

/*
  let (count, set_count) = create_signal(cx, 1);
  let derived_signal_double_count = move || count() * 2;
  let memoized_double_count = create_memo(cx, move |_| count() * 2);
*/

//    For guidance on whether to use a derived signal or a memo, see the
//    docs for create_memo

// 2. `C` is a function of `A` and some other thing `B`. Create signals
//    for `A` and `B` and a derived signal or memo for `C`.

/*
  let (first_name, set_first_name) = create_signal(cx, "Bridget".to_string());
  let (last_name, set_last_name) = create_signal(cx, "Jones".to_string());
  let full_name = move || format!("{} {}", first_name(), last_name());
*/

// 3. `A` and `B` are independent signals, but sometimes updated at the
//    same time. When you make the call to update `A`, make a separate call
//    to update B.

/*
  let (age, set_age) = create_signal(cx, 32);
  let (favorite_number, set_favorite_number) = create_signal(cx, 42);
  // use this to handle a click on a `Clear` button
  let clear_handler = move |_| {
    set_age(0);
    set_favorite_number(0);
  };
*/

// If you really must...

// 4. Create an effect to write to `B` whenever `A` changes. This is
//    officially discouraged, for several reasons:
//  a) It will always be less efficient, as it means every time `A`
//    updates you do two full trips through the reactive process.
//    (You set `A`, which causes the effect to run, as well as any other
//    effects that depend on `A`. Then you set `B`, which causes any
//    effects that depend on `B` to run.)
//  b) It increases your chances of accidentally creating things like
//    infinite loops or over-re-running effects. This is the kind of
//    ping-ponging, reactive spaghetti code that was common in the early
//    2010s and that we try to avoid with things like read-write
//    segregation and discouraging writing to signals from effects.

// In most situations, it’s best to rewrite things such that there’s a
// clear, top-down data flow based on derived signals or memos. But this
// isn’t the end of the world.

// I’m intentionally not providing an example here. Read the create_effect
// docs to figure out how this would work.

fn main() {
    leptos::mount_to_body(|cx| {
        view! { cx,
        <h1>{"Reactivity - Working with Signals"}</h1>}
    })
}
