fn main() {
    let mut n = String::new();
    std::io::stdin().read_line( &mut n ).expect( "Error" );
    let mut n = n.trim().parse::<i32>().expect( "NaN" );
    
    let mut max = 0;
    let mut s = 0;    
    while n > 0 {
        if n % 2 == 1 {
            s += 1;
            if s > max {
                max = s;
            }
        } else {
            s = 0;
        }
        n = n / 2;
    }
    
    println!( "{}", max );
}
