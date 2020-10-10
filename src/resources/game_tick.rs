// A GameTick determines the rate of systems that run on the fixed game schedule
pub struct GameTick {
    pub rate: f64,
    pub tick: u64,
}

impl Default for GameTick {
    fn default() -> Self {
        GameTick {
            rate: 1.0 / 10.0,
            tick: 0,
        }
    }
}

impl GameTick {
    // rate: ticks per second
    pub fn new(rate: f64) -> Self {
        GameTick {
            rate: 1.0 / rate,
            tick: 0,
        }
    }
}