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


// -------------------- CHECK PERMUTATION --------------------
// Given two strings, write a method to decide if one is a permutation of the other.

pub fn check_permutation(s1: String, s2: String) -> bool {
    let mut res: bool = true;

    if s1.len() != s2.len() {
        res = false;

    } else {
        let mut charvec = s1.chars().collect::<Vec<char>>();
        charvec.sort();
        let sort_s1 = charvec.into_iter().collect::<String>();

        charvec = s2.chars().collect::<Vec<char>>();
        charvec.sort();
        let sort_s2 = charvec.into_iter().collect::<String>();

        res = sort_s1.eq(&sort_s2);
    }
    res
}


// -------------------- URLify --------------------
// Write a method to replace all spaces in a string with '%20'.

pub fn urlify (word: String) -> String {
    word.replace(" ", "%20")
}