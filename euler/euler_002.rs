// https://www.hackerrank.com/contests/projecteuler/challenges/euler002

fn main() {
    let mut t = String::new();
    std::io::stdin().read_line( &mut t ).expect( "Error" );
    let t = t.trim().parse::<usize>().expect( "Invalid" );
    
    let mut v: Vec<i64> = vec![];
    for _ in 0 .. t {
        let mut s = String::new();
        std::io::stdin().read_line( &mut s ).expect( "Error" );
        v.push( s.trim().parse::<i64>().expect( "Invalid" ) );
    }
    
    for n in v {

        let mut a: i64 = 0;
        let mut b: i64 = 1;
        let mut t: i64 = 0;

        while b < n {
            if b % 2 == 0 {
                t += b;
            }
            let t = a;
            a = b;
            b += t;
        }

        println!( "{}", t );
    }
}
