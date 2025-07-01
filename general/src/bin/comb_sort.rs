fn main() {
    let mut arr = [10., 1.5, 9., 2.5, 8., 3.5, 7., 4.5, 6., 5.5];
    println!("arr before sorting: {:?}", arr);
    let arr_s = [1.5, 2.5, 3.5, 4.5, 5.5, 6., 7., 8., 9., 10.];
    comb_sort(&mut arr);
    println!("arr after sorting: {:?}", arr);
    assert_eq!(arr, arr_s);
}

/// Implementation of comb-sort algorithm.
///
/// Sorts `arr` in place without heap allocations.
pub fn comb_sort<T: PartialOrd>(arr: &mut [T]) {
    let len = arr.len();
    if len <= 1 {
        return;
    }

    let mut gap = len;
    let mut swapped = true;
    let shrink = 1.3;

    while gap != 1 || swapped {
        // Calculate next gap
        gap = (gap as f64 / shrink).floor() as usize;
        if gap < 1 {
            gap = 1;
        }

        swapped = false;
        let mut i = 0;

        // Single "comb" pass through the array
        while i + gap < len {
            if arr[i] > arr[i + gap] {
                arr.swap(i, i + gap);
                swapped = true;
            }
            i += 1;
        }
    }
}
