pub fn dp_rec_mc(amount: u32) -> u32 {
    // Available bill denominations
    let coins = [1, 2, 5, 10, 20, 30, 50, 100];
    
    // Create a DP array to store minimum coins needed for each amount
    // Initialize with a value larger than any possible solution
    let mut dp = vec![u32::MAX - 1; (amount + 1) as usize];
    
    // Base case: 0 amount needs 0 coins
    dp[0] = 0;
    
    // Fill the dp array bottom-up
    for i in 1..=amount as usize {
        // Try each coin denomination
        for &coin in coins.iter() {
            if coin as usize <= i {
                // If using this coin results in fewer total coins, update dp[i]
                dp[i] = dp[i].min(dp[i - coin as usize] + 1);
            }
        }
    }
    
    dp[amount as usize]
}
