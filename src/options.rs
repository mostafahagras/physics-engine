#[derive(Debug)]
pub struct Options {
    pub gravity: f64,
    pub floor_enabled: bool,
}

impl Options {
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

    pub fn new(gravity: f64, floor_enabled: bool) -> Self {
        Self {
            gravity,
            floor_enabled,
        }
    }
}

impl Default for Options {
    fn default() -> Self {
        Self {
            gravity: Self::EARTH_GRAVITY,
            floor_enabled: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Options;

    #[test]
    fn options_new() {
        let consts = Options::new(Options::JUPITER_GRAVITY, false);
        assert_eq!(consts.gravity, Options::JUPITER_GRAVITY);
        assert_eq!(consts.floor_enabled, false);
    }

    #[test]
    fn options_default() {
        let consts = Options::default();
        assert_eq!(consts.gravity, Options::EARTH_GRAVITY);
        assert_eq!(consts.floor_enabled, true);
    }
}
