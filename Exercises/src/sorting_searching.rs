
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

// -------- SELECTION SORT --------

// -------- MERGE SORT --------

// -------- QUICK SORT --------

// -------- RADIX SORT --------


// -------------------- SEARCHING ALGORITHMS --------------------