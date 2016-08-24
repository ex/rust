use std::collections::HashMap;

fn main() {
    let mut n = String::new();
    std::io::stdin().read_line( &mut n ).expect( "Error" );
    let n = n.trim().parse::<usize>().expect( "NaN" );
    
    let mut map: HashMap<String, String> = HashMap::new();

    for _ in 0 .. n {
        let mut v = String::new();
        std::io::stdin().read_line( &mut v ).expect( "Error" );    
        v = v.trim().to_string();
        let v: Vec<&str> = v.split( " " ).collect();
        let k = v[0].to_string();
        let d = v[1].to_string();
        map.insert( k, d );
    }

    loop {
        let mut s = String::new();
        std::io::stdin().read_line( &mut s ).expect( "Error" );
        s = s.trim().to_string();
        if s.len() > 0 {
            match map.get( &mut s ) {
                Some( data ) => println!("{}={}", s, data ),
                None => println!( "Not found" )
            }
        } else {
            break;
        }
    }
}
