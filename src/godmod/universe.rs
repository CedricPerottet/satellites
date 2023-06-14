use super::planet::Planet;
use std::{fmt, thread, time};

#[derive(Debug)]
pub struct Universe {
    pub planets: Vec<Planet>,
}

impl Universe {
    pub fn do_time_step(&mut self, dt: f64) {
        // for planet in self.planets {}

        let mut planets_after_timestep = Vec::new();
        for planet in &self.planets {
            planets_after_timestep.push(planet.planet_after_time_step(dt, &self.planets));
        }
        self.planets = planets_after_timestep;

        let mut planets_crashs = Vec::new();
        let mut planets_freed = Vec::new();
        for i_planet in 0..self.planets.len() {
            for j_planet in i_planet + 1..self.planets.len() {
                if self.planets[i_planet].crashes_on(&self.planets[j_planet]) {
                    let (planet_i, planet_j) = {
                        let (left, right) = self.planets.split_at_mut(j_planet);
                        (&mut left[i_planet], &mut right[0])
                    };
                    if planet_i.get_mass() >= planet_j.get_mass() {
                        planet_i.absorb(planet_j);
                        planets_crashs.push(j_planet);
                    } else {
                        planet_j.absorb(planet_i);
                        planets_crashs.push(i_planet);
                    }
                    if planet_i.get_force().x < 100. && planet_i.get_force().y < 100. {
                        planets_freed.push(i_planet);
                    }
                }
            }
        }
        for i_planet in planets_crashs.iter().rev() {
            let planet_to_remove = self.planets[*i_planet].get_name().clone();
            println!("{} has been destroyed in the impact...", planet_to_remove);
            self.remove(planet_to_remove);
            let sleep_millis = time::Duration::from_millis(1000);
            thread::sleep(sleep_millis);
        }
        if planets_crashs.len() == 0 && self.planets.len() > 2 {
            for i_planet in planets_freed.iter().rev() {
                let planet_to_remove = self.planets[*i_planet].get_name().clone();
                println!("{} has left gravity field...", planet_to_remove);
                self.remove(planet_to_remove);
                println!("Universe is now : {}.", self);
                let sleep_millis = time::Duration::from_millis(5000);
                thread::sleep(sleep_millis);
            }
        }
    }

    pub fn draw(&self) {
        print!("\x1B[2J");
        let n_pixel_x = 50;
        let n_pixel_y = 30;
        let total_distance_window = 384000000. * 5.;
        let single_pixel_distance_x = total_distance_window / (n_pixel_x as f64);
        let single_pixel_distance_y = total_distance_window / (n_pixel_y as f64);
        for i_pixel_y in 0..n_pixel_y {
            print!("|");
            for i_pixel_x in 0..n_pixel_x {
                let mut found_planet = false;
                let min_x =
                    (i_pixel_x as f64) * single_pixel_distance_x - total_distance_window / 2.;
                let max_x =
                    ((i_pixel_x + 1) as f64) * single_pixel_distance_x - total_distance_window / 2.;
                let min_y =
                    (i_pixel_y as f64) * single_pixel_distance_y - total_distance_window / 2.;
                let max_y =
                    ((i_pixel_y + 1) as f64) * single_pixel_distance_y - total_distance_window / 2.;
                for i_planet in 0..self.planets.len() {
                    if self.planets[i_planet].get_position().x >= min_x
                        && self.planets[i_planet].get_position().x < max_x
                        && self.planets[i_planet].get_position().y >= min_y
                        && self.planets[i_planet].get_position().y < max_y
                    {
                        found_planet = true;
                    }
                }
                if found_planet {
                    print!("O");
                } else {
                    print!(" ");
                }
            }
            print!("|");
            print!("\n");
        }
        println!("Universe total kinetic energy = {:.2e}", self.energy());
    }

    fn remove(&mut self, planet_name: String) {
        let index = self
            .planets
            .iter()
            .position(|x| *x.get_name() == *planet_name)
            .unwrap();
        self.planets.remove(index);
    }

    fn energy(&self) -> f64 {
        let mut energy = 0.;
        for i_planet in 0..self.planets.len() {
            energy += self.planets[i_planet].energy();
        }
        energy
    }
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[")?;
        for (count, planet) in self.planets.iter().enumerate() {
            if count != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", planet)?;
        }
        write!(f, "]")
    }
}
