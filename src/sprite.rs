#[derive(Clone,Copy,PartialEq,Debug)]
pub struct SpriteInfo {
    pub first: (f64, f64), //left corner of first
    pub num_in_row: u16,
    pub num_in_col: u16,
    pub w_h: (f64, f64),
    pub pad: (f64, f64, f64, f64),
}
impl Spriteable for SpriteInfo {
    fn first(&self) -> (f64, f64) {
        self.first
    }
    fn num_in_row(&self) -> u16 {
        self.num_in_row
    }
    fn num_in_col(&self) -> u16 {
        self.num_in_col
    }
    fn w_h(&self) -> (f64, f64) {
        self.w_h
    }
    fn pad(&self) -> (f64, f64, f64, f64) {
        self.pad
    }
}
pub trait Spriteable {
    fn first(&self) -> (f64, f64);
    fn num_in_row(&self) -> u16;
    fn num_in_col(&self) -> u16;
    fn w_h(&self) -> (f64, f64);
    fn pad(&self) -> (f64, f64, f64, f64);
}
pub fn spriteable_rect<H: Spriteable>(s: H, index: f64) -> ([f64; 2], [f64; 2]) {
    let (x, y) = (index % s.num_in_row() as f64, (index / (s.num_in_row() as f64)).floor());
    ([s.first().0 + x * s.w_h().0 + s.pad().0, s.first().1 - y * s.w_h().1 - s.pad().2],
     [s.first().0 + (x + 1.0) * s.w_h().0 - s.pad().1,
      s.first().1 - (y + 1.0) * s.w_h().1 + s.pad().3])
}
