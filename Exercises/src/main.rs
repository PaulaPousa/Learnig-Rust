extern crate Exercises;

fn main() {
    // ---------- ARRAYS_STRINGS FUNCTIONS CALLS ---------- 
    Exercises::arrays_strings::is_unique_array(String::from("string"));
    Exercises::arrays_strings::is_unique_hashmap(String::from("string"));
    
    Exercises::arrays_strings::check_permutation(String::from("pepe"), String::from("ppee"));

    Exercises::arrays_strings::urlify(String::from("Mr John Smith "));

    let result = Exercises::arrays_strings::palindrome_permutation(String::from("Tact rCoa"));
    
    println!("El resultado es: {}", result);

}
