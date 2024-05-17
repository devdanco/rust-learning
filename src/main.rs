use std::fmt::Display;

fn main() {
    println!("Lifetimes");   
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
            write!(f, "My name is {}", self.name)
        }
    }
    
    let user = User { name: "John" };
    user.print_name();
    println!("{}", user);
}