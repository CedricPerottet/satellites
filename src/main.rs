use std::{thread, time};

#[derive(Debug)]
struct Planet{
    name : String,
    position : Position,
    speed : Speed,
    force : Force,
    mass : f32,
    radius : f32,
}

#[derive(Debug)]
struct Position{
    x : f32,
    y : f32
}

#[derive(Debug)]
struct Speed{
    x : f32,
    y : f32
}

#[derive(Debug)]
struct Force{
    x : f32,
    y : f32,
}

#[derive(Debug)]
struct UnitVector{
    x : f32,
    y : f32,
}

#[derive(Debug)]
struct Universe <'a>{
    planets : Vec<&'a mut Planet>,
}

impl Planet {
    fn distance(&self, other_planet: &Planet) -> f32 {
        let dist_squared : f32 = 
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
    

    fn gravity_force_applied_by(&self, other_planet: &Planet) -> Force {
        const G : f32 = 6.6743e-11;
        let distance = self.distance(other_planet);
        let force_norm = G * self.mass * other_planet.mass / (distance * distance);
        let unit_vector = self.unit_vector_to(other_planet);
        if distance < other_planet.radius + self.radius {
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

    fn update_speed(&mut self, dt : f32){
        self.speed.x += self.force.x / self.mass * dt;
        self.speed.y += self.force.y / self.mass * dt;
    }

    fn update_position(&mut self, dt : f32){
        self.position.x += self.speed.x * dt;
        self.position.y += self.speed.y * dt;
        println!();
    }
}

impl <'a> Universe<'a>{

    fn do_time_step(&mut self, dt:f32){
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
    }


    fn draw(&self){
        print!("\x1B[2J");
        let n_pixel_x = 50;
        let n_pixel_y = 30;
        let total_distance_window = 384000000. * 3.;
        let single_pixel_distance_x = total_distance_window / (n_pixel_x as f32);
        let single_pixel_distance_y = total_distance_window / (n_pixel_y as f32);
        for i_pixel_y in 0..n_pixel_y{
            print!("|");
            for i_pixel_x in 0..n_pixel_x{
                let mut found_planet = false;
                let min_x = (i_pixel_x as f32) * single_pixel_distance_x - total_distance_window / 2.;
                let max_x = ((i_pixel_x + 1) as f32) * single_pixel_distance_x - total_distance_window / 2.;
                let min_y = (i_pixel_y as f32) * single_pixel_distance_y - total_distance_window / 2.;
                let max_y = ((i_pixel_y + 1) as f32) * single_pixel_distance_y - total_distance_window / 2.;
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
    }

    fn remove(&mut self, planet_name : String){
        let index = self.planets.iter().position(|x| x.name == planet_name).unwrap();
        self.planets.remove(index);
    }
}


fn build_planet(name: String, pos_x: f32, pos_y: f32, spd_x: f32, spd_y: f32, mass: f32, radius: f32) -> Planet {
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
    let mut lune1 : Planet= build_planet("Lune".to_string(), 384000000., 0.0, 0.0, 1000.0, 7.36e22, 1.737e6);
    let mut lune2 : Planet= build_planet("Lune".to_string(), -384000000., 0.0, 0.0, 600.0, 7.36e22, 1.737e6);
    let mut lune3 : Planet= build_planet("Lune".to_string(), -184000000., 0.0, 0.0, -600.0, 7.36e22, 1.737e6);
    let mut lune4 : Planet= build_planet("Lune".to_string(), 184000000., 0.0, 0.0, -800.0, 7.36e22, 1.737e6);
    let mut lune5 : Planet= build_planet("Lune".to_string(), 584000000., 0.0, 0.0, -400.0, 7.36e22, 1.737e6);
    universe.planets.push(&mut terre);
    universe.planets.push(&mut lune1);
    universe.planets.push(&mut lune2);
    universe.planets.push(&mut lune3);
    universe.planets.push(&mut lune4);
    universe.planets.push(&mut lune5);

    // universe.remove("Terre".to_string());

    let dt = 10.; // [s]
    let total_simulation_time = 3600. * 24. * 60.; // [s]
    let n_steps = ((total_simulation_time / dt) + 1.0) as i32;
    for step in 0..n_steps {
        universe.do_time_step(dt);
        if step % 360 == 0 {
            universe.draw();
            let sleep_millis = time::Duration::from_millis(40);
            thread::sleep(sleep_millis);
        };
    }
}
                
            

