fn main() {
    let mut meal_cost = String::new();
    std::io::stdin().read_line( &mut meal_cost ).expect( "Error" );
    let meal_cost = meal_cost.trim().parse::<f64>().unwrap();
    
    let mut tip_percent = String::new();
    std::io::stdin().read_line( &mut tip_percent ).expect( "Error" );
    let tip_percent = tip_percent.trim().parse::<i32>().expect( "Invalid" );
    
    let mut tax_percent = String::new();
    std::io::stdin().read_line( &mut tax_percent ).expect( "Error" );
    let tax_percent = tax_percent.trim().parse::<i32>().expect( "Invalid" );
    
    let total_cost = ( meal_cost * ( 1.0 + 0.01 * tip_percent as f64 + 0.01 * tax_percent as f64 ) + 0.5 ) as i64;
    println!( "The total meal cost is {} dollars.", total_cost );
}