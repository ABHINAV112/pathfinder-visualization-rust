use crate::maze::{Grid, GridRenderer, GridValue, Index, RenderAction};
use macroquad::prelude::*;

pub struct MacroQuadRenderer {}

impl MacroQuadRenderer {
    pub fn new() -> Self {
        MacroQuadRenderer {}
    }
    fn get_unit_dimesions(&self, grid: &Grid) -> (f32, f32) {
        (
            screen_width() / grid.len() as f32,
            screen_height() / grid[0].len() as f32,
        )
    }
    fn get_color(&self, grid_value: &GridValue) -> Color {
        match grid_value {
            GridValue::Wall => GRAY,
            GridValue::End => RED,
            GridValue::Empty => WHITE,
            GridValue::Start => GREEN,
        }
    }
}
impl GridRenderer for MacroQuadRenderer {
    fn handle_input(&self, grid: &Grid) -> Result<RenderAction, ()> {
        let unit_dimensions: (f32, f32) = self.get_unit_dimesions(grid);
        if is_mouse_button_down(MouseButton::Left) {
            println!("mouse positions {:?}", mouse_position());
            let mouse_pos_index: Index = (
                (mouse_position().0 / unit_dimensions.0) as usize,
                (mouse_position().1 / unit_dimensions.1) as usize,
            );
            return Ok(RenderAction::FillCell(mouse_pos_index, GridValue::Wall));
        }
        Ok(RenderAction::None)
    }

    fn render(&self, grid: &Grid) -> Result<(), ()> {
        let unit_dimensions: (f32, f32) = self.get_unit_dimesions(grid);
        for (i, row) in grid.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                let color = self.get_color(cell);
                draw_rectangle(
                    i as f32 * unit_dimensions.0,
                    j as f32 * unit_dimensions.1,
                    unit_dimensions.0 * 0.95,
                    unit_dimensions.1 * 0.95,
                    color,
                );
            }
        }
        Ok(())
    }
}
