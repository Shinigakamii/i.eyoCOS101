fn main() {
    // Constants
    const MILES_TO_KILOMETERS: f64 = 1.60934;
    
    // Given data
    let miles = 80.0;
    let hours = 2.0;
    
    // Calculate the speed in kilometers per hour
    let kilometers = miles * MILES_TO_KILOMETERS;
    let speed_kmph = kilometers / hours;
    
    // Display the result
    println!("The car travels at {:.2} kilometers per hour.", speed_kmph);
}