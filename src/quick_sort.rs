///////////////////////////////////////////////////////////////////////////////

use std::fmt;

///////////////////////////////////////////////////////////////////////////////

// --- Public documentation
//
/// Sorts the provided slice in ascending order.
///
/// - Inputs
///     | `arr: &[T]`
///     | The slice to sort
///
///
fn quick_sort<T: Clone + Ord + fmt::Debug>(arr: Vec<T>) -> Vec<T> {
    fn inner<T: Clone + Ord + fmt::Debug>(arr: Vec<&T>) -> Vec<&T> {
        if arr.len() < 2 {
            arr.to_vec()
        } else {
            let pivot = arr.len() / 2;

            let mut less: Vec<&T> = vec![];
            let mut greater: Vec<&T> = vec![];

            for i in 0..pivot {
                let item = arr[i];
                if item < arr[pivot] {
                    less.push(item);
                } else {
                    greater.push(item);
                }
            }

            for i in pivot + 1..arr.len() {
                let item = arr[i];
                if item < arr[pivot] {
                    less.push(item);
                } else {
                    greater.push(item);
                }
            }

            let less = inner(less);
            let greater = inner(greater);

            [less, vec![arr[pivot]], greater].concat()
        }
    }

    inner(arr.iter().map(|i| i).collect())
        .iter()
        .map(|i| i.to_owned().to_owned())
        .collect()
}

///////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {

    use super::*;

    fn helper(cases: Vec<Vec<i32>>) {
        for case in cases {
            let real = case.clone();
            let mut expected = case.clone();

            let real = quick_sort(real);

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
}

///////////////////////////////////////////////////////////////////////////////
