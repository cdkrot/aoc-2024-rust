extern crate derive_more;
use derive_more::{Add, AddAssign, Display, Neg, Sub, SubAssign};

#[derive(Add, Sub, AddAssign, SubAssign, Copy, Clone, Debug, PartialEq, Eq, Display, Neg, Hash)]
#[display("({x}, {y})")]
pub(crate) struct Vec2d {
    pub(crate) x: i32,
    pub(crate) y: i32,
}

impl Vec2d {
    pub(crate) fn new(x: i32, y: i32) -> Vec2d {
        Vec2d { x, y }
    }

    pub(crate) fn len2(self) -> i64 {
        let x = self.x as i64;
        let y = self.y as i64;
        x * x + y * y
    }

    pub(crate) fn scale(self, c: i32) -> Vec2d {
        Vec2d::new(c * self.x, c * self.y)
    }

    pub(crate) fn to_upperplane(self) -> Vec2d {
        if (self.x, self.y) < (0, 0) {
            return -self;
        }
        return self;
    }
}
