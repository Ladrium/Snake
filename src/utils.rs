use piston_window::{rectangle, types::Color, Context, G2d};

const ENTITY_SIZE: i32 = 25;

pub fn gui_coords((x, y): (i32, i32)) -> (f64, f64) {
    let x = x * ENTITY_SIZE + 100;
    let y = y * ENTITY_SIZE + 100;
    (x as f64, y as f64)
}
pub fn draw_block(color: Color, x: i32, y: i32, con: &Context, g: &mut G2d) {
    let (x, y) = gui_coords((x, y));
    rectangle(
        color,
        [x, y, ENTITY_SIZE as f64, ENTITY_SIZE as f64],
        con.transform,
        g,
    );
}
