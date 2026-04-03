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
