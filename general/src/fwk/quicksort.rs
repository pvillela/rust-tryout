//! In-place quicksort algorithm.

use std::{cmp, fmt::Debug};

#[derive(Debug)]
enum Movement {
    Stuck,
    Met,
    Free,
}

fn move_i<T: PartialOrd>(arr: &[T], i: &mut usize, j: usize) -> Movement {
    let pivot = arr.len() - 1;
    match (*i < j - 1, arr[*i] <= arr[pivot]) {
        (_, false) => Movement::Stuck,
        (false, _) => Movement::Met,
        (true, true) => {
            *i += 1;
            Movement::Free
        }
    }
}

fn move_j<T: PartialOrd>(arr: &[T], i: usize, j: &mut usize) -> Movement {
    let pivot = arr.len() - 1;
    match (i < *j - 1, arr[*j] >= arr[pivot]) {
        (_, false) => Movement::Stuck,
        (false, _) => Movement::Met,
        (true, true) => {
            *j -= 1;
            Movement::Free
        }
    }
}

fn recurse_sort<T: PartialOrd + Debug>(arr: &mut [T], pivot: usize, pivot_landing: usize) {
    arr.swap(pivot_landing, pivot);
    println!(
        "    >>> left={:?}, pivot_val={:?} right={:?}",
        &arr[..pivot_landing],
        arr[pivot_landing],
        &arr[pivot_landing + 1..]
    );
    quicksort(&mut arr[..pivot_landing]);
    quicksort(&mut arr[pivot_landing + 1..]);
}

/// Implementation of quicksort algorithm.
///
/// Sorts `arr` in place without heap allocations, but there is stack allocation with recursion.
pub fn quicksort<T: PartialOrd + Debug>(arr: &mut [T]) {
    let len = arr.len();

    println!("*** arr={arr:?}, len={len}");

    if len <= 1 {
        println!("    sorted_arr={arr:?}");
        return;
    }

    if len == 2 {
        if arr[0] > arr[1] {
            arr.swap(0, 1);
        }
        println!("    sorted_arr={arr:?}");
        return;
    }

    let m = len / 2;
    arr.swap(m, len - 1);
    let pivot = len - 1;

    let mut i = 0;
    let mut j = pivot - 1;

    println!("    arr_after_pivot={arr:?}, len={len}, pivot={pivot}");

    loop {
        println!("    i={i}, j={j}");
        let i_movement = move_i(arr, &mut i, j);
        let j_movement = move_j(arr, i, &mut j);
        println!("    i_movement={i_movement:?}, i={i}, j_movement={j_movement:?}, j={j}");

        match (i_movement, j_movement) {
            (Movement::Stuck, Movement::Met) => {
                let pivot_landing = i;
                recurse_sort(arr, pivot, pivot_landing);
                break;
            }
            (Movement::Met, Movement::Stuck) => {
                let pivot_landing = j + 1;
                recurse_sort(arr, pivot, pivot_landing);
                break;
            }
            (Movement::Met, Movement::Met) => {
                let pivot_landing = j;
                recurse_sort(arr, pivot, pivot_landing);
                break;
            }
            (Movement::Stuck, Movement::Stuck) => {
                arr.swap(i, j);
            }
            (Movement::Free, _) | (_, Movement::Free) => (),
        }
    }
    println!("    sorted_arr={arr:?}");
}

fn recurse_select<T: PartialOrd + Debug>(
    arr: &mut [T],
    pivot: usize,
    pivot_landing: usize,
    k: usize,
) {
    arr.swap(pivot_landing, pivot);
    println!(
        "    >>> left={:?}, pivot_val={:?} right={:?}",
        &arr[..pivot_landing],
        arr[pivot_landing],
        &arr[pivot_landing + 1..]
    );

    match k.cmp(&pivot_landing) {
        cmp::Ordering::Less => quickselect(&mut arr[..pivot_landing], k),
        cmp::Ordering::Equal => return,
        cmp::Ordering::Greater => quickselect(&mut arr[pivot_landing + 1..], k - pivot_landing - 1),
    }
}

/// Implementation of quickselect algorithm.
///
/// Rearranges `arr`, with in-place changes, so that its `k`th ranked item is in the correct position.
/// No heap allocations, but there is stack allocation with recursion.
///
/// # Panics
/// - If `k` >= `arr.len()`.
pub fn quickselect<T: PartialOrd + Debug>(arr: &mut [T], k: usize) {
    let len = arr.len();
    assert!(k < len, "`k` must be less than `arr.len()`");

    println!("*** arr={arr:?}, len={len}");

    if len <= 1 {
        println!("    final_arr={arr:?}");
        return;
    }

    if len == 2 {
        if arr[0] > arr[1] {
            arr.swap(0, 1);
        }
        println!("    final_arr={arr:?}");
        return;
    }

    let m = len / 2;
    arr.swap(m, len - 1);
    let pivot = len - 1;

    let mut i = 0;
    let mut j = pivot - 1;

    println!("    arr_after_pivot={arr:?}, len={len}, pivot={pivot}");

    loop {
        println!("    i={i}, j={j}");
        let i_movement = move_i(arr, &mut i, j);
        let j_movement = move_j(arr, i, &mut j);
        println!("    i_movement={i_movement:?}, i={i}, j_movement={j_movement:?}, j={j}");

        match (i_movement, j_movement) {
            (Movement::Stuck, Movement::Met) => {
                let pivot_landing = i;
                recurse_select(arr, pivot, pivot_landing, k);
                break;
            }
            (Movement::Met, Movement::Stuck) => {
                let pivot_landing = j + 1;
                recurse_select(arr, pivot, pivot_landing, k);
                break;
            }
            (Movement::Met, Movement::Met) => {
                let pivot_landing = j;
                recurse_select(arr, pivot, pivot_landing, k);
                break;
            }
            (Movement::Stuck, Movement::Stuck) => {
                arr.swap(i, j);
            }
            (Movement::Free, _) | (_, Movement::Free) => (),
        }
    }
    println!("    final_arr={arr:?}");
}

fn move_max_to_end<T: PartialOrd + Debug>(arr: &mut [T]) {
    let len = arr.len();

    for i in 0..len {
        if arr[i] > arr[len - 1] {
            arr.swap(i, len - 1);
        }
    }
}

/// Implementation of quickmedian algorithm.
///
/// Rearranges `arr`, with in-place changes, so that its median item or upper and lower median items is/are in the
/// correct position.
/// No heap allocations, but there is stack allocation with recursion.
///
/// # Panics
/// - If `arr.len() == 0`.
pub fn quickmedian<T: PartialOrd + Debug>(arr: &mut [T]) {
    let len = arr.len();
    assert!(len > 0, "`arr.len()` must be positive");
    let k = len / 2;
    quickselect(arr, k);
    if len % 2 == 0 {
        move_max_to_end(&mut arr[0..k]);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sort() {
        let arr = [10., 1.5, 9., 2.5, 8., 3.5, 7., 4.5, 6., 5.5];
        println!("arr before sorting: {:?}", arr);

        let arr_s = [1.5, 2.5, 3.5, 4.5, 5.5, 6., 7., 8., 9., 10.];

        let arr_r = {
            let mut arr_r = arr_s.clone();
            arr_r.reverse();
            arr_r
        };
        println!("arr_r before sorting: {:?}", arr_r);

        {
            println!();
            let mut a = arr_s.clone();
            quicksort(&mut a);
            println!("arr_s after quicksort: {:?}", a);
            assert_eq!(a, arr_s);
        }

        {
            println!();
            let mut a = arr.clone();
            quicksort(&mut a);
            println!("arr after quicksort: {:?}", a);
            assert_eq!(a, arr_s);
        }

        {
            println!();
            let mut a = arr_r.clone();
            quicksort(&mut a);
            println!("arr_r after quicksort: {:?}", a);
            assert_eq!(a, arr_s);
        }
    }

    #[test]
    fn test_select() {
        let arr = [10., 1.5, 9., 2.5, 8., 3.5, 7., 4.5, 6., 5.5];
        println!("arr before pivots: {:?}", arr);

        let arr_s = [1.5, 2.5, 3.5, 4.5, 5.5, 6., 7., 8., 9., 10.];

        let arr_r = {
            let mut arr_r = arr_s.clone();
            arr_r.reverse();
            arr_r
        };
        println!("arr_r before pivots: {:?}", arr_r);

        for k in [3, 0] {
            {
                println!();
                let mut a = arr_s.clone();
                quickselect(&mut a, k);
                println!("arr_s after quickselect({k}): {:?}", a);
                assert_eq!(a[k], arr_s[k]);
            }

            {
                println!();
                let mut a = arr.clone();
                quickselect(&mut a, k);
                println!("arr after quickselect({k}): {:?}", a);
                assert_eq!(a[k], arr_s[k]);
            }

            {
                println!();
                let mut a = arr_r.clone();
                quickselect(&mut a, k);
                println!("arr_r after quickselect({k}): {:?}", a);
                assert_eq!(a[k], arr_s[k]);
            }
        }
    }

    #[test]
    fn test_median() {
        let arr = [10., 1.5, 9., 2.5, 8., 3.5, 7., 4.5, 6., 5.5];
        println!("arr before pivots: {:?}", arr);

        let arr_s = [1.5, 2.5, 3.5, 4.5, 5.5, 6., 7., 8., 9., 10.];

        let arr_r = {
            let mut arr_r = arr_s.clone();
            arr_r.reverse();
            arr_r
        };
        println!("arr_r before pivots: {:?}", arr_r);

        {
            println!("Even length vector");
            let len = 10;
            let k = len / 2;

            {
                println!();
                let mut a = arr_s.clone();
                quickmedian(&mut a);
                println!("arr_s after quickmedian(): {:?}", a);
                assert_eq!(a[k - 1..k + 1], arr_s[k - 1..k + 1]);
            }

            {
                println!();
                let mut a = arr.clone();
                quickmedian(&mut a);
                println!("arr after quickmedian(): {:?}", a);
                assert_eq!(a[k - 1..k + 1], arr_s[k - 1..k + 1]);
            }

            {
                println!();
                let mut a = arr_r.clone();
                quickmedian(&mut a);
                println!("arr_r after quickmedian(): {:?}", a);
                assert_eq!(a[k - 1..k + 1], arr_s[k - 1..k + 1]);
            }
        }

        {
            println!("Odd length vector");
            let len = 9;
            let k = len / 2;

            let arr = arr[0..len].to_vec();
            let arr_s = {
                let mut a = arr.clone();
                a.sort_by(f64::total_cmp);
                a
            };
            let arr_r = {
                let mut a = arr.clone();
                a.reverse();
                a
            };

            {
                println!();
                let mut a = arr_s.clone();
                quickmedian(&mut a);
                println!("arr_s after quickmedian(): {:?}", a);
                assert_eq!(a[k], arr_s[k]);
            }

            {
                println!();
                let mut a = arr.clone();
                quickmedian(&mut a);
                println!("arr after quickmedian(): {:?}", a);
                assert_eq!(a[k], arr_s[k]);
            }

            {
                println!();
                let mut a = arr_r.clone();
                quickmedian(&mut a);
                println!("arr_r after quickmedian(): {:?}", a);
                assert_eq!(a[k], arr_s[k]);
            }
        }
    }
}
