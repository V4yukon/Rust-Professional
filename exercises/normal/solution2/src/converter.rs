pub fn convert_base(num_str: &str, to_base: u32) -> String {
    let (value, base_from) = match parse_input(num_str) {
        Ok((v, b)) => (v, b),
        Err(_) => return String::new(),
    };

    if !(2..=16).contains(&base_from) || !(2..=16).contains(&to_base) {
        return String::new();
    }


    let decimal = match to_decimal(value, base_from) {
        Ok(d) => d,
        Err(_) => return String::new(),
    };


    from_decimal(decimal, to_base)

}
fn parse_input(s: &str) -> Result<(&str, u32), &str> {
    let parts: Vec<&str> = s.split(')').collect();
    if parts.len() != 2 {
        return Err("Invalid format");
    }
    let (value_part, base_part) = parts[0].split_once('(').ok_or("Invalid format")?;
    let base = base_part.parse().map_err(|_| "Invalid base")?;
    
    Ok((value_part, base))
}


fn to_decimal(s: &str, base: u32) -> Result<u64, &str> {
    let mut result = 0;
    for c in s.chars() {
        let digit = c.to_uppercase()
            .next()
            .and_then(|c| c.to_digit(base))
            .ok_or("Invalid digit")? as u64;
        
        result = result * base as u64 + digit;
    }
    Ok(result)
}


fn from_decimal(mut n: u64, base: u32) -> String {
    if n == 0 {
        return "0".to_string();
    }

    let mut digits = Vec::new();
    while n > 0 {
        let rem = (n % base as u64) as u8;
        digits.push(match rem {
            0..=9 => (b'0' + rem as u8) as char,
            10..=15 => (b'a' + rem - 10) as char, 
            _ => unreachable!(),
        });
        n /= base as u64;
    }

    digits.iter().rev().collect()
}
