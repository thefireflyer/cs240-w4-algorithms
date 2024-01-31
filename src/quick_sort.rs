///////////////////////////////////////////////////////////////////////////////

use std::fmt;

use rand::{rngs::ThreadRng, thread_rng, Rng};

///////////////////////////////////////////////////////////////////////////////

// --- Public documentation
//
/// Sorts the provided slice in ascending order.
///
/// - Inputs
///     | `arr: &[T]` TODO: UPDATE!!
///     | The slice to sort
///
///
fn quick_sort<T: Clone + Ord + fmt::Debug>(arr: &mut [T]) {
    fn debug<T: Clone + Ord + fmt::Debug>(arr: &mut [T], i: usize, pivot: usize, lower_end: usize) {
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

    fn part<T: Clone + Ord + fmt::Debug>(arr: &mut [T], rng: &mut ThreadRng) -> usize {
        let pivot = arr.len() - 1;

        // let pivot: usize = rng.gen_range(0..arr.len());
        // arr.swap(pivot, arr.len() - 1);
        // let pivot = arr.len() - 1;

        println!("{:-<80}", "");

        let mut lower_end = 0;

        // [2,1,3] // [5,1,2,8,3,0,9]
        //  ^ * .  //  ! . . . *
        // [2,1,3] // [5,1,2,8,3,0,9]
        //  e ~ .  //  e ^ . . *
        // [2,1,3] // [1,5,2,8,3,0,9]
        //  e * ^  //  . e ^ . *
        //         // [1,2,5,8,3,0,9]
        //         //  . . e ^ *
        //         // [1,2,5,8,3,0,9]
        //         //  . . e . ~
        //         // [1,2,5,8,3,0,9]
        //         //  . . e . * ^
        //         // [1,2,0,8,3,5,9]
        //         //  . . . e * . ^
        //         // [1,2,0,8,3,5,9]
        //         //  . . . e * . .

        //         // [1,2,5,8,3,0,9]
        //         //  . . e . ~
        //         // [1,2,3,8,5,0,9]
        //         //  . . . e * ^
        //         // [1,2,3,0,5,8,9]
        //         //  . . . . e . ^
        //         // [1,2,3,0,5,8,9]

        for i in 0..arr.len() {
            if arr[i] < arr[pivot] {
                arr.swap(i, lower_end);
                lower_end += 1;
            }
            debug(arr, i, pivot, lower_end);
        }
        // [2,1,3] // [1,2,0,8,3,5,9]
        //  ^ * .  //  . . . e * . .
        // [1,2,3] // [1,2,0,3,8,5,9]

        arr.swap(pivot, lower_end);

        println!("{:-<80}\n{:?}\n", "", arr);

        lower_end
    }

    fn inner<T: Clone + Ord + fmt::Debug>(arr: &mut [T], rng: &mut ThreadRng) {
        println!("> {:?}\n", arr);
        if arr.len() > 1 {
            let pivot = part(arr, rng);
            inner(&mut arr[..pivot], rng);
            inner(&mut arr[pivot + 1..], rng);
        }
    }

    let mut rng = thread_rng();

    println!(">> {:?}", arr);
    inner(arr, &mut rng);
    println!("== {:?}\n\n", arr);
}

///////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {

    // extern crate test;

    // use test::Bencher;

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
        let big_number = (2 as i32).pow(20);
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
