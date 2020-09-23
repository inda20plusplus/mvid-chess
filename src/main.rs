use chess::game;
use ggez;
use ggez::event;
use ggez::graphics;
use ggez::nalgebra as na;
use std::path;
mod screen;
#[derive(Clone)]
enum Piece {
    King(Color),
    Queen(Color),
    Rook(Color),
    Pawn(Color),
    Knight(Color),
    Bishop(Color),
    None,
}
enum State {
    Checkedmate,
    Remi,
    Playing{promotion: bool, check: bool},
    None,
}
struct Moves(Position, Vec<Position>);
#[derive(Clone)]
enum Overlay {
    Moves {
        selected: Position,
        to: Vec<Position>,
    },
    None,
}
#[derive(Debug, Clone, Copy)]
struct Position(usize, usize);
impl Position {
    pub fn translate(&mut self) -> chess::Point {
        chess::Point((self.0 + 1) as i8, (self.1 + 1) as i8)
    }
    pub fn new(pos: &chess::Point) -> Self {
        Position((pos.0 - 1) as usize, (pos.1 - 1) as usize)
    }
}
struct Board(Vec<(Piece, Position)>);
#[derive(PartialEq, Clone)]
enum Color {
    Black,
    White,
}
struct MainState {
    pub game: game::Game,
    pub board: Board,
    pub turn: Color,
    pub state: State,
    pub help: Overlay,
    pub selected: Selected,
}
#[derive(Clone)]
enum Selected {
    Position(Position),
    None,
}
impl MainState {
    fn new() -> ggez::GameResult<MainState> {
        let mut s = MainState {
            game: game::Game::new(),
            board: Board(vec![]),
            turn: Color::White,
            state: State::Playing{promotion: false, check: false},
            help: Overlay::None,
            selected: Selected::None,
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
                if self.game.get_board().contains_key(&map_point) {
                    let cur = self.game.get_board()[&map_point].clone();
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

enum Element {
    Tile(Position),
    None,
}
fn board_edge(ctx: &mut ggez::Context) -> graphics::Mesh {
    graphics::Mesh::new_rectangle(
        ctx,
        graphics::DrawMode::fill(),
        graphics::Rect {
            x: 0.0,
            y: 0.0,
            w: 1200.0,
            h: 900.0,
        },
        graphics::Color::from_rgb(0, 0, 66),
    )
    .unwrap()
}
fn text(ctx: &mut ggez::Context, x: f32, y: f32, to_draw: &str) {
    let font = graphics::Font::new(ctx, "/DejaVuSansMono.ttf");
    let text = graphics::Text::new((to_draw, font.unwrap(), 48.0));
    let dest_point = na::Point2::new(x, y);
    graphics::draw(ctx, &text, (dest_point,));
}
fn draw_piece(ctx: &mut ggez::Context, position: Position, piece: Piece) -> () {
    let mut path;
    match piece {
        Piece::King(color) => {
            path = if color == Color::White {
                "/pieces/Chess_klt60.png"
            } else {
                "/pieces/Chess_kdt60.png"
            }
        }
        Piece::Queen(color) => {
            path = if color == Color::White {
                "/pieces/Chess_qlt60.png"
            } else {
                "/pieces/Chess_qdt60.png"
            }
        }
        Piece::Rook(color) => {
            path = if color == Color::White {
                "/pieces/Chess_rlt60.png"
            } else {
                "/pieces/Chess_rdt60.png"
            }
        }
        Piece::Pawn(color) => {
            path = if color == Color::White {
                "/pieces/Chess_plt60.png"
            } else {
                "/pieces/Chess_pdt60.png"
            }
        }
        Piece::Bishop(color) => {
            path = if color == Color::White {
                "/pieces/Chess_blt60.png"
            } else {
                "/pieces/Chess_bdt60.png"
            }
        }
        Piece::Knight(color) => {
            path = if color == Color::White {
                "/pieces/Chess_nlt60.png"
            } else {
                "/pieces/Chess_ndt60.png"
            }
        }
        Piece::None => return,
    }
    let image = graphics::Image::new(ctx, path).unwrap();
    graphics::draw(
        ctx,
        &image,
        (na::Point2::new(
            70.0 + 100.0 * position.0 as f32,
            70.0 + 100.0 * (7 - position.1) as f32,
        ),),
    );
}
fn get_element(mut x: f32, mut y: f32) -> Element {
    if x > 50.0 && x < 850.0 && y > 50.0 && y < 850.0 {
        x -= 50.0;
        y -= 50.0;
        return Element::Tile(Position((x / 100.0) as usize, 7 - (y / 100.0) as usize));
    }
    Element::None
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut ggez::Context) -> ggez::GameResult {
        Ok(())
    }
    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut ggez::Context,
        button: ggez::event::MouseButton,
        x: f32,
        y: f32,
    ) {
        match get_element(x, y) {
            Element::Tile(mut pos) => match self.selected.clone() {
                Selected::None => {
                    self.selected = Selected::Position(pos);
                    let res = self.game.get_moves(&(pos.clone()).translate());
                    let mut to: Vec<Position> = vec![];
                    for i in res.iter() {
                        to.push(Position::new(&i));
                    }
                    self.help = Overlay::Moves { selected: pos, to };
                }
                Selected::Position(position) => {
                    println!("{:?} -> {:?}", position.clone().translate(), pos.translate());
                    let state = self.game.turn(position.clone().translate(), pos.translate());
                    match state {
                        chess::game::TurnResult::Moved=>(),
                        chess::game::TurnResult::Checked=>(),
                        _=>()
                    }
                    self.parse();
                    self.selected = Selected::None;
                    self.help = Overlay::None;
                }
            },
            _ => {
                self.selected = Selected::None;
                self.help = Overlay::None;
            }
        }
    }
    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        graphics::clear(ctx, [0.0, 0.0, 0.0, 1.0].into());
        self.playing(ctx);
        graphics::present(ctx)?;
        Ok(())
    }
}

pub fn main() -> ggez::GameResult {
    let resource_dir = path::PathBuf::from("./resources");
    let cb = ggez::ContextBuilder::new("super_simple", "ggez")
        .window_setup(ggez::conf::WindowSetup::default().title("Best chess game!"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(1200.0, 900.0))
        .add_resource_path(resource_dir);
    let (ctx, event_loop) = &mut cb.build()?;
    let state = &mut MainState::new()?;
    event::run(ctx, event_loop, state)
}
