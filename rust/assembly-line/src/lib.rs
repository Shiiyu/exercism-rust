// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub fn production_rate_per_hour(speed: u8) -> f64 {
    let speed_float = speed as f64;

    if speed < 5 {
        return speed_float * 221.0;
    } else if speed < 9 {
        return speed_float * 221.0 * 0.9;
    }

    speed_float * 221.0 * 0.77
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    production_rate_per_hour(speed) as u32 / 60
}
