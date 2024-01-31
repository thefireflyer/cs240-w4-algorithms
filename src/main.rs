///////////////////////////////////////////////////////////////////////////////

use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::Error;

mod merge_sort;
mod quick_sort;

///////////////////////////////////////////////////////////////////////////////

fn main() -> Result<(), Error> {
    // load numbers and turn them into a vector
    let mut input = parse_file("numbers-4.txt")?;

    // use quick sort
    let mut quick = input.clone();
    quick_sort::quick_sort(&mut quick);

    // use merge sort
    let merge = input.clone();
    let merge = merge_sort::merge_sort(merge);

    // use the standard library's sorting algorithm (merge sort)
    input.sort();

    // use builtin binary search to compare results
    println!("{:<7} 90262 @ {:?}", "[quick]", quick.binary_search(&90262));
    println!("{:<7} 90262 @ {:?}", "[merge]", merge.binary_search(&90262));
    println!("{:<7} 90262 @ {:?}", "[std]", input.binary_search(&90262));
    println!();
    println!("{:<7} 11559 @ {:?}", "[quick]", quick.binary_search(&11559));
    println!("{:<7} 11559 @ {:?}", "[merge]", merge.binary_search(&11559));
    println!("{:<7} 11559 @ {:?}", "[std]", input.binary_search(&11559));

    Ok(())
}

///////////////////////////////////////////////////////////////////////////////

fn parse_file(path: &str) -> Result<Vec<i32>, Error> {
    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    let arr: Vec<i32> = buffered
        .lines()
        .map(|val| val.unwrap().parse::<i32>().unwrap())
        .collect();

    Ok(arr)
}

///////////////////////////////////////////////////////////////////////////////
