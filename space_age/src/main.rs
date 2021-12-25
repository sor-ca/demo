use space_age::Age;
use space_age::Planet;
fn main() {
    let your_age = Age::new();
    let age_in_seconds=your_age.in_seconds();
    let your_planet= Planet::new();
    let your_planet_age = your_age.planet_age(&your_planet);
}
