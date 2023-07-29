use std::collections::VecDeque;

#[derive(Default, Debug, Clone, Eq, PartialEq)]
pub enum GridValue {
    #[default]
    Empty,
    Wall,
    Start,
    End,
    Highlight,
    Path,
}

pub type Grid = Vec<Vec<GridValue>>;
pub type Index = (usize, usize);

#[derive(Default, Debug, Clone, Eq, PartialEq)]
pub enum RenderAction {
    #[default]
    None,
    FillCell(Index, GridValue),
    FillCellVector(Vec<(Index, GridValue)>),
    Clear,
    ClearHighlight,
}

// the goal is to have a renderer that is independent of the library being used
pub trait GridRenderer {
    fn handle_input(&self, grid: &Grid, grid_value: &GridValue) -> Result<RenderAction, ()>;
    fn render(&self, grid: &Grid) -> Result<(), ()>;
}

pub struct GridManager {
    pub grid: Grid,
    grid_value: GridValue,
    render_queue: VecDeque<RenderAction>,
    renderer: Box<dyn GridRenderer>,
    pub start: Option<Index>,
    pub end: Option<Index>,
}

impl GridManager {
    pub fn new(wh: (usize, usize), renderer: Box<dyn GridRenderer>) -> GridManager {
        GridManager {
            grid: (0..wh.1)
                .map(|_| (0..wh.0).map(|_| GridValue::default()).collect())
                .collect(),
            grid_value: GridValue::Wall,
            render_queue: VecDeque::new(),
            renderer,
            start: None,
            end: None,
        }
    }

    pub fn add_render_action(&mut self, render_action: RenderAction) {
        if let RenderAction::FillCell(index, grid_value) = &render_action {
            match grid_value {
                GridValue::Start => {
                    if let Some(start) = self.start {
                        self.grid[start.0][start.1] = GridValue::Empty;
                    }
                    self.start = Some(index.clone());
                }
                GridValue::End => {
                    if let Some(end) = self.end {
                        self.grid[end.0][end.1] = GridValue::Empty;
                    }
                    self.end = Some(index.clone());
                }
                _ => {}
            }
        }
        self.render_queue.push_back(render_action);
    }

    pub fn set_grid_value(&mut self, grid_value: GridValue) {
        self.grid_value = grid_value;
    }

    pub fn handle_input(&mut self) -> Result<(), ()> {
        let render_action = self.renderer.handle_input(&self.grid, &self.grid_value)?;
        if render_action != RenderAction::None {
            self.add_render_action(render_action);
        }
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
            RenderAction::ClearHighlight => {
                self.grid = self
                    .grid
                    .iter()
                    .map(|row| {
                        row.iter()
                            .map(|grid_value| match grid_value {
                                GridValue::Highlight | GridValue::Path => GridValue::Empty,
                                _ => grid_value.clone(),
                            })
                            .collect()
                    })
                    .collect();
            }
            RenderAction::None => {}
        };

        self.renderer.render(&self.grid)
    }
}
