// Include the aski-generated domain types from multiple modules
#[allow(dead_code, unused_variables, unreachable_patterns)]
mod chart {
    include!(concat!(env!("OUT_DIR"), "/chart_generated.rs"));
}

use chart::*;

use ply_aski::chart::{ChartConfig, Placement, draw_chart};
use ply_aski::macroquad;
use ply_aski::prelude::*;

/// Convert ecliptic longitude (0-360) to zodiac sign.
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

/// Detect aspect between two planetary longitudes.
fn detect_aspect(lon1: f64, lon2: f64) -> Option<(Aspect, f64)> {
    let mut diff = (lon1 - lon2).abs();
    if diff > 180.0 { diff = 360.0 - diff; }

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

/// Compute planet positions using Swiss Ephemeris
fn compute_chart(year: i32, month: i32, day: i32, hour: f64) -> Vec<(Planet, f64)> {
    use swiss_eph::safe;

    let jd = safe::julday(year, month, day, hour);
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
    for (aski_planet, eph_planet) in &planets {
        match safe::calc(jd, *eph_planet, flags) {
            Ok(pos) => {
                positions.push((*aski_planet, pos.longitude));
            }
            Err(e) => {
                eprintln!("Failed to compute {:?}: {:?}", aski_planet, e);
            }
        }
    }
    positions
}

fn window_conf() -> macroquad::conf::Conf {
    macroquad::conf::Conf {
        miniquad_conf: miniquad::conf::Conf {
            window_title: "astro-aski — Natal Chart".to_owned(),
            window_width: 800,
            window_height: 750,
            high_dpi: true,
            ..Default::default()
        },
        ..Default::default()
    }
}

/// Compute and print chart data (works without display)
fn print_chart() -> Vec<(Planet, f64)> {
    let positions = compute_chart(2026, 4, 3, 12.0);

    println!("=== Natal Chart — 2026-04-03 12:00 UTC ===\n");
    for (planet, longitude) in &positions {
        let sign = sign_from_degree(*longitude);
        let degree = longitude % 30.0;
        let dignity = planet.dignity(&sign);
        let element = sign.element();
        println!("  {:?} at {:.1}° {:?} ({:?}) — {:?}", planet, degree, sign, element, dignity);
    }

    println!("\n=== Aspects ===\n");
    for i in 0..positions.len() {
        for j in (i + 1)..positions.len() {
            if let Some((aspect, orb)) = detect_aspect(positions[i].1, positions[j].1) {
                println!("  {:?} {:?} {:?} (orb: {:.1}°)", positions[i].0, aspect, positions[j].0, orb);
            }
        }
    }

    positions
}

/// Text-only mode: compute and display chart data without GUI
fn text_mode() {
    let positions = print_chart();
    let chart_data = ChartData {
        sun_sign: sign_from_degree(positions[0].1),
        moon_sign: sign_from_degree(positions[1].1),
        rising: sign_from_degree(0.0), // placeholder
        mid_heaven: sign_from_degree(positions[0].1),
    };
    println!("\n  Sun:  {:?}  Moon:  {:?}", chart_data.sun_sign, chart_data.moon_sign);
}

#[macroquad::main(window_conf)]
async fn main() {
    let positions = compute_chart(2026, 4, 3, 12.0);

    // Build placements for the visual chart
    let placements: Vec<Placement> = positions.iter().enumerate().map(|(i, (_, lon))| {
        Placement { planet_index: i, longitude: *lon as f32 }
    }).collect();

    let config = ChartConfig {
        center_x: 400.0,
        center_y: 375.0,
        outer_radius: 280.0,
        inner_radius: 220.0,
        planet_radius: 180.0,
        ascendant: 0.0, // TODO: compute from birth location
    };

    // Render loop
    loop {
        clear_background(macroquad::color::Color::new(0.05, 0.05, 0.1, 1.0));

        // Title
        draw_text("astro-aski — 2026-04-03 12:00 UTC", 220.0, 30.0, 24.0,
            macroquad::color::Color::new(0.7, 0.7, 0.8, 1.0));

        // Draw the chart
        draw_chart(&config, &placements);

        // Planet legend at bottom
        let legend_y = 710.0;
        for (i, (planet, lon)) in positions.iter().enumerate() {
            let sign = sign_from_degree(*lon);
            let deg = lon % 30.0;
            let text = format!("{:?}: {:.0}° {:?}", planet, deg, sign);
            let x = 30.0 + (i as f32) * 110.0;
            draw_text(&text, x, legend_y, 14.0,
                macroquad::color::Color::new(0.6, 0.6, 0.7, 1.0));
        }

        next_frame().await;
    }
}
