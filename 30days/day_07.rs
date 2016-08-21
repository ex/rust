fn main() {
    let mut n = String::new();
    std::io::stdin().read_line( &mut n ).expect( "Error" );
    let n = n.trim().parse::<usize>().expect( "NaN" );
    
    let mut v = String::new();
    std::io::stdin().read_line( &mut v ).expect( "Error" );    
    let v: Vec<&str> = v.trim().split( " " ).collect();
    
    let mut o = String::new();
    for k in 0 .. n {
        o.push_str( v[n - 1 - k] );
        if k < n - 1 {
            o.push_str( " " );
        }
    }
    println!( "{}", o );
}