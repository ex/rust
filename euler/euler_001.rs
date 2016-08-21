// https://www.hackerrank.com/contests/projecteuler/challenges/euler001

fn main() {
    let mut t = String::new();
    std::io::stdin().read_line( &mut t ).expect( "Error" );
    let t = t.trim().parse::<usize>().expect( "Invalid" );
    
    let mut v: Vec<i32> = vec![];
    for _ in 0 .. t {
        let mut s = String::new();
        std::io::stdin().read_line( &mut s ).expect( "Error" );
        v.push( s.trim().parse::<i32>().expect( "Invalid" ) );
    }
    
    for n in v {
        let m3 = ( n - 1 ) as i64 / 3;
        let m5 = ( n - 1 ) as i64 / 5;
        let m15 = ( n - 1 ) as i64 / 15;
        println!( "{}", 3 * m3 * ( m3 + 1 ) / 2 + 5 * m5 * ( m5 + 1 ) / 2 
                                                - 15 * m15 * ( m15 + 1 ) / 2 );
    }
}
