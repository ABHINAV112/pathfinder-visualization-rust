mod macroquad_renderer;
mod maze;
mod maze_algo;

use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui};
use macroquad_renderer::MacroQuadRenderer;
use maze::GridManager;
use maze_algo::random_horizontal::random_horizontal;
use maze_algo::random_vertical::random_vertical;

#[macroquad::main("Events")]
async fn main() -> Result<(), ()> {
    let mut grid_manager = GridManager::new((50, 50), Box::new(MacroQuadRenderer::new()));
    loop {
        clear_background(WHITE);

        root_ui().window(
            hash!(),
            Vec2::new(0., screen_height() - 50.),
            Vec2::new(screen_width(), 50.),
            |ui| {
                ui.same_line(0.);
                if ui.button(None, "Wall") {
                    grid_manager.set_grid_value(maze::GridValue::Wall);
                }
                ui.same_line(40.);
                if ui.button(None, "Empty") {
                    grid_manager.set_grid_value(maze::GridValue::Empty);
                }
                ui.same_line(80.);
                if ui.button(None, "Start") {
                    grid_manager.set_grid_value(maze::GridValue::Start);
                }
                ui.same_line(120.);
                if ui.button(None, "End") {
                    grid_manager.set_grid_value(maze::GridValue::End);
                }

                ui.same_line(160.);
                if ui.button(None, "Clear") {
                    grid_manager.add_render_action(maze::RenderAction::Clear);
                }

                ui.same_line(200.);
                if ui.button(None, "Vertical") {
                    grid_manager.add_render_action(maze::RenderAction::Clear);
                    random_vertical(&grid_manager.grid)
                        .into_iter()
                        .for_each(|action| {
                            grid_manager.add_render_action(action);
                        });
                }
                ui.same_line(260.);
                if ui.button(None, "Horizontal") {
                    grid_manager.add_render_action(maze::RenderAction::Clear);
                    random_horizontal(&grid_manager.grid)
                        .into_iter()
                        .for_each(|action| {
                            grid_manager.add_render_action(action);
                        });
                }
            },
        );
        grid_manager.handle_input()?;
        grid_manager.render()?;
        next_frame().await;
    }
}
