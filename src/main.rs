use ggez::{conf, event, Context, GameResult};

struct Game {}

impl event::EventHandler<ggez::GameError> for Game {
    fn update(&mut self, _context: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, _context: &mut Context) -> GameResult {
        Ok(())
    }
}

pub fn main() -> GameResult {
    let context_builder = ggez::ContextBuilder::new("sokoban", "jalever")
        .window_setup(conf::WindowSetup::default().title("Sokoban"))
        .window_mode(conf::WindowMode::default().dimensions(800.0, 600.0))
        .add_resource_path("./resources");

    let (context, event_loop) = context_builder.build()?;
    let game = Game {};
    event::run(context, event_loop, game)
}
