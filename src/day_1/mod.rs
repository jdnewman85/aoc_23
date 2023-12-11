#![allow(dead_code)]

use std::{error::Error, collections::HashMap, fs::File, io::{BufReader, BufRead as _}};

use regex::Regex;

const DIGIT_RADIX: u32 = 10;
pub fn p1(input_filename: &str) -> Result<u32, Box<dyn Error>> {
    let file = File::open(input_filename)?;
    let lines = BufReader::new(file).lines();

    let calibration_values:u32 = lines.map(|line| {
        let mut line = line.unwrap(); //? .to_lowercase();
        line.retain(|c| c.is_digit(DIGIT_RADIX));
        let mut digits = line.chars();

        let first = digits
            .next().unwrap()
            .to_digit(DIGIT_RADIX).unwrap();
        let last = match digits.last() {
            Some(d) => d.to_digit(DIGIT_RADIX).unwrap(),
            None => first,
        };

        first*10 + last
    }).sum();

    Ok(calibration_values)
}

pub fn p2(input_filename: &str) -> Result<u32, Box<dyn Error>> {
    //Setup
    let words = vec![
        "one",
        "two",
        "three",
        "four",
        "five",
        "six",
        "seven",
        "eight",
        "nine",
    ];

    let digits_to_u32 = words
        .iter()
        .enumerate()
        .fold(HashMap::new(), |mut map, (idx, &word)| {
            let num = (idx+1) as u32;
            map.insert(word.to_string(), num);
            map.insert(num.to_string(), num);
            map
        });

    let re_first = Regex::new(&format!(r"\d|{}", words.join("|")))?;
    let re_last = Regex::new(&format!(".*({re_first})"))?;


    //Action
    let file = File::open(input_filename)?;
    let lines = BufReader::new(file).lines();

    let calibration_values:u32 = lines.map(|line| {
        let line = line.unwrap(); //? .to_lowercase();
        let first_match = re_first.find(&line).unwrap();
        let last_capture = re_last.captures_at(&line, first_match.end()); //? first_match.start()+1); //oneight == 11 or 18?

        let first = first_match.as_str();
        let last = match last_capture {
            Some(captures) => captures.get(1).unwrap().as_str(),
            None => first,
        };

        let first = digits_to_u32[first];
        let last = digits_to_u32[last];

        first*10 + last
    }).sum();

    Ok(calibration_values)
}

#[cfg(test)]
mod tests {
use crate::*;
    #[test]
    fn day1_p1_sample() {
        assert_eq!(day_1::p1("inputs/day_1_p1.sample").unwrap(), 142);
    }
    #[test]
    fn day1_p2_sample() {
        assert_eq!(day_1::p2("inputs/day_1_p2.sample").unwrap(), 281);
    }
}
