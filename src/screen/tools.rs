use ggez;
use ggez::graphics;
use super::super::{Position, Color};
fn draw_tile(ctx: &mut ggez::Context, position: &Position, color: graphics::Color) -> () {
    let rectangle = graphics::Mesh::new_rectangle(
        ctx,
        graphics::DrawMode::fill(),
        graphics::Rect {
            x: 50.0 + 100.0 * (position.0) as f32,
            y: 50.0 + 100.0 * (7 - position.1) as f32,
            w: 100.0,
            h: 100.0,
        },
        color,
    )
    .unwrap();
    graphics::draw(ctx, &rectangle, (na::Point2::new(0.0, 0.0),));
}

fn board_edge(ctx: &mut ggez::Context) -> graphics::Mesh {
    graphics::Mesh::new_rectangle(
        ctx,
        graphics::DrawMode::fill(),
        ggez::graphics::get_screen_cordinates(ctx),
        graphics::Color::from_rgb(0, 0, 66),
    )
    .unwrap()
}