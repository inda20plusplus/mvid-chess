pub mod board;
pub mod game;
pub mod pieces;

#[derive(Copy, Clone, Hash, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub struct Point(pub i8, pub i8);

impl Point {
    pub fn add(&self, other: &Point) -> Point {
        Point(self.0 + other.0, self.1 + other.1)
    }

    pub fn index(&self) -> usize {
        ((self.1 - 1) * 8 + (self.0 - 1)) as usize
    }
}

trait Sorted {
    fn as_sorted(&self) -> Self;
}

impl Sorted for Vec<Point> {
    fn as_sorted(&self) -> Self {
        let mut clone = self.clone();
        clone.sort();
        clone
    }
}

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
pub enum Color {
    White,
    Black,
}

impl Color {
    fn inverse(&self) -> Self {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
    fn get_index(&self) -> usize {
        match self {
            Color::White => 0,
            Color::Black => 1,
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Move(Point, bool);

#[cfg(test)]
mod tests {
    use super::Point;

    #[test]
    fn test_point_addition() {
        assert_eq!(Point(3, 3).add(&Point(1, 2)), Point(4, 5));
        assert_eq!(Point(3, 3).add(&Point(-2, -1)), Point(1, 2));
    }
}
