extern crate exercises;

use exercises::arrays_strings as strings;
use exercises::sorting_searching as sort;

fn main() {
    // ---------- ARRAYS_STRINGS FUNCTIONS CALLS ---------- 
    println!("-- ARRAYS_STRINGS FUNCTIONS CALLS --");

    println!("is_unique_array: {}", strings::is_unique_array(String::from("string")));
    println!("is_unique_hashmap: {}", strings::is_unique_hashmap(String::from("string")));
    println!("check_permutation: {}", strings::check_permutation(String::from("pepe"), String::from("ppee")));
    println!("urlify: {}", strings::urlify(String::from("Mr John Smith ")));
    println!("palindrome_permutation: {}", strings::palindrome_permutation(String::from("Tact rCoa")));
    println!("one_way: {}", strings::one_way(String::from("pep1"), String::from("pepe")));
    println!("string_compression: {}", strings::string_compression(String::from("aabcccccaaa")));

    println!("");

    // ---------- SORTING AND SEARCHING FUNCTIONS CALLS ---------- 
    println!("-- SORTING AND SEARCHING FUNCTIONS CALLS --");

    let mut v: Vec<i32> = vec![5,3,1,4,2];
    println!("bubble_sort: {:?}", sort::bubble_sort(&mut v));
    
    v = vec![5,3,1,4,2];
    println!("selection_sort: {:?}", sort::selection_sort(&mut v));

    v = vec![5,3,1,4,2];
    println!("merge_sort: {:?}", sort::merge_sort(&mut v));
    

}
