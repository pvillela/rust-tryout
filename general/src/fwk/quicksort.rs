//! In-place quicksort algorithm.

use std::fmt::Debug;

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

fn recurse<T: PartialOrd + Debug>(arr: &mut [T], pivot: usize, pivot_landing: usize) {
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

/// Implementation of qucksort algorithm.
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
                recurse(arr, pivot, pivot_landing);
                break;
            }
            (Movement::Met, Movement::Stuck) => {
                let pivot_landing = j + 1;
                recurse(arr, pivot, pivot_landing);
                break;
            }
            (Movement::Met, Movement::Met) => {
                let pivot_landing = j;
                recurse(arr, pivot, pivot_landing);
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
            println!("arr_r after bubble sort: {:?}", a);
            assert_eq!(a, arr_s);
        }
    }
}
