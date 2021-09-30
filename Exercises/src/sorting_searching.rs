
// -------------------- SORTING ALGORITHMS --------------------

// -------- BUBBLE SORT (Runtime: 0(n^2) average and worst case. Memory: 0(1)) --------
// We start at the beginning of the array and swap the first two elements if the first is greater
// than the second. Then, we go to the next pair, and so on, continuously making sweeps of the array until it is
// sorted. In doing so, the smaller items slowly "bubble" up to the beginning of the list.
pub fn bubble_sort(array: &mut Vec<i32>) -> &mut Vec<i32> { 
    for i in 0..array.len() {
        for j in 0..array.len() - i - 1 {
            if array[j] > array[j + 1] { array.swap(j, j+1); }
        }
    }
    array
}

// -------- SELECTION SORT (Runtime: 0(n^2) average and worst case. Memory: 0(1))--------
// Find the smallest element using a linear scan and move it to the front (swapping it with the front element). 
// Then, find the second smallest and move it, again doing a linear scan. Continue doing this until all the elements are in place.
pub fn selection_sort(array: &mut Vec<i32>) -> &mut Vec<i32> { 
    for i in 0..array.len() {
        let mut pos = array.len() + 1;

        for j in i..array.len() {
            if pos == array.len() + 1 || array[pos] > array[j] { pos = j; }
        }
        
        array.swap(i, pos);
    }
    array
}

// -------- MERGE SORT --------

// -------- QUICK SORT --------

// -------- RADIX SORT --------


// -------------------- SEARCHING ALGORITHMS --------------------