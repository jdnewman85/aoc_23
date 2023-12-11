use std::{fs::File, error::Error, collections::HashMap, io::{BufReader, BufRead}};
use regex::Regex;

fn main() -> Result<(), Box<dyn Error>> {
    //let calibration_values = day1_p2("day1_p2.input")?;
    //println!("day1_p2: {calibration_values:?}");

    let game_id_sum = day2_p1("day2_p1.input")?;
    println!("day2_p1: {game_id_sum:?}");

    let game_power_sum = day2_p2("day2_p1.input")?;
    println!("day2_p2: {game_power_sum:?}");
    Ok(())
}

fn day2_p2(input_filename: &str) -> Result<u32, Box<dyn Error>> {
    //Setup
    let max_hash: HashMap<&str, u32> = [
        ("red", 0),
        ("green", 0),
        ("blue", 0),
    ].into();

    //Action
    let file = File::open(input_filename)?;
    let lines = BufReader::new(file).lines();

    let game_power_sum:u32 = lines.map(|line| {
        let line = line.unwrap(); //? .to_lowercase();
        let game = line.split(':');
        let rounds = game
            .skip(1)
            .next().unwrap();

        let maxes = rounds.split(';').fold(max_hash.clone(), |mut maxes, round| {
            round.split(',').for_each(|grab| {
                let mut grab_data = grab.split_whitespace();
                let mut grab_num: u32 = grab_data
                    .next().unwrap()
                    .parse().unwrap();
                let grab_color: &str = grab_data.next().unwrap();

                maxes.entry(grab_color).and_modify(|m| *m = *m.max(&mut grab_num));
            });
            maxes
        });

        maxes
            .values()
            .map(|v| *v)
            .reduce(|power, some_max| {
            power * some_max
        }).unwrap()

    }).sum();

    Ok(game_power_sum)
}

fn day2_p1(input_filename: &str) -> Result<u32, Box<dyn Error>> {
    //Setup
    let max_of_each: HashMap<&str, u32> = [
        ("red", 12),
        ("green", 13),
        ("blue", 14),
    ].into();

    //Action
    let file = File::open(input_filename)?;
    let lines = BufReader::new(file).lines();

    let game_id_sum:u32 = lines.map(|line| {
        let line = line.unwrap(); //? .to_lowercase();
        let mut game = line.split(':');
        let round_num: u32 = game
            .next().unwrap()
            .split_whitespace()
            .skip(1)
            .next().unwrap()
            .parse().unwrap();
        let rounds = game.next().unwrap();

        let possible = rounds.split(';').all(|round| {
            round.split(',').all(|grab| {
                let mut grab_data = grab.split_whitespace();
                let grab_num: u32 = grab_data
                    .next().unwrap()
                    .parse().unwrap();
                let grab_color: &str = grab_data.next().unwrap();

                let max = max_of_each[grab_color];
                return grab_num <= max;
            })
        });

        match possible {
            true => round_num,
            false => 0,
        }
    }).sum();

    Ok(game_id_sum)
}


#[allow(dead_code)]
fn day1_p2(input_filename: &str) -> Result<u32, Box<dyn Error>> {
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
    fn day1_p2_sample() {
        assert_eq!(day1_p2("day1_p2.sample").unwrap(), 281);
    }

    #[test]
    fn day2_p1_sample() {
        assert_eq!(day2_p1("day2_p1.sample").unwrap(), 8);
    }

    #[test]
    fn day2_p2_sample() {
        assert_eq!(day2_p2("day2_p1.sample").unwrap(), 2286);
    }
}
