mod board;
mod pieces;

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
struct Point(i8, i8);

#[derive(PartialEq, Debug)]
pub enum Color {
    White,
    Black,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Move(Point, bool);
