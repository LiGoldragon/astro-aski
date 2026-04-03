// Text-only chart output — works without display server

#[allow(dead_code, unused_variables, unreachable_patterns)]
mod chart {
    include!(concat!(env!("OUT_DIR"), "/chart_generated.rs"));
}

use chart::*;

fn sign_from_degree(longitude: f64) -> Sign {
    let sign_index = (longitude / 30.0) as usize % 12;
    match sign_index {
        0 => Sign::Aries, 1 => Sign::Taurus, 2 => Sign::Gemini,
        3 => Sign::Cancer, 4 => Sign::Leo, 5 => Sign::Virgo,
        6 => Sign::Libra, 7 => Sign::Scorpio, 8 => Sign::Sagittarius,
        9 => Sign::Capricorn, 10 => Sign::Aquarius, 11 => Sign::Pisces,
        _ => unreachable!(),
    }
}

fn detect_aspect(lon1: f64, lon2: f64) -> Option<(Aspect, f64)> {
    let mut diff = (lon1 - lon2).abs();
    if diff > 180.0 { diff = 360.0 - diff; }
    let aspects = [
        (Aspect::Conjunction, 0.0), (Aspect::Sextile, 60.0),
        (Aspect::Square, 90.0), (Aspect::Trine, 120.0),
        (Aspect::Opposition, 180.0),
    ];
    for (aspect, exact) in &aspects {
        let orb = (diff - exact).abs();
        if orb <= aspect.orb() { return Some((*aspect, orb)); }
    }
    None
}

fn main() {
    use swiss_eph::safe;

    let jd = safe::julday(2026, 4, 3, 12.0);
    let flags = safe::CalcFlags::new().with_speed();

    let planets = [
        (Planet::Sun, safe::Planet::Sun),
        (Planet::Moon, safe::Planet::Moon),
        (Planet::Mercury, safe::Planet::Mercury),
        (Planet::Venus, safe::Planet::Venus),
        (Planet::Mars, safe::Planet::Mars),
        (Planet::Jupiter, safe::Planet::Jupiter),
        (Planet::Saturn, safe::Planet::Saturn),
    ];

    let mut positions = Vec::new();
    println!("=== Natal Chart — 2026-04-03 12:00 UTC ===\n");
    for (aski_planet, eph_planet) in &planets {
        match safe::calc(jd, *eph_planet, flags) {
            Ok(pos) => {
                let sign = sign_from_degree(pos.longitude);
                let degree = pos.longitude % 30.0;
                let dignity = aski_planet.dignity(&sign);
                let element = sign.element();
                println!("  {:?} at {:.1}° {:?} ({:?}) — {:?}", aski_planet, degree, sign, element, dignity);
                positions.push((*aski_planet, pos.longitude));
            }
            Err(e) => eprintln!("  {:?}: error {:?}", aski_planet, e),
        }
    }

    println!("\n=== Aspects ===\n");
    for i in 0..positions.len() {
        for j in (i + 1)..positions.len() {
            if let Some((aspect, orb)) = detect_aspect(positions[i].1, positions[j].1) {
                println!("  {:?} {:?} {:?} (orb: {:.1}°)", positions[i].0, aspect, positions[j].0, orb);
            }
        }
    }
}
