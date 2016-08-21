fn main() {
    let mut n = String::new();
    std::io::stdin().read_line( &mut n ).expect( "Error" );
    let n = n.trim().parse::<i32>().expect( "Invalid" );
    
    for k in 1 .. 11 {
        println!( "{} x {} = {}", n, k, n * k );        
    }
}
