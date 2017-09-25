extern crate num;

use num::Num;

pub struct Triangle<N>(N, N, N);

impl<N> Triangle<N>
where
    N: Num + Copy + PartialOrd,
{
    pub fn build(
        sides: [N; 3],
    ) -> Result<Self, &'static str> {
        if sides.iter().any(|side| side.is_zero()) ||
            !Triangle::valid(&sides)
        {
            Err("Invalid triangle")
        } else {
            Ok(Triangle(sides[0], sides[1], sides[2]))
        }
    }

    pub fn is_equilateral(&self) -> bool {
        let &Triangle(a, b, c) = self;
        a == b && b == c
    }

    pub fn is_isosceles(&self) -> bool {
        let &Triangle(a, b, c) = self;
        a == b || b == c || a == c
    }

    pub fn is_scalene(&self) -> bool {
        let &Triangle(a, b, c) = self;
        a != b && b != c && a != c
    }

    fn valid(sides: &[N; 3]) -> bool {
        let (a, b, c) = (sides[0], sides[1], sides[2]);
        (a + b >= c) && (a + c >= b) && (b + c >= a)
    }
}
