// use std::collections::HashSet;
// pub fn new_count_distinct(input_str: &str) -> usize {
//     input_str.split(",")
//         .collection::<HashSet<_>>()
//         .len
// }

use std::collections::HashSet;

pub fn new_count_distinct(input_str: &str) -> usize {
    input_str
        .split(',')          // Split by comma (not ",")
        .filter(|s| !s.is_empty())  // Optional: filter out empty strings
        .collect::<HashSet<_>>()  // Collect into HashSet (fixed typo in "collect")
        .len()               // Call len() as a method (added parentheses)
}
