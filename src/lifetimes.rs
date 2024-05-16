//! Lifetimes, used by the compiler to know when variables can be dropped
//! and how long references last. Usually, you don’t need to specify lifetimes in your code,
//! but sometimes the compiler needs a bit of help and will ask you to tell it
//! how long something should last.


//!  There is actually more than one type of `&str`
//! `String literals`
//! You make these when you write `let my_str = "I am a &str";`
//! They last for the whole program because they are written directly into the binary.
//! They have the type `&'static str`
//! The ' means its lifetime, and string literals have a lifetime called `static`
//!
//! `Borrowed str`
//! This is the regular `&str` form without a 'static lifetime.
//! If you have a `String` and pass a reference to it (a `&String`),
//! Rust will convert it to a `&str` when you need it.
//! This is thanks to a trait called `Deref`.
//! You can pass in a `&String` to a function that takes a `&str`.

//! This is simple function that takes a `&str` and prints it
pub fn prints_str(my_str: &str) {
    println!("{my_str}");
}