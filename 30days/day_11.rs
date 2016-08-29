fn main() {

    let mut m: Vec<Vec<i32>> = Vec::new();

    for _ in 0 .. 6 {
        let mut line = String::new();
        std::io::stdin().read_line( &mut line ).expect( "Error" );
        let v: Vec<i32> = line.split_whitespace()
                              .map( |s| s.parse().unwrap() ).collect();
        m.push( v );
    }

    let mut max = i32::min_value();

    for y in 0 .. m.len() - 2 {
        for x in 0 .. 4 {        
            let t = m[y][x] + m[y][x + 1] + m[y][x + 2] + m[y + 1][x + 1]
                            + m[y + 2][x] + m[y + 2][x + 1] + m[y + 2][x + 2];
            if t > max {
                max = t;
            }
        }
    }

    println!( "{}", max );
}