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

//! This is example of Borrowed str
//! This is simple function that takes a `&str` and prints it
pub fn prints_str(my_str: &str) {
    println!("{my_str}");
}

fn main() {
    // We define `String` variable called `my_string`
    // and pass this variable as a parameter to `prints_str`
    let my_string: String = String::from("I am a string");

    // Here `&my_string` is of type `&String`
    // and it is still work even though input to `prints_str` is of type `&str`
    // So rust will convert `&String` to `&str` when we pass it to `prints_str`
    // thanks to `Deref` trait
    prints_str(&my_string);

    // We know that you can’t return a reference from something that only lives inside a function
    // because it dies as soon as the function is over.
    // When the variable dies, you don’t want to have a reference pointing to where the data was.
    // That’s unsafe, so Rust doesn’t allow it.
    // But when using a `str` with a `'static` lifetime, the data never disappears.
    // So, you can return a reference to it!
    // In the following code, the first function will work, but the second will not

    // Here everything works we have string literal `"I live forever!"` which has a lifetime of `'static`
    // which means it lives forever
    fn works() -> &'static str {
        "I live forever!"
    }

    // Here we cannot return reference to temporary value
    // returns a reference to data owned by the current function
    // `String::from("Sorry, I only live inside the fn. Not 'static")` this is the data owned by the function
    // and when function is done this String does not exist anymore
    // therefore we cannot return a reference to it
    // fn does_not_work() -> &'static str {
    //     &String::from("Sorry, I only live inside the fn. Not 'static")
    // }
}