/*
    Anagram Check
    Given two strings, check if they are anagrams of each other. 
    Anagrams are words or phrases formed by rearranging the letters of another, 
    using all the original letters exactly once. 
    The strings may contain spaces or punctuation, but you need to ignore them while checking.

    You need to implement the function `are_anagrams(s1: String, s2: String) -> bool`.
    The function should return `true` if the two strings are anagrams, and `false` otherwise.

    Hint: Consider normalizing the strings by removing non-alphabetical characters and converting to lowercase before checking.
*/

use std::fmt::{self, Display, Formatter};
use std::collections::HashMap;

pub fn are_anagrams(s1: String, s2: String) -> bool {
        let s1 = s1.replace(" ", "").to_lowercase();
        let s2 = s2.replace(" ", "").to_lowercase();
    
        if s1.len() != s2.len() {
            return false;
        }

        let mut char_count = HashMap::new();
    
        for char in s1.chars() {
            *char_count.entry(char).or_insert(0) += 1;
        }
    
        for char in s2.chars() {
            match char_count.get_mut(&char) {
                Some(count) => {
                    *count -= 1;
                    if *count < 0 {
                        return false; // More occurrences in s2 than in s1
                    }
                },
                None => return false, // char not found in s1
            }
        }
    
        char_count.values().all(|&count| count == 0)
    // TODO: Implement the logic to check if two strings are anagrams
    // Placeholder return value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_anagram_1() {
        let s1 = "listen".to_string();
        let s2 = "silent".to_string();
        let result = are_anagrams(s1, s2);
        println!("Are anagrams: {}", result);
        assert_eq!(result, true);
    }

    #[test]
    fn test_anagram_2() {
        let s1 = "evil".to_string();
        let s2 = "vile".to_string();
        let result = are_anagrams(s1, s2);
        println!("Are anagrams: {}", result);
        assert_eq!(result, true);
    }

    #[test]
    fn test_anagram_3() {
        let s1 = "hello".to_string();
        let s2 = "world".to_string();
        let result = are_anagrams(s1, s2);
        println!("Are anagrams: {}", result);
        assert_eq!(result, false);
    }

    #[test]
    fn test_anagram_4() {
        let s1 = "Clint Eastwood".to_string();
        let s2 = "Old West Action".to_string();
        let result = are_anagrams(s1, s2);
        println!("Are anagrams: {}", result);
        assert_eq!(result, true);
    }

    #[test]
    fn test_anagram_5() {
        let s1 = "Astronomer".to_string();
        let s2 = "Moon starer".to_string();
        let result = are_anagrams(s1, s2);
        println!("Are anagrams: {}", result);
        assert_eq!(result, true);
    }
}
