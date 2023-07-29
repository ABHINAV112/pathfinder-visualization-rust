use std::collections::VecDeque;

#[derive(Default, Debug)]
pub enum GridValue {
    #[default]
    Empty,
    Wall,
    Start,
    End,
}

pub type Grid = Vec<Vec<GridValue>>;
pub type Index = (usize, usize);

#[derive(Default)]
pub enum RenderAction {
    #[default]
    None,
    FillCell(Index, GridValue),
    FillCellVector(Vec<(Index, GridValue)>),
    Clear,
    ToggleCell(Index),
}

// the goal is to have a renderer that is independent of the library being used
pub trait GridRenderer {
    fn handle_input(&self, grid: &Grid) -> Result<RenderAction, ()>;
    fn render(&self, grid: &Grid) -> Result<(), ()>;
}

pub struct GridManager {
    grid: Grid,
    render_queue: VecDeque<RenderAction>,
    renderer: Box<dyn GridRenderer>,
}

impl GridManager {
    pub fn new(wh: (usize, usize), renderer: Box<dyn GridRenderer>) -> GridManager {
        GridManager {
            grid: (0..wh.1)
                .map(|_| (0..wh.0).map(|_| GridValue::default()).collect())
                .collect(),
            render_queue: VecDeque::new(),
            renderer,
        }
    }

    pub fn add_render_action(&mut self, render_action: RenderAction) {
        self.render_queue.push_back(render_action);
    }

    pub fn handle_input(&mut self) -> Result<(), ()> {
        let render_action = self.renderer.handle_input(&self.grid)?;
        self.add_render_action(render_action);
        Ok(())
    }

    pub fn render(&mut self) -> Result<(), ()> {
        let render_action = match self.render_queue.pop_front() {
            None => return self.renderer.render(&self.grid),
            Some(val) => val,
        };
        match render_action {
            RenderAction::FillCell(index, grid_value) => self.grid[index.0][index.1] = grid_value,
            RenderAction::FillCellVector(fill_actions) => {
                fill_actions
                    .into_iter()
                    .map(|(index, grid_value)| self.grid[index.0][index.1] = grid_value)
                    .count();
            }
            RenderAction::Clear => {
                self.grid = self
                    .grid
                    .iter()
                    .map(|row| row.iter().map(|_| GridValue::Empty).collect())
                    .collect();
            }
            RenderAction::ToggleCell(index) => {
                self.grid[index.0][index.1] = match self.grid[index.0][index.1] {
                    GridValue::Empty => GridValue::Wall,
                    _ => GridValue::Empty,
                }
            }
            RenderAction::None => {}
        };

        self.renderer.render(&self.grid)
    }
}
