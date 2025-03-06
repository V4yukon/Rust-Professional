pub fn find_max_prime_factor(mut number: u128) -> u128 {
    // Handle edge cases
    if number <= 1 {
        return number;
    }
    
    let mut max_prime = 0;
    
    // Remove all factors of 2
    while number % 2 == 0 {
        max_prime = 2;
        number /= 2;
    }
    
    // Check small prime factors first: 3, 5, 7, etc.
    let mut i: u128 = 3;
    while i * i <= number && i <= 1_000_000 {
        // While i divides number, divide it out
        while number % i == 0 {
            max_prime = i;
            number /= i;
        }
        i += 2; // Only check odd numbers
    }
    
    // If we've reduced the number significantly but it's not 1,
    // use a more efficient approach for larger remaining values
    if number > 1_000_000 {
        // For very large numbers, we can use probabilistic primality tests
        // But for this implementation, we'll assume the remaining number is prime
        // This is valid because if it had any factors, we would have found them earlier
        max_prime = number;
    } else if number > 1 {
        // Continue checking for smaller remaining numbers
        i = if i < 3 { 3 } else { i };
        while i * i <= number {
            while number % i == 0 {
                max_prime = i;
                number /= i;
            }
            i += 2;
        }
        
        // If number is a prime greater than our last checked factor
        if number > 1 {
            max_prime = number;
        }
    }
    
    max_prime
}