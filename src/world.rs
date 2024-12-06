use crate::options::Options;
use std::time::Instant;

#[derive(Debug)]
pub struct World {
    pub options: Options,
    pub static_bodies: Vec<()>,
    pub dynamic_bodies: Vec<()>,
    pub time: f64,
    last_tick: Instant,
}

impl World {
    pub fn new(options: Options) -> Self {
        Self {
            options,
            static_bodies: vec![],
            dynamic_bodies: vec![],
            time: 0.0,
            last_tick: Instant::now(),
        }
    }

    pub fn tick(&mut self) {
        let dt = self.last_tick.elapsed();
        self.last_tick = Instant::now();
        self.time += dt.as_secs_f64();
    }
}

impl Default for World {
    fn default() -> Self {
        Self {
            options: Options::default(),
            static_bodies: Vec::new(),
            dynamic_bodies: Vec::new(),
            time: 0.0,
            last_tick: Instant::now(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn world_new() {
        let world = World::new(Options::new(Options::MOON_GRAVITY, true));
        assert_eq!(world.options.gravity, Options::MOON_GRAVITY);
    }

    #[test]
    fn world_default() {
        let world = World::default();
        assert_eq!(world.options.gravity, Options::EARTH_GRAVITY);
    }
}
