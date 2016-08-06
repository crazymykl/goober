use {Point, MU, WIDTH, HEIGHT};

#[derive(Clone)]
pub struct Entity {
    pos: Point,
    pub color: [f32; 4],
    dv: Point,
    width: f64,
    height: f64,
}

impl Entity {
    pub fn new(pos: Point, color: [f32; 4], w: f64, h: f64) -> Self {
        Entity {
            pos: pos,
            color: color,
            dv: Point(0.0, 0.0),
            width: w,
            height: h,
        }
    }

    pub fn geometry(&self) -> [f64; 4] {
        [self.pos.0, self.pos.1, self.width, self.height]
    }

    pub fn nudge(&mut self) {
        self.pos.0 += self.dv.0;
        self.pos.1 += self.dv.1;

        self.dv.0 *= MU;
        self.dv.1 *= MU;

        self.clamp();
    }

    pub fn adjust_dx(&mut self, dx: f64) {
        self.dv.0 += dx;
    }

    pub fn adjust_dy(&mut self, dy: f64) {
        self.dv.1 += dy;
    }

    fn clamp(&mut self) {
        let (w, h) = (self.max_width(), self.max_height());

        if self.pos.0 < 0.0 { self.pos.0 = 0.0 };
        if self.pos.0 > w   { self.pos.0 = w };
        if self.pos.1 < 0.0 { self.pos.1 = 0.0 };
        if self.pos.1 > h   { self.pos.1 = h };
    }

    fn max_height(&self) -> f64 {
        HEIGHT as f64 - self.height
    }

    fn max_width(&self) -> f64 {
        WIDTH as f64 - self.width
    }

}
