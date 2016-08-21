// https://www.hackerrank.com/challenges/camelcase
fn main() {
    let mut s = String::new();
    std::io::stdin().read_line( &mut s ).expect( "Error" );
    s = s.trim().to_string();
    
    let mut u = 0;
    for b in s.bytes() {
        if b < 97 {
            u += 1;
        }
    }
    println!( "{}", u + 1 );
}