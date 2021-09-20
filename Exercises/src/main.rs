extern crate Exercises;

fn main() {

    // ---------- ARRAYS_STRINGS FUNTIONS CALLS ---------- 
    Exercises::arrays_strings::is_unique_array(String::from("string"));
    let result = Exercises::arrays_strings::is_unique_hashmap(String::from("string"));
    println!("El resultado es: {}", result);

}
