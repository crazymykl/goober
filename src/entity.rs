use point::Point;
use {MU, WIDTH, HEIGHT};

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
            dv: Point{x: 0.0, y: 0.0},
            width: w,
            height: h,
        }
    }

    pub fn geometry(&self) -> [f64; 4] {
        [self.pos.x, self.pos.y, self.width, self.height]
    }

    pub fn nudge(&mut self) {
        self.pos.x += self.dv.x;
        self.pos.y += self.dv.y;

        self.dv.x *= MU;
        self.dv.y *= MU;

        self.clamp();
    }

    pub fn adjust_dx(&mut self, dx: f64) {
        self.dv.x += dx;
    }

    pub fn adjust_dy(&mut self, dy: f64) {
        self.dv.y += dy;
    }

    fn clamp(&mut self) {
        let (w, h) = (self.max_width(), self.max_height());

        if self.pos.x < 0.0 { self.pos.x = 0.0 };
        if self.pos.x > w   { self.pos.x = w };
        if self.pos.y < 0.0 { self.pos.y = 0.0 };
        if self.pos.y > h   { self.pos.y = h };
    }

    fn max_height(&self) -> f64 {
        HEIGHT as f64 - self.height
    }

    fn max_width(&self) -> f64 {
        WIDTH as f64 - self.width
    }

}
