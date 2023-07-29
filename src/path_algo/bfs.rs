use crate::maze::{Grid, GridValue, Index, RenderAction};
use std::collections::VecDeque;

pub fn bfs(grid: &Grid, start: &Option<Index>, end: &Option<Index>) -> Vec<RenderAction> {
    let mut result: Vec<RenderAction> = vec![];
    if let (Some(start), Some(end)) = (start, end) {
        let mut queue: VecDeque<Index> = VecDeque::new();
        queue.push_back(*start);
        let mut visited: Vec<Vec<bool>> = vec![vec![false; grid[0].len()]; grid.len()];
        let mut parent: Vec<Vec<Option<Index>>> = vec![vec![None; grid[0].len()]; grid.len()];

        while !queue.is_empty() {
            let current: Index = queue.pop_front().unwrap();
            if current == *end {
                break;
            }

            let (i, j) = current;
            if i > 0
                && (grid[i - 1][j] == GridValue::Empty || grid[i - 1][j] == GridValue::End)
                && !visited[i - 1][j]
            {
                queue.push_back((i - 1, j));
                visited[i - 1][j] = true;
                parent[i - 1][j] = Some(current);
            }
            if i < grid.len() - 1
                && (grid[i + 1][j] == GridValue::Empty || grid[i + 1][j] == GridValue::End)
                && !visited[i + 1][j]
            {
                queue.push_back((i + 1, j));
                visited[i + 1][j] = true;
                parent[i + 1][j] = Some(current);
            }
            if j > 0
                && (grid[i][j - 1] == GridValue::Empty || grid[i][j - 1] == GridValue::End)
                && !visited[i][j - 1]
            {
                queue.push_back((i, j - 1));
                visited[i][j - 1] = true;
                parent[i][j - 1] = Some(current);
            }
            if j < grid[0].len() - 1
                && (grid[i][j + 1] == GridValue::Empty || grid[i][j + 1] == GridValue::End)
                && !visited[i][j + 1]
            {
                queue.push_back((i, j + 1));
                visited[i][j + 1] = true;
                parent[i][j + 1] = Some(current);
            }
            if current != *start {
                result.push(RenderAction::FillCell(current, GridValue::Highlight));
            }
        }
        let mut iter = *end;
        while iter != *start {
            if iter != *end {
                result.push(RenderAction::FillCell(iter, GridValue::Path));
            }
            iter = parent[iter.0][iter.1].unwrap();
        }
    }
    result
}
