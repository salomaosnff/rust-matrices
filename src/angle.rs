pub struct Angle(f64);

impl Angle {
    pub fn degrees(degrees: f64) -> Angle {
        Angle(degrees * std::f64::consts::PI / 180.0)
    }

    pub fn radians(radians: f64) -> Angle {
        Angle(radians)
    }

    pub fn sin(&self) -> f64 {
        self.0.sin()
    }

    pub fn cos(&self) -> f64 {
        self.0.cos()
    }

    pub fn to_degrees(&self) -> f64 {
        self.0 * 180.0 / std::f64::consts::PI
    }

    pub fn to_radians(&self) -> f64 {
        self.0
    }
}
