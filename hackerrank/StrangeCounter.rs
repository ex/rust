// https://www.hackerrank.com/contests/hourrank-11/challenges/strange-code
fn main() {
    let mut t = String::new();
    std::io::stdin().read_line( &mut t ).expect( "Error" );
    let t = t.trim().parse::<i64>().expect( "Error" );
    
    let mut p: i64 = 1;
    let mut m: i64 = 0;    
    
    while t > m {
        m = 3 * ( 2 * p  - 1 );
        p *= 2;
    }
    println!( "{}", m - t + 1 );
}