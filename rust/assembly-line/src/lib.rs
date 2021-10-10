static TOTAL_CARS_PER_HOUR: f64 = 221.0;

pub fn success_rate(speed: u8) -> f64 {
    match speed {
        0 => 0.00,
        1..=4 => 1.00,
        5..=8 => 0.90,
        9..=10 => 0.77,
        _ => panic!("Undefined fault rate for {}", speed)
    }
}

pub fn production_rate_per_hour(speed: u8) -> f64 {
    speed as f64 * TOTAL_CARS_PER_HOUR * success_rate(speed)
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    production_rate_per_hour(speed).round() as u32 / 60
}
