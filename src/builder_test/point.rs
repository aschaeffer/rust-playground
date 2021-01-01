use rand::Rng;

#[derive(Builder)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point {
    pub fn t(&self) -> (f64, f64, f64) {
        (self.x, self.y, self.z)
    }

    /// Randomizes the point in all three coordinates between -1.0 ... 1.0
    pub fn rnd(&mut self) {
        let mut rng = rand::thread_rng();
        self.x = rng.gen::<f64>() * 2.0 - 1.0;
        self.y = rng.gen::<f64>() * 2.0 - 1.0;
        self.z = rng.gen::<f64>() * 2.0 - 1.0;
    }
}
