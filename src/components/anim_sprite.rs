pub struct AnimSprite {
    pub rate: f64,
    pub acc: f64,
}

impl AnimSprite {
    pub fn new(rate: f64) -> AnimSprite {
        AnimSprite {
            rate,
            acc: 0.0,
        }
    }
}