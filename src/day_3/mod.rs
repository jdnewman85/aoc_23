use std::{error::Error, collections::HashMap, fs::File, io::{BufReader, BufRead as _}};

use regex::Regex;

const DIGIT_RADIX: u32 = 10;
pub fn p1(input_filename: &str) -> Result<u32, Box<dyn Error>> {
    let file = File::open(input_filename)?;
    let lines = BufReader::new(file).lines();

    let _a = lines.map(|line| {
        let mut line = line.unwrap(); //? .to_lowercase();
        line
    });

    Ok(0_u32)
}

#[cfg(test)]
mod tests {
use crate::*;
    #[test]
    fn day3_p1_sample() {
        assert_eq!(day_3::p1("inputs/day_3_p1.sample").unwrap(), 4361);
    }
}
