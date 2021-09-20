extern crate Exercises;

fn main() {
    // ---------- ARRAYS_STRINGS FUNCTIONS CALLS ---------- 
    Exercises::arrays_strings::is_unique_array(String::from("string"));
    Exercises::arrays_strings::is_unique_hashmap(String::from("string"));
    
    let result = Exercises::arrays_strings::check_permutation(String::from("pepe"), String::from("ppee"));
    println!("El resultado es: {}", result);

}
