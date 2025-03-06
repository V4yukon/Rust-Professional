use std::{
    collections::{BTreeMap, HashMap},
    fs::File,
    io::{BufRead, BufReader},
};

/// Parses a JSON file containing district information into a structured format
/// 
/// The function reads a JSON file line by line and organizes it into a nested structure:
/// - The outer BTreeMap uses batch numbers as keys
/// - Each batch contains a HashMap of cities and their connections
///
/// @param filename: Path to the JSON file to parse
/// @return A structured representation of the district data
pub fn parse_file(filename: &str) -> BTreeMap<String, HashMap<String, Vec<String>>> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut data = BTreeMap::new(); // Main data structure to store all batches
    let mut main_key = String::new(); // Current batch key being processed

    // Process the file line by line
    for line in reader.lines() {
        let line = line.unwrap().trim().to_string();
        // Skip empty lines and opening braces
        if line.is_empty() || line.starts_with('{') {
            continue;
        }

        // Check if this line defines a new batch (e.g. "1": {)
        if line.contains(": {") {
            let (key, _) = line.split_once(':').unwrap();
            main_key = key.trim().trim_matches('"').to_string(); // Extract the batch number
            data.insert(main_key.clone(), HashMap::new()); // Initialize an empty HashMap for this batch
            continue;
        }
        
        // Skip closing braces
        if line.starts_with('}') {
            continue;
        }
        
        // Parse city and its connections (e.g. "city1": ["city2", "city3"])
        let parts: Vec<&str> = line.split(':').collect();
        if parts.len() != 2 {
            continue; // Skip malformed lines
        }

        // Extract the city name
        let key = parts[0].trim().trim_matches('"');
        
        // Extract the connected cities
        let values_str = parts[1]
            .trim()
            .trim_matches(',')
            .trim_matches('[')
            .trim_matches(']')
            .trim_matches('"');
            
        // Parse the list of connected cities
        let values: Vec<String> = if values_str.is_empty() {
            Vec::new() // Empty list if no connections
        } else {
            values_str
                .split(',')
                .map(|v| v.trim().trim_matches('"').to_string())
                .collect()
        };

        // Add the city and its connections to the current batch
        let mm: &mut HashMap<String, Vec<String>> = data.get_mut(&main_key).unwrap();
        if mm.contains_key(key) {
            // If the city already exists, extend its connections
            if let Some(map) = mm.get_mut(key) {
                map.extend(values);
            }
        } else {
            // Otherwise, add a new city with its connections
            mm.insert(key.to_string(), values);
        }
    }
    data
}

/// Counts the number of provinces in each batch of district data
/// 
/// A province is defined as a connected group of cities. Two cities are in the same
/// province if they are directly connected or connected through other cities.
/// 
/// @return A comma-separated string of province counts for each batch
pub fn count_provinces() -> String {
    let mut result = Vec::new(); // Store the count of provinces for each batch
    let data = parse_file("district.json"); // Parse the district data
    
    // Process each batch in the data
    for item in data {
        // HashMap to store connected components (provinces)
        let mut list: HashMap<String, Vec<String>> = HashMap::new();
        
        // Process each city and its connections in the current batch
        for (sub_item_key, mut sub_item_values) in item.1 {
            // Try to get the existing vector for this city
            let tmp_vec = match list.get_mut(&sub_item_key) {
                Some(v) => v,
                None => &mut Vec::new(),
            };
            
            // If the city already exists in a province, extend its connections
            if tmp_vec.contains(&sub_item_key) {
                tmp_vec.extend(sub_item_values);
            } else {
                // Check if this city or any of its connections are already in existing provinces
                let mut found_on_keys = Vec::new();
                for (list_key, list_values) in list.iter_mut() {
                    // Check if the current city is in any existing province
                    if list_values.contains(&sub_item_key) {
                        found_on_keys.push(list_key.clone());
                    } else {
                        // Check if any of the city's connections are in an existing province
                        for kk in sub_item_values.iter() {
                            if list_values.contains(kk) {
                                found_on_keys.push(list_key.clone());
                                break;
                            }
                        }
                    }
                }

                // If no existing provinces contain this city or its connections,
                // create a new province
                if list.is_empty() || found_on_keys.is_empty() {
                    sub_item_values.push(sub_item_key.to_string());
                    list.insert(sub_item_key, sub_item_values);
                    continue;
                }

                // Merge this city with all provinces it connects to
                let mut tmp_v = vec![];
                tmp_v.push(sub_item_key.to_string());
                tmp_v.extend(sub_item_values);
                
                // Merge all connected provinces
                for key in found_on_keys {
                    let t = list.remove(&key).unwrap();
                    tmp_v.extend(t);
                }
                
                // Create a new merged province
                list.insert(tmp_v[0].clone(), tmp_v);
            }
        }
        
        // The number of provinces is the number of entries in the list HashMap
        result.push(format!("{}", list.len()));
    }
    
    // Join all province counts with commas
    result.join(",")
}
