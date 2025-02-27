pub fn goldbach_conjecture() -> String {
    // 保持原有素数判断逻辑不变
    fn is_prime(n: u64) -> bool {
        if n <= 1 { return false; }
        if n <= 3 { return true; }
        if n % 2 == 0 || n % 3 == 0 { return false; }
        let (mut i, mut w) = (5, 2);
        while i * i <= n {
            if n % i == 0 { return false; }
            i += w;
            w = 6 - w;
        }
        true
    }

    // 核心逻辑优化点：平方数预计算
    let (mut n, mut results) = (9u64, vec![]);
    while results.len() < 2 {
        if !is_prime(n) {
            let mut found = false;
            let max_k = (n as f64).sqrt().floor() as u64;
            
            // 遍历平方数范围缩小到 sqrt(n/2)
            for k in 1..=max_k {
                let square = k * k;
                if let Some(p) = n.checked_sub(2 * square) {
                    if p >= 2 && is_prime(p) {
                        found = true;
                        break;
                    }
                }
            }
            
            if !found { results.push(n); }
        }
        n += 2; // 仅检查奇数
    }
    
    // 关键修改点：添加逗号分隔符
    format!("{},{}", results[0], results[1])
}
