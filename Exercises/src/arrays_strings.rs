use std::collections::HashMap; 

// -------------------- IS UNIQUE --------------------
// Implement an algorithm to determine if a string has all unique characters.

pub fn is_unique_array(word: String) -> bool {
    let mut unique: bool = true;
    
    if word.len() > 128 {        
        unique = false;
    
    } else {
        let mut characters = [false; 128]; 

        for c in word.chars() {

            let index = c as u32;

            if characters[index as usize] {
                unique = false;
                break;
            }
            characters[index as usize] = true;
        }
    }
    unique
}

pub fn is_unique_hashmap(word: String) -> bool {
    let mut unique: bool = true;
    
    if word.len() > 128 { 
        unique = false;
    
    } else {
        let mut characters = HashMap::new();

        for c in word.chars() {        
            let count = characters.entry(c).or_insert(0);
            *count += 1;    

            if *count == 2 {
                unique = false;
                break;
            }
        }
    }
    unique 
}



