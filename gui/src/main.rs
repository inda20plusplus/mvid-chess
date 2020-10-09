use chess::game;
use chess::pieces;
use ggez;
use ggez::event;
use ggez::graphics;
use ggez::nalgebra as na;
use std::path;
mod network;
mod screen;
use std::sync::{Arc, Mutex};
pub const WINDOW_SIZE: (f32, f32) = (1200.0, 900.0);
#[derive(Debug, Clone)]
pub enum Piece {
    King(Color),
    Queen(Color),
    Rook(Color),
    Pawn(Color),
    Knight(Color),
    Bishop(Color),
    None,
}
pub enum State {
    Checkmate,
    Remi,
    Playing { promotion: bool, check: bool },
    None,
}
pub struct Moves(Position, Vec<Position>);
#[derive(Clone)]
pub enum Overlay {
    Moves {
        selected: Position,
        to: Vec<Position>,
    },
    None,
}
#[derive(Debug, Clone, Copy)]
pub struct Position(usize, usize);
impl Position {
    pub fn translate(&mut self) -> chess::Point {
        chess::Point((self.0 + 1) as i8, (self.1 + 1) as i8)
    }
    pub fn new(pos: &chess::Point) -> Self {
        Position((pos.0 - 1) as usize, (pos.1 - 1) as usize)
    }
}
pub struct Board(Vec<(Piece, Position)>);
#[derive(PartialEq, Clone, Debug)]
pub enum Color {
    Black,
    White,
    None,
}
pub struct StatePtr {
    pub state: Arc<Mutex<MainState>>
}
pub struct MainState {
    pub game: game::Game,
    pub board: Board,
    pub turn: Color,
    pub state: State,
    pub help: Overlay,
    pub selected: Selected,
    pub connection: network::Connection,
    pub my_color: Color,
}
#[derive(Clone)]
pub enum Selected {
    Position(Position),
    None,
}
pub const MYCOLOR: Color = Color::Black;
impl MainState {
    fn new() -> ggez::GameResult<MainState> {
        let mut s = MainState {
            game: game::Game::new(),
            board: Board(vec![]),
            turn: Color::White,
            state: State::Playing {
                promotion: false,
                check: false,
            },
            help: Overlay::None,
            selected: Selected::None,
            connection: network::Connection::init(MYCOLOR),
            my_color: MYCOLOR,
        };
        s.parse();
        Ok(s)
    }
    fn parse(&mut self) {
        self.board = Board(vec![]);
        for i in 1..=8 {
            for j in 1..=8 {
                let map_point = chess::Point(i, j);
                let position = Position::new(&map_point);
                if let Some(cur) = self.game.get_board().at_point(&map_point) {
                    let color = match cur.color {
                        chess::Color::White => Color::White,
                        chess::Color::Black => Color::Black,
                    };
                    let piece = match cur.kind {
                        chess::pieces::Kind::King => Piece::King(color),
                        chess::pieces::Kind::Knight => Piece::Knight(color),
                        chess::pieces::Kind::Rook => Piece::Rook(color),
                        chess::pieces::Kind::Pawn => Piece::Pawn(color),
                        chess::pieces::Kind::Queen => Piece::Queen(color),
                        chess::pieces::Kind::Bishop => Piece::Bishop(color),
                    };
                    self.board.0.push((piece, position));
                }
            }
        }
        self.turn = match self.game.color {
            chess::Color::Black => Color::Black,
            chess::Color::White => Color::White,
        };
    }
}
#[derive(Debug)]
pub enum ButtonType {
    Promotion(Piece),
    LCastling,
    SCastling,
}
#[derive(Debug)]
pub enum Element {
    Tile(Position),
    Button(ButtonType),
    None,
}
pub struct Box(f32, f32, f32, f32);
impl Box {
    pub fn selected(&mut self, point: &mut (f32, f32)) -> bool {
        if point.0 > self.0 && point.0 < self.1 && point.1 > self.2 && point.1 < self.3 {
            point.0 -= self.0;
            point.1 -= self.2;
            true
        } else {
            false
        }
    }
}
fn get_element(point: &mut (f32, f32)) -> Element {
    let mut board = Box(50.0, 850.0, 50.0, 850.0);
    if board.selected(point) {
        return Element::Tile(Position(
            (point.0 / 100.0) as usize,
            7 - (point.1 / 100.0) as usize,
        ));
    }
    let mut promotions = Box(950.0, 1150.0, 250.0, 450.0);
    if promotions.selected(point) {
        return match point {
            (x, y) if *x < 100.0 && *y < 100.0 => {
                Element::Button(ButtonType::Promotion(Piece::Bishop(Color::None)))
            }
            (x, y) if *x < 100.0 => {
                Element::Button(ButtonType::Promotion(Piece::Queen(Color::None)))
            }
            (x, y) if *y < 100.0 => {
                Element::Button(ButtonType::Promotion(Piece::Knight(Color::None)))
            }
            _ => Element::Button(ButtonType::Promotion(Piece::Rook(Color::None))),
        };
    }
    Element::None
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut ggez::Context) -> ggez::GameResult {
        if self.my_color != self.turn {
            self.connection.get();
        };
        Ok(())
    }
    fn mouse_button_down_event(
        &mut self,
        ctx: &mut ggez::Context,
        button: ggez::event::MouseButton,
        x: f32,
        y: f32,
    ) {
        {
            let mut val = self.connection.tx.lock().unwrap();
            val.push(1);
        };
        match get_element(&mut (x, y)) {
            Element::Tile(mut pos) => match self.selected.clone() {
                Selected::None => {
                    self.selected = Selected::Position(pos);
                    if let Some(res) = self.game.get_moves(&(pos.clone()).translate()) {
                        let mut to = Vec::<Position>::with_capacity(res.len());
                        for i in res.iter() {
                            to.push(Position::new(&i));
                        }
                        self.help = Overlay::Moves { selected: pos, to };
                    }
                }
                Selected::Position(position) => {
                    if self.turn == self.my_color {
                        let state = self
                            .game
                            .turn(position.clone().translate(), pos.translate());

                        match state {
                            game::TurnResult::Promotion => {
                                self.state = State::Playing {
                                    promotion: true,
                                    check: false,
                                }
                            }
                            game::TurnResult::Checked => {
                                self.state = State::Playing {
                                    promotion: false,
                                    check: true,
                                }
                            }
                            game::TurnResult::Moved => {
                                self.state = State::Playing {
                                    promotion: false,
                                    check: false,
                                }
                            }
                            game::TurnResult::GameEnd(_) => self.state = State::Checkmate,
                            _ => (),
                        };
                        self.connection.push();
                    }
                    self.parse();
                    self.selected = Selected::None;
                    self.help = Overlay::None;
                }
            },
            Element::Button(ButtonType::Promotion(piece)) => {
                self.game.promote(match piece {
                    Piece::Queen(_) => pieces::Kind::Queen,
                    Piece::Rook(_) => pieces::Kind::Rook,
                    Piece::Knight(_) => pieces::Kind::Knight,
                    Piece::Bishop(_) => pieces::Kind::Bishop,
                    _ => pieces::Kind::King,
                });

                self.state = State::Playing {
                    promotion: false,
                    check: false,
                };
            }
            _ => {
                self.selected = Selected::None;
                self.help = Overlay::None;
            }
        }
    }
    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        graphics::clear(ctx, [0.0, 0.0, 0.0, 1.0].into());
        screen::playing::playing(self, ctx);
        graphics::present(ctx)?;
        Ok(())
    }
}

pub fn main() -> ggez::GameResult {
    let resource_dir = path::PathBuf::from("./gui/src/resources");
    let cb = ggez::ContextBuilder::new("super_simple", "ggez")
        .window_setup(ggez::conf::WindowSetup::default().title("Best chess game!"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(WINDOW_SIZE.0, WINDOW_SIZE.1))
        .add_resource_path(resource_dir);
    let (ctx, event_loop) = &mut cb.build()?;
    let state = &mut MainState::new()?;
    event::run(ctx, event_loop, state)
}
