use std::fmt::Debug;

#[derive(Clone, Debug)]
pub struct Dimensions(pub f32, pub f32);

#[derive(Debug)]
pub enum Cell {
    Start,
    End,
    Empty,
    Wall,
    WeightedWall(i32),
}

pub struct Maze {
    maze: Vec<Vec<Cell>>,
    overall_dimensions: Dimensions,
    unit_dimensions: Dimensions,
}
impl Debug for Maze {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Maze")
            .field("overall_dimensions", &self.overall_dimensions)
            .field("unit_dimensions", &self.unit_dimensions)
            .finish()
    }
}

pub struct MazeBuilder {
    pub maze_units: Option<(i32, i32)>,
    pub overall_dimensions: Option<Dimensions>,
}

#[derive(Debug)]
pub enum MazeBuildError {
    BadMazeUnits,
    BadOverallDimensions,
    BadAttributes,
}

impl MazeBuilder {
    pub fn new() -> MazeBuilder {
        MazeBuilder {
            maze_units: None,
            overall_dimensions: None,
        }
    }

    pub fn set_maze_units(&mut self, maze_units: (i32, i32)) -> &Self {
        self.maze_units = Some(maze_units);
        self
    }

    pub fn set_overall_dimensions(&mut self, dimensions: Dimensions) -> &Self {
        self.overall_dimensions = Some(dimensions);
        self
    }

    pub fn build(self) -> Result<Maze, MazeBuildError> {
        match (self.maze_units, self.overall_dimensions) {
            (None, None) => Err(MazeBuildError::BadAttributes),
            (None, _) => Err(MazeBuildError::BadMazeUnits),
            (_, None) => Err(MazeBuildError::BadOverallDimensions),
            (Some(maze_units), Some(overall_dimensions)) => Ok(Maze {
                maze: (0..maze_units.0)
                    .map(|_| (0..maze_units.1).map(|_| Cell::Empty).collect())
                    .collect(),
                overall_dimensions: overall_dimensions.clone(),
                unit_dimensions: Dimensions(
                    overall_dimensions.0 / maze_units.0 as f32,
                    overall_dimensions.1 / maze_units.1 as f32,
                ),
            }),
        }
    }
}
