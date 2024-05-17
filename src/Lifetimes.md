# Lifetimes

- **Purpose**:
    - Lifetimes help the compiler understand:
        - When variables can be dropped
        - How long references should last

- **Usage**:
    - Typically, we don't need to specify lifetimes in our code.
    - Sometimes, the compiler requires us to explicitly indicate how long something should last.

## Types of `&str`

### String Literals (`&'static str`)

- Created with: `let my_str = "I am a &str";`
- Characteristics:
    - Last for the entire program duration.
    - Written directly into the binary.
    - Have a lifetime called `'static`.

### Borrowed `str`

- Regular `&str` form without a `'static` lifetime.
- Conversion:
    - Passing a reference to a `String` (a `&String`) is converted to a `&str` using the `Deref` trait (automatically).
    - Example: A `&String` can be passed to a function expecting a `&str`.


#### Example
This is example of Borrowed str
This is simple function that takes a `&str` and prints it
```rust
fn prints_str(my_str: &str) {
    println!("{my_str}");
}
let my_string: String = String::from("I am a string");
prints_str(&my_string);
```
Here, `&my_string` is of type `&String`, but Rust converts it to `&str` thanks to the `Deref` trait.

## Lifetime Safety
- References cannot live longer than the object they reference.
- Returning a reference to a local variable is unsafe because the variable is dropped when the function ends.

#### Example
- Working example with string literal:
```rust
fn works() -> &'static str {
   "I live forever!"
}
```
- Non-working example with temporary value:
```rust
fn does_not_work() -> &'static str {
    &String::from("Sorry, I only live inside the fn. Not 'static")
}
```
The `String` is owned by the function and disappears when the function ends. Therefore,
we cannot return reference `&String` from function.

## Lifetime Annotations
- Define how long a variable or reference lives.
- Rust usually handles lifetimes, but sometimes it needs extra help with references.
- IMPORTANT: References should not outlive the object they reference to avoid pointing to freed memory.

#### Example
- Example without Lifetime Specifier
```rust
fn returns_reference() -> &str {
    let my_string = String::from("I am a string");
    &my_string
}
```
Error: `my_string` only lives inside `returns_reference` function and cannot return referenced value (`&String`) after the function ends.

- Example with Static Lifetime
```rust
fn returns_str() -> &'static str {
    "I am a str"
}
```
This tells Rust the function will only return a string literal (`str` with lifetime specifier `'static`) that lives forever.

## Lifetime Annotations in Types

- **Creating a Struct with a Reference Field**:
    - Imagine we want to create a `User` struct with a `name` field of type `&str`.
```rust
#[derive(Debug)]
struct User {
    name: &str
}
```
- The compiler will give an error: `missing lifetime specifier`.
#### Why Does This Happen?
- Rust needs a lifetime for `&str` because `&str` is a reference.
- If the value that `name` points to is dropped, the reference could point to invalid data.
- This is unsafe, so Rust prevents it.

#### Using 'static Lifetime:
- You can make the field `&'static str` to avoid the error.
```rust
#[derive(Debug)]
struct User {
    name: &'static str
}
```
This compiles but limits the name to string literals only.

#### Adding a Lifetime Specifier
- To fix this, we give the `User` struct and `&str` a lifetime specifier.
- This means `User` will only accept a reference for `name` if it lives as long as `User`.
```rust
#[derive(Debug)]
struct User<'a> {
    name: &'a str
}
```
This means: The `User` struct has a lifetime `'a`, and name must live at least as long as `'a`.


## The anonymous lifetime
When we have a struct like `User` we can implement `Display` trait for it
```rust
impl fmt::Display for User {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "My name is {}.", self.name)
  }
}
```
input to formater is as follows
```rust
f: &mut fmt::Formatter<'_>
```
and it has something which is called `anonymous lifetime specifier` `<'_>`. 
It is an indicator that references are being used
```rust
struct User<'a> {
  name: &'a str
}
    
impl User<'_> {
  fn print_name(&self) {
    println!("{}", self.name);
  }
}
    
impl Display for User<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.name)
  }
}
    
let user = User { name: "John" };
user.print_name();
```
