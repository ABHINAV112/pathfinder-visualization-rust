use crate::maze::{Grid, GridValue, RenderAction};
use rand::prelude::*;

pub fn random_vertical(grid: &Grid) -> Vec<RenderAction> {
    let (i_max, j_max) = (grid.len(), grid[0].len());
    let mut result = vec![];
    for j in (0..j_max).step_by(2) {
        let mut rng = thread_rng();
        let i_empty = rng.gen_range(0..i_max);
        for i in 0..i_max {
            if i != i_empty {
                result.push(RenderAction::FillCell((j, i), GridValue::Wall));
            } else {
                result.push(RenderAction::FillCell((j, i), GridValue::Empty));
            }
        }
    }

    return result;
}
