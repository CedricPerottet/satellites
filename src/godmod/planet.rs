use super::physics::{Force, Position, Speed, UnitVector};
use std::{fmt, io};

#[derive(Debug)]
pub struct Planet {
    name: String,
    position: Position,
    speed: Speed,
    force: Force,
    mass: f64,
    radius: f64,
}

impl fmt::Display for Planet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Planet {
    pub fn get_name(&self) -> &String {
        &self.name
    }
    pub fn get_position(&self) -> &Position {
        &self.position
    }
    pub fn get_speed(&self) -> &Speed {
        &self.speed
    }
    pub fn get_mass(&self) -> &f64 {
        &self.mass
    }
    pub fn get_force(&self) -> &Force {
        &self.force
    }

    fn distance(&self, other_planet: &Planet) -> f64 {
        let dist_squared: f64 = (self.position.x - other_planet.position.x)
            * (self.position.x - other_planet.position.x)
            + (self.position.y - other_planet.position.y)
                * (self.position.y - other_planet.position.y);
        dist_squared.sqrt()
    }

    fn unit_vector_to(&self, other_planet: &Planet) -> UnitVector {
        let distance = self.distance(other_planet);
        UnitVector {
            x: (other_planet.position.x - self.position.x) / distance,
            y: (other_planet.position.y - self.position.y) / distance,
        }
    }

    pub fn crashes_on(&self, other_planet: &Planet) -> bool {
        let distance = self.distance(other_planet);
        if distance < other_planet.radius + self.radius {
            return true;
        }
        false
    }

    pub fn gravity_force_applied_by_planet(&self, other_planet: &Planet) -> io::Result<Force> {
        const G: f64 = 6.6743e-11;
        let distance = self.distance(other_planet);
        let force_norm = G * self.mass * other_planet.mass / (distance * distance);
        if distance == 0.0 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!(
                    "Distance between {} and {} is 0.0.",
                    self.name, other_planet.name
                ),
            ));
        }
        let unit_vector = self.unit_vector_to(other_planet);
        if self.crashes_on(other_planet) {
            Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!(
                    "Unhandled crash between {} and {}.",
                    self.name, other_planet.name
                ),
            ))
        } else {
            Ok(Force {
                x: unit_vector.x * force_norm,
                y: unit_vector.y * force_norm,
            })
        }
    }

    fn gravity_force_applied_by_planets(&self, other_planets: &Vec<Planet>) -> io::Result<Force> {
        let mut total_force = Force { x: 0., y: 0. };
        for other_planet in other_planets {
            if std::ptr::eq(other_planet, self) {
                continue;
            };
            // let force = self.gravity_force_applied_by_planet(other_planet)?;
            let force = self
                .gravity_force_applied_by_planet(other_planet)
                .unwrap_or(Force { x: 0., y: 0. });

            total_force.x += force.x;
            total_force.y += force.y;
        }
        Ok(total_force)
    }

    pub fn reset_force(&mut self) {
        self.force.x = 0.;
        self.force.y = 0.;
    }

    pub fn add_force_applied_by(&mut self, other_planet: &Planet) -> io::Result<()> {
        let force = self.gravity_force_applied_by_planet(other_planet)?;
        self.force.x += force.x;
        self.force.y += force.y;
        Ok(())
    }

    pub fn update_speed(&mut self, dt: f64) {
        self.speed.x += self.force.x / self.mass * dt;
        self.speed.y += self.force.y / self.mass * dt;
    }

    pub fn update_position(&mut self, dt: f64) {
        self.position.x += self.speed.x * dt;
        self.position.y += self.speed.y * dt;
    }

    pub fn planet_after_time_step(
        &self,
        dt: f64,
        other_planets: &Vec<Planet>,
    ) -> io::Result<Planet> {
        let external_force = self.gravity_force_applied_by_planets(other_planets)?;
        let new_speed = Speed {
            x: self.speed.x + external_force.x / self.mass * dt,
            y: self.speed.y + external_force.y / self.mass * dt,
        };
        let new_position = Position {
            x: self.position.x + new_speed.x * dt,
            y: self.position.y + new_speed.y * dt,
        };
        Ok(Planet {
            position: new_position,
            speed: new_speed,
            force: external_force,
            name: self.name.clone(),
            mass: self.mass,
            radius: self.radius,
        })
    }

    pub fn absorb(&mut self, other_planet: &mut Planet) {
        let plus = " + ".to_string();
        self.name.push_str(&plus);
        self.name.push_str(&other_planet.name);
        self.speed.x = (other_planet.mass * other_planet.speed.x + self.speed.x * self.mass)
            / (other_planet.mass + self.mass);
        self.speed.y = (other_planet.mass * other_planet.speed.y + self.speed.y * self.mass)
            / (other_planet.mass + self.mass);
        self.mass += other_planet.mass;
        self.radius = (other_planet.radius.powf(2.0) + self.radius.powf(2.0)).sqrt();
    }

    pub fn energy(&self) -> f64 {
        (self.mass as f64)
            * ((self.speed.x as f64) * (self.speed.x as f64)
                + (self.speed.y as f64) * (self.speed.y as f64))
    }

    pub fn new(
        name: String,
        pos_x: f64,
        pos_y: f64,
        spd_x: f64,
        spd_y: f64,
        mass: f64,
        radius: f64,
    ) -> io::Result<Planet> {
        if mass < 0.0 {
            Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("{} : mass should be positive, got {}", name, mass),
            ))
        } else if radius < 0.0 {
            Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("{} : radius should be positive, got {}.", name, radius),
            ))
        } else {
            Ok(Planet {
                name,
                position: Position { x: pos_x, y: pos_y },
                speed: Speed { x: spd_x, y: spd_y },
                force: Force { x: 0., y: 0. },
                mass,
                radius,
            })
        }
    }
}
