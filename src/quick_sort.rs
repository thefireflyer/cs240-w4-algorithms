///////////////////////////////////////////////////////////////////////////////

use std::fmt;

///////////////////////////////////////////////////////////////////////////////

/// Sorts the provided slice in ascending order.
///
/// - Inputs
///     | `arr: &mut [T]`
///     | The slice to sort (mutable)
///
/// - Side effects
///     | Sorts `arr` in ascending order
///
pub fn quick_sort<T: Ord + fmt::Debug>(arr: &mut [T]) {
    // heavily based on [2]
    // see commit `fa58f0d` for quick sort without in place mutation
    // (faster, more memory)

    /*
    --- Quick sort


    */

    fn part<T: Ord + fmt::Debug>(arr: &mut [T]) -> usize {
        let pivot = arr.len() - 1;

        let mut lower_end = 0;

        for i in 0..arr.len() {
            if arr[i] < arr[pivot] {
                arr.swap(i, lower_end);
                lower_end += 1;
            }
        }

        arr.swap(pivot, lower_end);

        lower_end
    }

    fn inner<T: Ord + fmt::Debug>(arr: &mut [T]) {
        if arr.len() > 1 {
            let pivot = part(arr);
            inner(&mut arr[..pivot]);
            inner(&mut arr[pivot + 1..]);
        }
    }

    inner(arr);
}

///////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {

    use super::*;

    fn helper(cases: Vec<Vec<i32>>) {
        for case in cases {
            let mut real = case.clone();
            let mut expected = case.clone();

            quick_sort(&mut real);

            expected.sort();

            assert_eq!(real, expected);
        }
    }

    #[test]
    fn special_cases() {
        helper(vec![vec![], vec![1]])
    }

    #[test]
    fn random_cases() {
        helper(vec![
            vec![1, 3, 2],
            vec![2, 3, 1],
            vec![3, 3, 3],
            vec![3, 3, 2],
            vec![593, 52, 0, 40104, 20, 19, 2, 30, 8],
            vec![5, 23, 6, 8, 9, 0, 2],
        ])
    }

    #[test]
    fn sorted_cases() {
        helper(vec![
            vec![1, 2],
            vec![1, 2, 3],
            vec![0, 2, 5, 6, 8, 9, 23],
            vec![-503, 1, 203, 585, 900],
        ]);
    }

    #[test]
    fn reverse_sorted_cases() {
        helper(vec![
            vec![2, 1],
            vec![3, 2, 1],
            vec![5, 4, 3, 2, 1, 0, -40],
            vec![23, 9, 8, 6, 5, 2, 0],
        ]);
    }

    #[test]
    fn test_big_sorted() {
        let big_number = (2 as i32).pow(9);
        let mut arr: Vec<i32> = Vec::with_capacity(big_number as usize);
        for i in 0..big_number {
            arr.push(i);
        }

        helper(vec![arr]);
    }

    #[test]
    fn test_big_rev_sorted() {
        let big_number = (2 as i32).pow(25);
        let mut arr: Vec<i32> = Vec::with_capacity(big_number as usize);
        for i in big_number..0 {
            arr.push(i);
        }

        helper(vec![arr]);
    }
}

///////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod benchmarks {
    // extern crate test;

    // use test::Bencher;

    // fn bench_helper(cases: Vec<Vec<i32>>, b: &mut Bencher) {
    //     for mut case in cases {
    //         b.bench(|_| Ok(quick_sort(&mut case)));
    //     }
    // }

    // #[bench]
    // fn random_cases_b(b: &mut Bencher) {
    //     bench_helper(
    //         vec![
    //             vec![1, 3, 2],
    //             vec![2, 3, 1],
    //             vec![3, 3, 3],
    //             vec![3, 3, 2],
    //             vec![593, 52, 0, 40104, 20, 19, 2, 30, 8],
    //             vec![5, 23, 6, 8, 9, 0, 2],
    //         ],
    //         b,
    //     );
    // }
}

///////////////////////////////////////////////////////////////////////////////

fn debug<T: fmt::Debug>(arr: &mut [T], i: usize, pivot: usize, lower_end: usize) {
    println!("{:?}", arr);

    print!(" ");
    for n in 0..arr.len() {
        if n == i {
            print!("^");
        } else {
            print!(".");
        }
        print!("  ");
    }
    println!();

    print!(" ");
    for n in 0..arr.len() {
        if n == pivot {
            print!("*");
        } else {
            print!(".");
        }
        print!("  ");
    }
    println!();

    print!(" ");
    for n in 0..arr.len() {
        if n == lower_end {
            print!("e");
        } else {
            print!(".");
        }
        print!("  ");
    }
    println!();
}

///////////////////////////////////////////////////////////////////////////////
