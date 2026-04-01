// Include the aski-generated domain types
#[allow(dead_code, unused_variables)]
mod chart {
    include!(concat!(env!("OUT_DIR"), "/chart_generated.rs"));
}

use chart::*;

fn main() {
    // Test the generated domain types
    let sun_sign = Sign::Leo;
    let moon_sign = Sign::Cancer;
    let rising = Sign::Scorpio;

    println!("=== Natal Chart ===");
    println!("Sun:    {:?} ({:?}, {:?})", sun_sign, sun_sign.element(), sun_sign.modality());
    println!("Moon:   {:?} ({:?}, {:?})", moon_sign, moon_sign.element(), moon_sign.modality());
    println!("Rising: {:?} ({:?}, {:?})", rising, rising.element(), rising.modality());

    // Test essential dignities
    let planets = [
        Planet::Sun, Planet::Moon, Planet::Mercury, Planet::Venus,
        Planet::Mars, Planet::Jupiter, Planet::Saturn,
    ];

    println!("\n=== Essential Dignities in {:?} ===", sun_sign);
    for planet in &planets {
        let dignity = planet.dignity(&sun_sign);
        println!("  {:?}: {:?}", planet, dignity);
    }

    // Test aspect orbs
    println!("\n=== Aspect Orbs ===");
    let aspects = [
        Aspect::Conjunction, Aspect::Opposition, Aspect::Trine,
        Aspect::Square, Aspect::Sextile,
    ];
    for aspect in &aspects {
        println!("  {:?}: {:.1} degrees", aspect, aspect.orb());
    }

    // Test chart data construction
    let chart = ChartData {
        sun_sign: Sign::Leo,
        moon_sign: Sign::Cancer,
        rising: Sign::Scorpio,
        mid_heaven: Sign::Leo,
    };
    println!("\n=== Chart Data ===");
    println!("  Sun:        {:?}", chart.sun_sign);
    println!("  Moon:       {:?}", chart.moon_sign);
    println!("  Rising:     {:?}", chart.rising);
    println!("  MidHeaven:  {:?}", chart.mid_heaven);
}
