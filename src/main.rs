// use components::maze::Maze;
use nannou::prelude::*;
mod maze;

use maze::{Dimensions, Maze, MazeBuilder};
fn main() {
    nannou::sketch(view).run();
}

fn view(app: &App, frame: Frame) {
    let draw = app.draw();

    draw.background().color(PLUM);

    let win = app.window_rect();
    win.x_y();

    let mut maze_builder = MazeBuilder::new();
    maze_builder.set_maze_units((100, 100));
    maze_builder.set_overall_dimensions(Dimensions(win.h(), win.w()));
    let maze: Maze = maze_builder.build().expect("lmao");
    println!("{:?}", maze);
    draw.to_frame(app, &frame).unwrap()
}
