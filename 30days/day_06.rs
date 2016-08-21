fn main() {
    let mut n = String::new();
    std::io::stdin().read_line( &mut n ).expect( "Error" );
    let n = n.trim().parse::<usize>().expect( "Invalid" );
    
    let mut v: Vec<String> = vec![];

    for _ in 0 .. n {
        let mut s = String::new();
        std::io::stdin().read_line( &mut s ).expect( "Error" );
        v.push( s.trim().to_string() );
    }
    
    for k in 0 .. n {
        let mut a = String::new();
        let mut b = String::new();        
        for ( k, c ) in v[k].chars().enumerate() {
            if k % 2 == 0 {
                a.push( c );
            } else {
                b.push( c );
            }
        }
        println!( "{} {}", a, b )
    }
}