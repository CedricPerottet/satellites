mod godmod;
use std::{thread, time};
pub use godmod::planet::{Planet, build_planet};
pub use godmod::universe::Universe;

fn main() {
    let mut universe = Universe { planets : Vec::new()};
    let mut terre : Planet= build_planet("Terre".to_string(), 0.0, 0.0, 0.0, 0.0, 5.97e24, 6.371e6);
    let mut lune1 : Planet= build_planet("Lune 1".to_string(), 384000000., 0.0, 0.0, 1000.0, 7.36e22, 1.737e6);
    let mut lune2 : Planet= build_planet("Lune 2".to_string(), -384000000., 0.0, 0.0, 600.0, 7.36e22, 1.737e6);
    let mut lune3 : Planet= build_planet("Lune 3".to_string(), -184000000., 0.0, 0.0, -600.0, 7.36e22, 1.737e6);
    let mut lune4 : Planet= build_planet("Lune 4".to_string(), 184000000., 0.0, 0.0, -800.0, 7.36e22, 1.737e6);
    let mut lune5 : Planet= build_planet("Lune 5".to_string(), 584000000., 0.0, 0.0, -400.0, 7.36e22, 1.737e6);
    let mut lune6 : Planet= build_planet("Lune 6".to_string(), 584000000., 584000000.0, -2000.0, -2000.0, 7.36e22, 1.737e6);
    universe.planets.push(&mut terre);
    universe.planets.push(&mut lune1);
    universe.planets.push(&mut lune2);
    universe.planets.push(&mut lune3);
    universe.planets.push(&mut lune4);
    universe.planets.push(&mut lune5);
    universe.planets.push(&mut lune6);

    // universe.remove("Terre".to_string());

    let dt = 10.; // [s]
    let total_simulation_time = 3600. * 24. * 20.; // [s]
    let n_steps = ((total_simulation_time / dt) + 1.0) as i32;
    for step in 0..n_steps {
        universe.do_time_step(dt);
        if step % 600 == 0 {
            universe.draw();
            let sleep_millis = time::Duration::from_millis(80);
            thread::sleep(sleep_millis);
        };
    }
    println!("Universe is now : {}",&universe);
}
                
            

