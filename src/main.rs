///////////////////////////////////////////////////////////////////////////////
// #![feature(test)]

use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::Error;

mod merge_sort;
mod quick_sort;

///////////////////////////////////////////////////////////////////////////////

fn main() -> Result<(), Error> {
    let mut input = parse_file("numbers-4.txt")?;

    let mut quick = input.clone();
    quick_sort::quick_sort(&mut quick);

    let merge = input.clone();
    let merge = merge_sort::merge_sort(merge);

    input.sort();

    println!("[quick] 90262 @ {:?}", quick.binary_search(&90262));
    println!("[merge] 90262 @ {:?}", merge.binary_search(&90262));
    println!("[std] 90262 @ {:?}", input.binary_search(&90262));
    println!();
    println!("[quick] 11559 @ {:?}", quick.binary_search(&11559));
    println!("[merge] 11559 @ {:?}", merge.binary_search(&11559));
    println!("[std] 11559 @ {:?}", input.binary_search(&11559));

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
