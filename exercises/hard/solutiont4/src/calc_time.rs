/// Returns formatted time information based on a date string
/// The returned string contains: week number, weekday, day of year, remaining days in year,
/// days to spring, and days to next trading day
/// 
/// # Arguments
/// * `time` - A date string in the format "YYYY-MM-DD"
/// 
/// # Returns
/// A comma-separated string with calendar information
pub fn time_info(time: &str) -> String {
    // Parse the date string into year, month, and day components
    let (year, month, day) = parse_date(time);
    
    // Calculate the day of year (1-365)
    let day_of_year = day_of_year_2025(month, day);
    
    // Calculate the week number (1-52)
    // Special handling for first 5 days of the year
    let week_num = if day_of_year <= 5 {
        1
    } else {
        // Formula to calculate week number for the rest of the year
        (((day_of_year - 6) / 7 % 52) + 2) % 52
    };

    // Calculate weekday (1-7 where 1 is Monday, 7 is Sunday)
    // Based on 2025 starting on a Wednesday (day 3)
    let mut weekday = (3 + day_of_year - 1) % 7;
    
    // Calculate remaining days in the year
    let remaining_days = 365 - day_of_year;
    
    // Calculate days until spring (March 20, 2025 - day 79)
    // If already past March 20, calculate days to next spring in 2026
    let days_to_spring = if day_of_year <= 29 {
        29 - day_of_year  // Days until January 29
    } else {
        let days_left_2025 = 365 - day_of_year;
        let days_in_2026 = 31 + 17;  // January (31) + February 17
        days_left_2025 + days_in_2026
    };
    
    // Find the next trading day (excluding weekends and holidays)
    let (next_year, next_month, next_day) = next_trading_day(year, month, day);
    
    // Calculate days between current date and next trading day
    let current_total = total_days_2025_base(year, month, day);
    let next_total = total_days_2025_base(next_year, next_month, next_day);
    let days_to_next_trading = next_total - current_total - 1;
    
    // Convert weekday 0 to 7 (Sunday becomes 7 instead of 0)
    if weekday == 0 {
        weekday = 7;
    }
    
    // Format the return string with all calculated values
    let string_ret = format!(
        "{},{},{},{},{},{}",
        week_num, weekday, day_of_year, remaining_days, days_to_spring, days_to_next_trading
    );
    
    println!("{}", string_ret);
    return string_ret;
}

/// Parses a date string in format "YYYY-MM-DD" into year, month, and day components
///
/// # Arguments
/// * `s` - A date string in the format "YYYY-MM-DD"
///
/// # Returns
/// A tuple of (year, month, day) as numeric values
fn parse_date(s: &str) -> (i32, u32, u32) {
    let parts: Vec<&str> = s.split('-').collect();
    let year = parts[0].parse().unwrap();
    let month = parts[1].parse().unwrap();
    let day = parts[2].parse().unwrap();
    (year, month, day)
}

/// Calculates the day of year (1-365) for a given date in 2025
///
/// # Arguments
/// * `month` - Month (1-12)
/// * `day` - Day of month
///
/// # Returns
/// Day of year (1-365)
fn day_of_year_2025(month: u32, day: u32) -> u32 {
    // Array of days in each month (non-leap year)
    let months = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    let mut days = day;
    // Add days from previous months
    for m in 0..(month - 1) as usize {
        days += months[m];
    }
    days
}

/// Calculates the next calendar day
///
/// # Arguments
/// * `year` - Year
/// * `month` - Month (1-12)
/// * `day` - Day of month
///
/// # Returns
/// A tuple (year, month, day) representing the next calendar day
fn next_day(year: i32, month: u32, day: u32) -> (i32, u32, u32) {
    let months_days = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    let (mut new_year, mut new_month, mut new_day) = (year, month, day + 1);

    // Handle month/year transitions
    if new_day > months_days[(new_month - 1) as usize] {
        new_day = 1;
        new_month += 1;
        if new_month > 12 {
            new_month = 1;
            new_year += 1;
        }
    }
    (new_year, new_month, new_day)
}

/// Determines if a given date is a trading day (not a weekend or holiday)
///
/// # Arguments
/// * `year` - Year
/// * `month` - Month (1-12)
/// * `day` - Day of month
///
/// # Returns
/// Boolean indicating if the date is a trading day
fn is_trading_day(year: i32, month: u32, day: u32) -> bool {
    // Check for holidays:
    // - New Year's Day (January 1)
    // - Chinese New Year (Jan 28 - Feb 4, 2025)
    // - Labor Day holiday (May 1-5)
    // - National Day (October 1)
    let is_holiday = 
        (month == 1 && day == 1) ||                        
        (year == 2025 && ( (month == 1 && day >= 28) ||     
                            (month == 2 && day <= 4) )) ||
        (month == 5 && day <= 5) ||                        
        (month == 10 && day == 1);                         
    
    if is_holiday { return false; }
    
    // Check if it's a weekend (Saturday or Sunday)
    let weekday = compute_weekday(year, month, day);
    !(weekday == 0 || weekday == 6)  // 0 = Sunday, 6 = Saturday
}

/// Computes the weekday (0-6, where 0 is Sunday) for a given date
///
/// # Arguments
/// * `year` - Year
/// * `month` - Month (1-12)
/// * `day` - Day of month
///
/// # Returns
/// Weekday number (0-6, where 0 is Sunday)
fn compute_weekday(year: i32, month: u32, day: u32) -> u32 {
    // Calculate days since a reference point and convert to weekday
    let total = total_days_2025_base(year, month, day);
    (total as u32 + 2) % 7  // +2 offset to align with correct weekday
}

/// Calculates total days since the start of 2025
///
/// # Arguments
/// * `year` - Year
/// * `month` - Month (1-12)
/// * `day` - Day of month
///
/// # Returns
/// Total number of days since the start of 2025
fn total_days_2025_base(year: i32, month: u32, day: u32) -> i32 {
    let mut total = 0;
    // Add days for each year since 2025
    for y in 2025..year {
        total += if is_leap_year(y) { 366 } else { 365 };
    }
    // Add days within the current year
    total += day_of_year_in_year(year, month, day) as i32;
    total
}

/// Determines if a year is a leap year
///
/// # Arguments
/// * `year` - Year to check
///
/// # Returns
/// Boolean indicating if the year is a leap year
fn is_leap_year(year: i32) -> bool {
    // Leap year rules: divisible by 4, but not by 100 unless also by 400
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}

/// Calculates the day of year (1-365/366) for any year (accounting for leap years)
///
/// # Arguments
/// * `year` - Year
/// * `month` - Month (1-12)
/// * `day` - Day of month
///
/// # Returns
/// Day of year (1-365 or 1-366 for leap years)
fn day_of_year_in_year(year: i32, month: u32, day: u32) -> u32 {
    // Select the appropriate month lengths based on leap year
    let months_days = if is_leap_year(year) {
        [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]  // Leap year
    } else {
        [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]  // Non-leap year
    };
    
    let mut days = day;
    // Add days from previous months
    for m in 0..(month - 1) as usize {
        days += months_days[m];
    }
    days
}

/// Finds the next trading day after a given date
///
/// # Arguments
/// * `start_year` - Starting year
/// * `start_month` - Starting month (1-12)
/// * `start_day` - Starting day of month
///
/// # Returns
/// A tuple (year, month, day) representing the next trading day
fn next_trading_day(start_year: i32, start_month: u32, start_day: u32) -> (i32, u32, u32) {
    let (mut y, mut m, mut d) = (start_year, start_month, start_day);
    loop {
        // Move to the next day
        (y, m, d) = next_day(y, m, d);
        // Check if it's a trading day
        if is_trading_day(y, m, d) {
            return (y, m, d);
        }
    }
}
