fn main() {

    let mut n = String::new();
    std::io::stdin().read_line( &mut n ).expect( "Error" );
    let n = n.trim().parse::<i32>().expect( "Invalid" );

    let m = if n % 2 != 0 {
        "Weird"
    } else if n <= 5 {
        "Not Weird"
    } else if n <= 20 {
        "Weird"
    } else {
        "Not Weird"
    };

    println!( "{}", m );
}