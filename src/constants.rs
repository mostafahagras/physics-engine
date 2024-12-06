#[derive(Debug)]
pub struct Constants {
    pub gravity: f64,
}

impl Constants {
    pub const EARTH_GRAVITY: f64 = -9.807;
    pub const MOON_GRAVITY: f64 = -1.62;
    pub const MERCURY_GRAVITY: f64 = -3.7;
    pub const VENUS_GRAVITY: f64 = -8.87;
    pub const MARS_GRAVITY: f64 = -3.71;
    pub const JUPITER_GRAVITY: f64 = -24.79;
    pub const SATURN_GRAVITY: f64 = -10.44;
    pub const URANUS_GRAVITY: f64 = -8.69;
    pub const NEPTUNE_GRAVITY: f64 = -11.15;
    pub const PLUTO_GRAVITY: f64 = -0.62;

    pub fn new(gravity: f64) -> Self {
        Self { gravity }
    }
}

impl Default for Constants {
    fn default() -> Self {
        Self {
            gravity: Self::EARTH_GRAVITY,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Constants;

    #[test]
    fn constants_new() {
        let consts = Constants::new(Constants::JUPITER_GRAVITY);
        assert_eq!(consts.gravity, Constants::JUPITER_GRAVITY);
    }
    
    #[test]
    fn constants_default() {
        let consts = Constants::default();
        assert_eq!(consts.gravity, Constants::EARTH_GRAVITY);
    }
}