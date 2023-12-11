use std::{error::Error, collections::HashMap, fs::File, io::{BufReader, BufRead as _}};

pub fn p1(input_filename: &str) -> Result<u32, Box<dyn Error>> {
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

pub fn p2(input_filename: &str) -> Result<u32, Box<dyn Error>> {
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

#[cfg(test)]
mod tests {
use crate::*;
    #[test]
    fn day2_p1_sample() {
        assert_eq!(day_2::p1("inputs/day_2_p1.sample").unwrap(), 8);
    }

    #[test]
    fn day2_p2_sample() {
        assert_eq!(day_2::p2("inputs/day_2_p2.sample").unwrap(), 2286);
    }
}
