pub mod godmod;

#[cfg(test)]
mod tests {
    use std::io;

    use crate::godmod::planet::Planet;
    //use crate::godmod::universe::Universe;

    #[test]
    fn planet_constructor_args() {
        let result = Planet::new("Lune 1".to_string(), 0.0, 0.0, 0.0, 0.0, -7.36e22, 1.737e6);
        assert!(result.is_err());
        let result = Planet::new("Lune 1".to_string(), 0.0, 0.0, 0.0, 0.0, 7.36e22, -1.737e6);
        assert!(result.is_err());
        let result = Planet::new("Lune 1".to_string(), 0.0, 0.0, 0.0, 0.0, 7.36e22, 1.737e6);
        assert!(result.is_ok());
    }

    #[test]
    fn force_overflow() {
        let planet1 =
            Planet::new("Lune 1".to_string(), 0.0, 0.0, 0.0, 0.0, 7.36e22, 1e-200).unwrap();
        let planet2 =
            Planet::new("Lune 2".to_string(), 0.0, 1e-180, 0.0, 0.0, 7.36e22, 1e-200).unwrap();
        let result = planet1.gravity_force_applied_by_planet(&planet2);
        assert!(result.unwrap_err().kind() == io::ErrorKind::InvalidInput);
        let planet1 = Planet::new("Lune 1".to_string(), 0.0, 0.0, 0.0, 0.0, 7.36e22, 1e6).unwrap();
        let planet2 = Planet::new("Lune 2".to_string(), 0.0, 1e4, 0.0, 0.0, 7.36e22, 1e6).unwrap();
        let result = planet1.gravity_force_applied_by_planet(&planet2);
        assert!(result.unwrap_err().kind() == io::ErrorKind::InvalidInput);
    }
}
