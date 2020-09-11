#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

mod board;
mod pieces;

struct Point(i8, i8);

pub enum Color {
    White,
    Black,
}
