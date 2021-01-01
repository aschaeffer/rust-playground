use rand::Rng;

#[derive(Builder)]
pub struct Bounds {
    pub min: f64,
    pub max: f64,
}

impl Bounds {
    pub fn t(&self) -> (f64, f64) {
        (self.min, self.max)
    }

    /// Randomizes the min and max bound between -1.0 ... 1.0
    pub fn rnd(&mut self) {
        let mut rng = rand::thread_rng();
        self.min = rng.gen::<f64>() * 2.0 - 1.0;
        self.max = rng.gen::<f64>() * 2.0 - 1.0;
        if self.min > self.max {
            let tmp = self.min;
            self.min = self.max;
            self.max = tmp;
        }
    }
}
