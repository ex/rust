fn factorial( n: i32 ) -> i64 {
    if n < 2 { 1 } else { n as i64 * factorial( n - 1 ) }
}

fn main() {
    let mut n = String::new();
    std::io::stdin().read_line( &mut n ).expect( "Error" );
    let n = n.trim().parse::<i32>().expect( "NaN" );
    
    println!( "{}", factorial( n ) );
}
