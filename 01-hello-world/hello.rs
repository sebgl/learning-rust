use std::fmt; // import fmt package for fmt::Display redefinition

/*
That's a
block comment
*/

// and a line comment

fn main() {
    // print hello
    // by using the println macro (hence the '!')
    // defined in std::fmt
    println!("Hello World!");

    // string formatting
    let name = "seb";
    println!("Hello {}", name);
    // with positional arguments
    println!("Hello {0} {1}", name, "what's up?");
    // with named arguments
    println!("Hello {name} {extra}", name=name, extra="what's up?");

    // print in debug mode instead of display mode
    println!("debug {:?}", name);

    // instanciate a point from struct Point defined below
    let point = Point{x: 1.0, y: 2.5};
    // print point in debug mode
    println!("debug point: {:?}", point);
    println!("display point: {}", point)
}

// declare a struct with automatic debug print derived
// it makes it possible to print the struct in debug mode ({:?})
// but not in display mode
#[derive(Debug)]
struct Point {
    x: f64,
    y: f64
}

// implement the trait fmt::Display for Point struct
impl fmt::Display for Point {
    // one function to implement
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // use write macro into f stream
        write!(f, "(x: {}, y: {})", self.x, self.y)
    }
}