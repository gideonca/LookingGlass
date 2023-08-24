use ggez::event;
use ggez::graphics::{self, Color, DrawParam, Image};
use ggez::{Context, ContextBuilder, GameResult};
use std::path;

struct MainState {
    sprite: Image,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let sprite = Image::new(ctx, "")?; //Replace with custom location
        Ok(MainState { sprite })
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        // TODO: Game logic here
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, Color::BLACK);

        let draw_param = DrawParam::new().des([100.0, 100.0]); // Position of the asset to draw
        graphics::draw(ctx, &self.sprite, draw_param)?;

        graphics::present(ctx)?;
        Ok(())
    }
}

fn RenderGraphics() -> GameResult {
    let cb = ContextBuilder::new("sprite_rendering_example", "your_username").add_resource_path(path::PathBuf::from("./resources")); // Replace with resource folder path

    let (ctx, event_loop) = &mut cb.build()?;
    let state = MainState::new(ctx)?;
    event::run(ctx, event_loop, state)
}