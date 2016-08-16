fn main() {
    
    let mut input = String::new();
    std::io::stdin().read_line( &mut input ).expect( "Error" );
    
    println!( "Hello, World.");
    println!( "{}", input.trim() );
}