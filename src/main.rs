use std::error::Error;

mod day_1;
mod day_2;

fn main() -> Result<(), Box<dyn Error>> {
    //let calibration_values = day1_p2("day1.input")?;
    //println!("day1_p2: {calibration_values:?}");

    let game_id_sum = day_2::p1("inputs/day_2.input")?;
    println!("day_2_p1: {game_id_sum:?}");

    let game_power_sum = day_2::p2("inputs/day_2.input")?;
    println!("day_2_p2: {game_power_sum:?}");
    Ok(())
}
