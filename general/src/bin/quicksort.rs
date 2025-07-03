use general::fwk::quicksort::quicksort;

fn main() {
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
