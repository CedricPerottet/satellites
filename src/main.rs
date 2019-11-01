use std::{thread, time};

#[derive(Debug)]
struct Planet{
    name : String,
    position : Position,
    speed : Speed,
    force : Force,
    mass : f64,
    radius : f64,
}

#[derive(Debug)]
struct Position{
    x : f64,
    y : f64
}

#[derive(Debug)]
struct Speed{
    x : f64,
    y : f64
}

#[derive(Debug)]
struct Force{
    x : f64,
    y : f64,
}

#[derive(Debug)]
struct UnitVector{
    x : f64,
    y : f64,
}

#[derive(Debug)]
struct Universe <'a>{
    planets : Vec<&'a mut Planet>,
}

impl Planet {
    fn distance(&self, other_planet: &Planet) -> f64 {
        let dist_squared : f64 = 
             (self.position.x - other_planet.position.x) * (self.position.x - other_planet.position.x)
            +(self.position.y - other_planet.position.y) * (self.position.y - other_planet.position.y);
        dist_squared.sqrt()
    }

    fn unit_vector_to(&self, other_planet: &Planet) -> UnitVector {
        let distance = self.distance(other_planet);
        UnitVector { 
            x : (other_planet.position.x - self.position.x) / distance,
            y : (other_planet.position.y - self.position.y) / distance,
        }
    }
    
    fn crashes_on(&self, other_planet: &Planet) -> bool{
        let distance = self.distance(other_planet);
        if distance < other_planet.radius + self.radius{
            return true;
        }
        false
    }

    fn gravity_force_applied_by(&self, other_planet: &Planet) -> Force {
        const G : f64 = 6.6743e-11;
        let distance = self.distance(other_planet);
        let force_norm = G * self.mass * other_planet.mass / (distance * distance);
        let unit_vector = self.unit_vector_to(other_planet);
        if self.crashes_on(other_planet) {
            println!("Unhandled crash between {} and {}. Set zero interaction.",self.name,other_planet.name);
            Force{
                x : 0.,
                y : 0.,
            }
        }
        else{
            Force{
                x : unit_vector.x * force_norm,
                y : unit_vector.y * force_norm,
            }
        }
    }

    fn reset_force(&mut self){
        self.force.x = 0.;
        self.force.y = 0.;
    }

    fn add_force_applied_by(&mut self, other_planet: &Planet){
        let force = self.gravity_force_applied_by(other_planet);
        self.force.x += force.x;
        self.force.y += force.y;
    }

    fn update_speed(&mut self, dt : f64){
        self.speed.x += self.force.x / self.mass * dt;
        self.speed.y += self.force.y / self.mass * dt;
    }

    fn update_position(&mut self, dt : f64){
        self.position.x += self.speed.x * dt;
        self.position.y += self.speed.y * dt;
        println!();
    }

    fn absorb(&mut self, other_planet : &mut Planet){
        self.name.push_str(&other_planet.name);
        self.speed.x = (other_planet.mass * other_planet.speed.x + self.speed.x * self.mass) / (other_planet.mass + self.mass);
        self.speed.y = (other_planet.mass * other_planet.speed.y + self.speed.y * self.mass) / (other_planet.mass + self.mass);
        self.mass += other_planet.mass;
    }

    fn energy(&self) -> f64{
        (self.mass as f64) * ((self.speed.x as f64) * (self.speed.x as f64) + (self.speed.y as f64) * (self.speed.y as f64))
    }
}

impl <'a> Universe<'a>{

    fn do_time_step(&mut self, dt:f64){
        for i_planet in 0..self.planets.len() {
            self.planets[i_planet].reset_force();
            for j_planet in 0..self.planets.len() {
                let (planet_i, planet_j) = if i_planet < j_planet {
                // `i` is in the left half
                    let (left, right) = self.planets.split_at_mut(j_planet);
                    (&mut left[i_planet], &mut right[0])
                } else if i_planet == j_planet {
                // cannot obtain two mutable references to the
                // same element
                continue;
                } else {
                    // `i` is in the right half
                    let (left, right) = self.planets.split_at_mut(i_planet);
                    (&mut right[0], &mut left[j_planet])
                };
                planet_i.add_force_applied_by(planet_j);
            }
        }
        for i_planet in 0..self.planets.len() {
            self.planets[i_planet].update_speed(dt);
            self.planets[i_planet].update_position(dt); 
        }

        let mut planets_crashs = Vec::new();
        let mut planets_freed = Vec::new();
        for i_planet in 0..self.planets.len() {
            for j_planet in i_planet+1..self.planets.len(){
                if self.planets[i_planet].crashes_on(self.planets[j_planet]){
                    let (planet_i, planet_j) = {
                        let (left, right) = self.planets.split_at_mut(j_planet);
                        (&mut left[i_planet], &mut right[0])
                    };
                    if planet_i.mass >= planet_j.mass {
                        planet_i.absorb(planet_j);
                        planets_crashs.push(j_planet);
                    }
                    else {
                        planet_j.absorb(planet_i);
                        planets_crashs.push(i_planet);
                    }
                    if planet_i.force.x < 100. && planet_i.force.y < 100.{
                        planets_freed.push(i_planet);
                    }
                }
            }
        }
        for i_planet in planets_crashs.iter().rev() {
            let planet_to_remove = self.planets[*i_planet].name.clone();
            println!("{} has been destroyed in the impact...",planet_to_remove);
            self.remove(planet_to_remove);
            let sleep_millis = time::Duration::from_millis(1000);
            thread::sleep(sleep_millis);
        }
        if planets_crashs.len() == 0 && self.planets.len() > 2{
            for i_planet in planets_freed.iter().rev() {
                let planet_to_remove = self.planets[*i_planet].name.clone();
                println!("{} has left gravity field...",planet_to_remove);
                self.remove(planet_to_remove);
                println!("Universe is now : {:?}.",self);
                let sleep_millis = time::Duration::from_millis(5000);
                thread::sleep(sleep_millis);
            }
        }
    }


    fn draw(&self){
        print!("\x1B[2J");
        let n_pixel_x = 50;
        let n_pixel_y = 30;
        let total_distance_window = 384000000. * 5.;
        let single_pixel_distance_x = total_distance_window / (n_pixel_x as f64);
        let single_pixel_distance_y = total_distance_window / (n_pixel_y as f64);
        for i_pixel_y in 0..n_pixel_y{
            print!("|");
            for i_pixel_x in 0..n_pixel_x{
                let mut found_planet = false;
                let min_x = (i_pixel_x as f64) * single_pixel_distance_x - total_distance_window / 2.;
                let max_x = ((i_pixel_x + 1) as f64) * single_pixel_distance_x - total_distance_window / 2.;
                let min_y = (i_pixel_y as f64) * single_pixel_distance_y - total_distance_window / 2.;
                let max_y = ((i_pixel_y + 1) as f64) * single_pixel_distance_y - total_distance_window / 2.;
                for i_planet in 0..self.planets.len() {
                    if self.planets[i_planet].position.x >= min_x && 
                       self.planets[i_planet].position.x < max_x  &&
                       self.planets[i_planet].position.y >= min_y &&
                       self.planets[i_planet].position.y < max_y 
                    {
                        found_planet = true;
                    }
                }
                if found_planet {
                    print!("O");
                }
                else{
                    print!(" ");
                }
            }
            print!("|");
            print!("\n");
        }
        println!("Universe total kinetic energy = {:e}", self.energy());
    }

    fn remove(&mut self, planet_name : String){
        let index = self.planets.iter().position(|x| x.name == *planet_name).unwrap();
        self.planets.remove(index);
    }

    fn energy(&self) -> f64{
        let mut energy = 0.;
        for i_planet in 0..self.planets.len() {
            energy += self.planets[i_planet].energy();
        }
        energy
    }
}


fn build_planet(name: String, pos_x: f64, pos_y: f64, spd_x: f64, spd_y: f64, mass: f64, radius: f64) -> Planet {
    Planet {
        name,
        position : Position{x : pos_x, y : pos_y},
        speed : Speed{x : spd_x, y : spd_y},
        force : Force{x : 0., y : 0.},
        mass,
        radius,
    }
}




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
    let total_simulation_time = 3600. * 24. * 720.; // [s]
    let n_steps = ((total_simulation_time / dt) + 1.0) as i32;
    for step in 0..n_steps {
        universe.do_time_step(dt);
        if step % 600 == 0 {
            universe.draw();
            let sleep_millis = time::Duration::from_millis(80);
            thread::sleep(sleep_millis);
        };
    }
    println!("Universe is now : {:?}.",&universe);
}
                
            

