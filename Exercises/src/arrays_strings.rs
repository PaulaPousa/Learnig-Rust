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
// There are three types of edits that can be performed on strings: insert a character, remove a character, or replace a character. 
// Given two strings, write a function to check if they are one edit away.
pub fn one_way(s1: String, s2: String) -> bool {

    if s1.eq(&s2) { return false; } // If they are the same string, none of them has been edited.
    if i32::abs(s1.len() as i32 - s2.len() as i32) > 1 { return false; } // I check if the length of both strings is equal or only 1 longer.

    let mut large = if s1.len() >= s2.len() { s1.chars().collect::<Vec<char>>() } else { s2.chars().collect::<Vec<char>>() };
    let mut small = if s1.len() < s2.len() { s1.chars().collect::<Vec<char>>() } else { s2.chars().collect::<Vec<char>>() };

    large.sort();
    small.sort();

    let equal = s1.len() == s2.len();
    let mut found = false;
    let mut cont = 0 as usize;

    for c in large {
        if c != small[cont] && !found { 
            found = true; 
            cont = if equal { cont + 1 } else { cont };
        }
        else if c != small[cont] { return false; }
        else { cont = cont + 1; }
    }
    true
}


//-------------------- STRING COMPRESSION --------------------


//-------------------- ROTATE MATRIX --------------------


//-------------------- ZERO MATRIX --------------------


//-------------------- STRING ROTATION --------------------