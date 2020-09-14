mod board;
mod pieces;

#[derive(Clone, Hash, Eq, PartialEq, Ord, PartialOrd, Debug)]
struct Point(i8, i8);

impl Point {
    fn add(&self, other: &Point) -> Point {
        Point(self.0 + other.0, self.1 + other.1)
    }
}

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
pub enum Color {
    White,
    Black,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Move(Point, bool);

#[cfg(test)]
mod tests {
    use super::Point;

    fn test_point_addition() {
        assert_eq!(
            Point(3, 3).add(&Point(1, 2)),
            Point(4, 5)
        );
        assert_eq!(
            Point(3, 3).add(&Point(-1, -1)),
            Point(2, 2)
        );
    }
}

