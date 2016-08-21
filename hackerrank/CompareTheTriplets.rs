// https://www.hackerrank.com/challenges/compare-the-triplets
fn main() {
    let mut s1 = String::new();
    std::io::stdin().read_line( &mut s1 ).expect( "Error" );
    s1 = s1.trim().to_string();
    let v1: Vec<&str> = s1.split( " " ).collect();

    let mut s2 = String::new();
    std::io::stdin().read_line( &mut s2 ).expect( "Error" );
    s2 = s2.trim().to_string();
    let v2: Vec<&str> = s2.split( " " ).collect();

    let mut a = 0;
    let mut b = 0;

    for k in 0 .. 3 {
        let va = v1[k].parse::<i32>().expect( "Invalid" );
        let vb = v2[k].parse::<i32>().expect( "Invalid" );
        if  va < vb {
            b += 1;
        } else if va > vb {
            a += 1;
        }
    }

    println!( "{} {}", a, b );
}