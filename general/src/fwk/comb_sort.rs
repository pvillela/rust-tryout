/// Implementation of bubble sort algorithm.
///
/// Sorts `arr` in place without heap allocations.
pub fn bubble_sort<T: PartialOrd>(arr: &mut [T]) {
    let len = arr.len();
    if len <= 1 {
        return;
    }

    let mut swapped: Option<usize> = Some(len);

    let mut passes = 0;
    while let Some(k) = swapped {
        swapped = None;
        for i in 0..k - 1 {
            if arr[i] > arr[i + 1] {
                arr.swap(i, i + 1);
                swapped = Some(i + 1);
            }
        }
        passes += 1;
    }
    println!("bubble sort passes: {passes}");
}

/// Implementation of comb sort algorithm.
///
/// Sorts `arr` in place without heap allocations.
pub fn comb_sort<T: PartialOrd>(arr: &mut [T]) {
    let len = arr.len();
    if len <= 1 {
        return;
    }

    let mut gap = len;
    let shrink = 1.3;

    let mut passes = 0;
    // pre-buble sort
    while gap != 1 {
        // Calculate next gap
        gap = (gap as f64 / shrink).floor() as usize;
        if gap < 1 {
            gap = 1;
        }

        let mut i = 0;

        // Single "comb" pass through the array
        while i + gap < len {
            if arr[i] > arr[i + gap] {
                arr.swap(i, i + gap);
            }
            i += 1;
        }
        passes += 1;
    }
    println!("comb sort passes pre-bubble: {passes}");

    // Now gap == 1, do bubble sort
    bubble_sort(arr);
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
            bubble_sort(&mut a);
            println!("arr_s after bubble sort: {:?}", a);
            assert_eq!(a, arr_s);
        }

        {
            println!();
            let mut a = arr.clone();
            bubble_sort(&mut a);
            println!("arr after bubble sort: {:?}", a);
            assert_eq!(a, arr_s);
        }

        {
            println!();
            let mut a = arr_r.clone();
            bubble_sort(&mut a);
            println!("arr_r after bubble sort: {:?}", a);
            assert_eq!(a, arr_s);
        }

        {
            println!();
            let mut a = arr_s.clone();
            comb_sort(&mut a);
            println!("arr_s after comb sort: {:?}", a);
            assert_eq!(a, arr_s);
        }

        {
            println!();
            let mut a = arr.clone();
            comb_sort(&mut a);
            println!("arr after comb sort: {:?}", a);
            assert_eq!(a, arr_s);
        }

        {
            println!();
            let mut a = arr_r.clone();
            comb_sort(&mut a);
            println!("arr_r after comb sort: {:?}", a);
            assert_eq!(a, arr_s);
        }
    }
}
