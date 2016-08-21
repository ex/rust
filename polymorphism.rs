use std::fmt;

struct Foo {
    x: i32
}

impl fmt::Display for Foo {
    fn fmt( &self, f: &mut fmt::Formatter ) -> fmt::Result {
        write!( f, "(x: {})", self.x )
    }
}

struct Bar {
    x: i32,
    y: i32
}

impl fmt::Display for Bar {
    fn fmt( &self, f: &mut fmt::Formatter ) -> fmt::Result {
        write!( f, "(x: {}, y: {})", self.x, self.y )
    }
}

fn print_me<T: fmt::Display>( obj: T ) {
    println!( "Value: {}", obj );
}

fn main() {
    let foo = Foo {x: 7};
    let bar = Bar {x: 5, y: 10};
    print_me( foo );
    print_me( bar );
}

