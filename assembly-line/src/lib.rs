// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub fn production_rate_per_hour(speed: u8) -> f64 {
    const CARS_PER_HOUR: f64 = 221.0;
    let mut success_rate: f64 = 0.0;

    match speed{
        1|2|3|4 =>success_rate = 1.0,
        5|6|7|8 =>success_rate = 0.90,
        9|10 => success_rate = 0.77,
        _=> success_rate = 0.0
    }

    let rate_per_hour: f64 = CARS_PER_HOUR * speed as f64 * success_rate;
    rate_per_hour
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    production_rate_per_hour(speed) as u32 / 60 
}
