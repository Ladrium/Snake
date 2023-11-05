extern crate piston_window;

mod game;
mod snake;
mod utils;

use game::Game;
use piston_window::*;

const ENTITY_SIZE: f64 = 25.0;

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Snake", (700, 700))
        .exit_on_esc(true)
        .build()
        .unwrap_or_else(|e| panic!("Failed: {} ", e));

    let mut game = Game::new(500, 500, ENTITY_SIZE);

    let mut glyphs = window.load_font("assets/Roboto-Medium.ttf").unwrap();

    while let Some(e) = window.next() {
        if let Some(Button::Keyboard(key)) = e.press_args() {
            game.key_down(key);
        }

        window.draw_2d(&e, |context, graphics, device| {
            game.draw(&context, graphics);
            game.draw_stats(&context, graphics, &mut glyphs);

            glyphs.factory.encoder.flush(device);
        });

        e.update(|arg| {
            game.update(arg.dt);
        });
    }
}
