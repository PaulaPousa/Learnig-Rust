extern crate Exercises;

fn main() {
    // ---------- ARRAYS_STRINGS FUNCTIONS CALLS ---------- 
    Exercises::arrays_strings::is_unique_array(String::from("string"));
    Exercises::arrays_strings::is_unique_hashmap(String::from("string"));
    Exercises::arrays_strings::check_permutation(String::from("pepe"), String::from("ppee"));
    Exercises::arrays_strings::urlify(String::from("Mr John Smith "));
    Exercises::arrays_strings::palindrome_permutation(String::from("Tact rCoa"));
    Exercises::arrays_strings::one_way(String::from("pep1"), String::from("pepe"));
    Exercises::arrays_strings::string_compression(String::from("aabcccccaaa"));

    // ---------- ARRAYS_STRINGS FUNCTIONS CALLS ---------- 
    let mut v: Vec<i32> = vec![5,3,1,4,2];
    Exercises::sorting_searching::bubble_sort(&mut v);
    v = vec![5,3,1,4,2];
    let result = Exercises::sorting_searching::selection_sort(&mut v);
    
    println!("{:?}", result);

}
