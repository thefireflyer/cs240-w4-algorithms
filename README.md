# CS 240 | Week 4 | Algorithms

[![Unit testing](https://github.com/thefireflyer/cs240-w4-algorithms/actions/workflows/test.yml/badge.svg?branch=master)](https://github.com/thefireflyer/cs240-w4-algorithms/actions/workflows/test.yml)


| | |
|-|-|
| Author | Aidan Beil |
| Date | 25/1/2024 |
| Class | CS240 2963 |
| Professor | Darrell Criss |

---

### Organization

- Pseudo code and time complexity
    - [merge sort](pseudocode/mergesort)
    - [quick sort](pseudocode/quicksort)
- Actual code
    - [main](src/main.rs)
    - [quick sort](src/quick_sort.rs)
    - [merge sort](src/merge_sort.rs)

---

### Usage

`cargo run`

> ```
>[quick] 90262 @ Ok(3608)
>[merge] 90262 @ Ok(3608)
>[std]   90262 @ Ok(3608)
>
>[quick] 11559 @ Err(448)
>[merge] 11559 @ Err(448)
>[std]   11559 @ Err(448)
> ```


`cargo test --release`

> ```
>running 12 tests
>test merge_sort::tests::random_cases ... ok
>test merge_sort::tests::reverse_sorted_cases ... ok
>test merge_sort::tests::special_cases ... ok
>test merge_sort::tests::sorted_cases ... ok
>test merge_sort::tests::test_big_rev_sorted ... ok
>test quick_sort::tests::random_cases ... ok
>test quick_sort::tests::sorted_cases ... ok
>test quick_sort::tests::special_cases ... ok
>test quick_sort::tests::reverse_sorted_cases ... ok
>test quick_sort::tests::test_big_rev_sorted ... ok
>test merge_sort::tests::test_big_sorted ... ok
>test quick_sort::tests::test_big_sorted ... ok
>
>test result: ok. 12 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 7.43s
> ```

---

### Sectioning

`///////////////////////////////////////////////////////////////////////////////`

`//---------------------------------------------------------------------------//`

`//...........................................................................//`


### Sources

[1] [Grokking Algorithms](https://livebook.manning.com/book/grokking-algorithms-second-edition/chapter-1/v-4/)

[2] [The Algorithm Design Manual, 3rd Edition](https://www.algorist.com/)


---
