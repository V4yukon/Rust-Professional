pub fn odd_fibnacci_sum(threshold: u32) -> u32 {
    if threshold < 1 {
        return 0;
    }
    
    let mut sum = 0;
    let mut a = 1; // First Fibonacci number
    let mut b = 1; // Second Fibonacci number
    
    // Process Fibonacci numbers until we reach the threshold
    while a <= threshold {
        // If current Fibonacci number is odd, add it to sum
        if a % 2 == 1 {
            sum += a;
        }
        
        // Generate next Fibonacci number
        let next = a + b;
        a = b;
        b = next;
    }
    
    sum
}
