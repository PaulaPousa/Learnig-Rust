// Publics Modules
pub mod arrays_strings;
pub mod linked_list;
pub mod sorting_searching;


// ---- TESTS ----
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bubble_sort_correct() {
        let mut array = vec![5, 4, 3, 2, 1];
        sorting_searching::bubble_sort(&mut array);
        assert_eq!(array, [1, 2, 3, 4, 5])
    }

    #[test]
    fn selection_sort_correct() {
        let mut array = vec![5, 4, 3, 2, 1];
        sorting_searching::selection_sort(&mut array);
        assert_eq!(array, [1, 2, 3, 4, 5])
    }
}

