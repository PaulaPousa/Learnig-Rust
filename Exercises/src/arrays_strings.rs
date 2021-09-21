use std::collections::HashMap; 

// -------------------- IS UNIQUE --------------------
// Implement an algorithm to determine if a string has all unique characters.

pub fn is_unique_array(word: String) -> bool {    
    if word.len() > 128 {        
        return false;
    
    } else {
        let mut characters = [false; 128]; 

        for c in word.chars() {
            let index = c as u32;

            if characters[index as usize] { return false; }
            characters[index as usize] = true;
        }
    }
    return true;
}

pub fn is_unique_hashmap(word: String) -> bool {   
    if word.len() > 128 { 
        return false;
    
    } else {
        let mut characters = HashMap::new();

        for c in word.chars() {        
            let count = characters.entry(c).or_insert(0);
            *count += 1;    

            if *count == 2 { return false; } 
        }
    }
    return true; 
}


// -------------------- CHECK PERMUTATION --------------------
// Given two strings, write a method to decide if one is a permutation of the other.

pub fn check_permutation(s1: String, s2: String) -> bool {

    if s1.len() != s2.len() {
        false

    } else {
        let mut charvec = s1.chars().collect::<Vec<char>>();
        charvec.sort();
        let sort_s1 = charvec.into_iter().collect::<String>();

        charvec = s2.chars().collect::<Vec<char>>();
        charvec.sort();
        let sort_s2 = charvec.into_iter().collect::<String>();

        sort_s1.eq(&sort_s2)
    }
}


// -------------------- URLify --------------------
// Write a method to replace clall spaces in a string with '%20'.

pub fn urlify(word: String) -> String {
    word.replace(" ", "%20")
}


// -------------------- PALINDROME PERMUTATION --------------------
// Given a string, write a function to check if it is permutation of a palindrome.
// (A palindrome is a word or phrase that is the same forwards and backwards.)

pub fn palindrome_permutation(word: String) -> bool {
    let mut found_odd: bool = false;
    let mut characters = HashMap::new();

    for c in word.chars().flat_map(char::to_lowercase) { 
        if c != ' ' {
            let count = characters.entry(c).or_insert(0);
            *count += 1; 
        }
    }

    for (_character, &number) in characters.iter() {
        if number % 2 != 0 && !found_odd { found_odd = true; }
        else if number % 2 != 0 { return false; }
    }
    true
}


//-------------------- ONE WAY --------------------


//-------------------- STRING COMPRESSION --------------------


//-------------------- ROTATE MATRIX --------------------


//-------------------- ZERO MATRIX --------------------


//-------------------- STRING ROTATION --------------------