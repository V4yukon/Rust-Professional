pub fn new_birthday_probability(n: u32) -> f64 {
    if n < 2 {
        panic!("Input must be at least 2");
    }

    // Calculate probability that all birthdays are different
    // Then subtract from 1 to get probability of at least one match
    let mut different_probability = 1.0;
    let days_in_year = 365.0;

    for i in 0..n {
        different_probability *= (days_in_year - i as f64) / days_in_year;
    }

    // Probability of at least one shared birthday
    let result = 1.0 - different_probability;
    
    // Round to 4 decimal places
    (result * 10000.0).round() / 10000.0
}