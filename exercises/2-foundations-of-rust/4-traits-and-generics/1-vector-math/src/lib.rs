use std::ops::{Add, Mul};

#[derive(Copy, Clone)]
struct Vec2D {
    x: i32,
    y: i32,
}

impl Add for Vec2D {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        todo!();
    }
}

#[cfg(test)]
mod test {
    use crate::Vec2D;

    #[test]
    fn integer_addition() {
        let a = Vec2D { x: 1, y: 2 };
        let b = Vec2D { x: 3, y: 4 };
        let res = a + b;
        assert_eq!(res.x, a.x + b.x);
        assert_eq!(res.y, a.y + b.y);
    }

    // #[test]
    // fn integer_dot_product() {
    //     let a = Vec2D { x: 1, y: 2 };
    //     let b = Vec2D { x: 3, y: 4 };
    //     let res = a * b;
    //     assert_eq!(res, a.x * b.x + a.y * b.y);
    // }

    // #[test]
    // fn float_addition() {
    //     let a = Vec2D { x: 1.5, y: 2.5 };
    //     let b = Vec2D { x: 3.7, y: 4.2 };
    //     let res = a + b;
    //     assert_eq!(res.x, a.x + b.x);
    //     assert_eq!(res.y, a.y + b.y);
    // }

    // #[test]
    // fn float_dot_product() {
    //     let a = Vec2D { x: 1.5, y: 2.5 };
    //     let b = Vec2D { x: 3.7, y: 4.2 };
    //     let res = a * b;
    //     assert_eq!(res, a.x * b.x + a.y * b.y);
    // }
}
