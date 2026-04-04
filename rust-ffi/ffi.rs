// Thin FFI adapter — the only hand-written Rust in this repo.
// Exposes swiss_eph through primitive-only signatures that aski can call.

use swiss_eph::safe;

pub fn julday(year: i32, month: i32, day: i32, hour: f64) -> f64 {
    safe::julday(year, month, day, hour)
}

pub fn calc_longitude(jd: f64, planet_index: i32) -> f64 {
    let planet = match planet_index {
        0 => safe::Planet::Sun,
        1 => safe::Planet::Moon,
        2 => safe::Planet::Mercury,
        3 => safe::Planet::Venus,
        4 => safe::Planet::Mars,
        5 => safe::Planet::Jupiter,
        6 => safe::Planet::Saturn,
        7 => safe::Planet::Uranus,
        8 => safe::Planet::Neptune,
        9 => safe::Planet::Pluto,
        _ => return 0.0,
    };
    let flags = safe::CalcFlags::new().with_speed();
    safe::calc(jd, planet, flags)
        .map(|pos| pos.longitude)
        .unwrap_or(0.0)
}
