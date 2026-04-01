// Include the aski-generated domain types from multiple modules
#[allow(dead_code, unused_variables, unreachable_patterns)]
mod chart {
    include!(concat!(env!("OUT_DIR"), "/chart_generated.rs"));
}

use chart::*;

/// Convert ecliptic longitude (0-360) to zodiac sign.
/// This is the FFI bridge — aski declares the interface, Rust implements it.
fn sign_from_degree(longitude: f64) -> Sign {
    let sign_index = (longitude / 30.0) as usize % 12;
    match sign_index {
        0 => Sign::Aries,
        1 => Sign::Taurus,
        2 => Sign::Gemini,
        3 => Sign::Cancer,
        4 => Sign::Leo,
        5 => Sign::Virgo,
        6 => Sign::Libra,
        7 => Sign::Scorpio,
        8 => Sign::Sagittarius,
        9 => Sign::Capricorn,
        10 => Sign::Aquarius,
        11 => Sign::Pisces,
        _ => unreachable!(),
    }
}

/// Convert house number (1-12) to House domain.
fn house_from_number(n: u32) -> House {
    match n {
        1 => House::First,
        2 => House::Second,
        3 => House::Third,
        4 => House::Fourth,
        5 => House::Fifth,
        6 => House::Sixth,
        7 => House::Seventh,
        8 => House::Eighth,
        9 => House::Ninth,
        10 => House::Tenth,
        11 => House::Eleventh,
        12 => House::Twelfth,
        _ => House::First,
    }
}

/// Detect aspect between two planetary longitudes.
fn detect_aspect(lon1: f64, lon2: f64) -> Option<(Aspect, f64)> {
    let mut diff = (lon1 - lon2).abs();
    if diff > 180.0 {
        diff = 360.0 - diff;
    }

    let aspects = [
        (Aspect::Conjunction, 0.0),
        (Aspect::Sextile, 60.0),
        (Aspect::Square, 90.0),
        (Aspect::Trine, 120.0),
        (Aspect::Opposition, 180.0),
    ];

    for (aspect, exact) in &aspects {
        let orb = (diff - exact).abs();
        if orb <= aspect.orb() {
            return Some((*aspect, orb));
        }
    }
    None
}

fn main() {
    // Example: a birth chart with known planetary positions (longitudes)
    // These would normally come from swiss-eph; hardcoded for testing
    let planet_longitudes: &[(Planet, f64)] = &[
        (Planet::Sun,     120.5),  // ~0° Leo
        (Planet::Moon,    95.3),   // ~5° Cancer
        (Planet::Mercury, 135.7),  // ~15° Leo
        (Planet::Venus,   108.2),  // ~18° Cancer
        (Planet::Mars,    25.8),   // ~25° Aries
        (Planet::Jupiter, 268.4),  // ~28° Sagittarius
        (Planet::Saturn,  305.1),  // ~5° Aquarius
    ];

    println!("=== Natal Chart ===\n");

    // Compute sign placements using the FFI bridge
    for (planet, longitude) in planet_longitudes {
        let sign = sign_from_degree(*longitude);
        let degree_in_sign = longitude % 30.0;
        let dignity = planet.dignity(&sign);
        let element = sign.element();
        let modality = sign.modality();

        println!(
            "  {:?} at {:.1}° {:?} ({:?}, {:?}) — {:?}",
            planet, degree_in_sign, sign, element, modality, dignity
        );
    }

    // Detect aspects between planets
    println!("\n=== Aspects ===\n");
    for i in 0..planet_longitudes.len() {
        for j in (i + 1)..planet_longitudes.len() {
            let (p1, lon1) = &planet_longitudes[i];
            let (p2, lon2) = &planet_longitudes[j];
            if let Some((aspect, orb)) = detect_aspect(*lon1, *lon2) {
                println!(
                    "  {:?} {:?} {:?} (orb: {:.1}°)",
                    p1, aspect, p2, orb
                );
            }
        }
    }

    // Summary
    let chart = ChartData {
        sun_sign: sign_from_degree(120.5),
        moon_sign: sign_from_degree(95.3),
        rising: sign_from_degree(210.0), // ~0° Scorpio (example)
        mid_heaven: sign_from_degree(120.0), // ~0° Leo (example)
    };

    println!("\n=== Chart Summary ===\n");
    println!("  Sun:        {:?}", chart.sun_sign);
    println!("  Moon:       {:?}", chart.moon_sign);
    println!("  Rising:     {:?}", chart.rising);
    println!("  MidHeaven:  {:?}", chart.mid_heaven);
}
