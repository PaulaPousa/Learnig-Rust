extern crate exercises;

use exercises::arrays_strings as strings;
use exercises::sorting_searching as sort;

fn main() {

    // ---------- ARRAYS_STRINGS FUNCTIONS CALLS ---------- 
    strings::is_unique_array(String::from("string"));
    strings::is_unique_hashmap(String::from("string"));
    strings::check_permutation(String::from("pepe"), String::from("ppee"));
    strings::urlify(String::from("Mr John Smith "));
    strings::palindrome_permutation(String::from("Tact rCoa"));
    strings::one_way(String::from("pep1"), String::from("pepe"));
    strings::string_compression(String::from("aabcccccaaa"));

    // ---------- ARRAYS_STRINGS FUNCTIONS CALLS ---------- 
    let mut v: Vec<i32> = vec![5,3,1,4,2];
    sort::bubble_sort(&mut v);
    
    v = vec![5,3,1,4,2];
    sort::selection_sort(&mut v);

    v = vec![5,3,1,4,2];
    let result = sort::merge_sort(&mut v);
    
    println!("{:?}", result);

}
