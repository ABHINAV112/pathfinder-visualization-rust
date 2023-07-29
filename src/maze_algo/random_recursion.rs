use crate::maze::{Grid, GridValue, Index, RenderAction};
use rand::prelude::*;

pub fn random_recursion(grid: &Grid) -> Vec<RenderAction> {
    let (i_max, j_max) = (grid.len(), grid[0].len());
    let mut result = vec![];

    let mut full_wall: Vec<(Index, GridValue)> = vec![];
    for j in 0..j_max {
        for i in 0..i_max {
            full_wall.push(((i, j), GridValue::Wall));
        }
    }
    for j in (0..j_max).step_by(2) {
        for i in (0..i_max).step_by(2) {
            full_wall.push(((i, j), GridValue::Empty));
        }
    }
    result.push(RenderAction::FillCellVector(full_wall));
    result
}
