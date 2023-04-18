use std::array;
use std::num;

use nannou::{prelude::*, rand::seq::index};

use crate::cell::*;
use crate::configs::*;

pub struct World {
    cells: Vec<Cell>,
    width: u32,
    height: u32,
    cell_size_px: u32,
}

struct NeighboursState {
    neighbours_to_check: Vec<Cell>,
    living_neighbours_count: u32,
}

impl World {
    pub fn new(width: u32, height: u32, cell_size_px: u32) -> World {
        let mut cells_new: Vec<Cell> = vec![];

        for x in 0..CELL_COLUMNS {
            for y in 0..CELL_ROWS {
                if x == y {
                    cells_new.push(Cell {
                        x: x,
                        y: y,
                        is_alive: true,
                    });
                } else {
                    cells_new.push(Cell {
                        x: x,
                        y: y,
                        is_alive: false,
                    });
                }
            }
        }

        World {
            cells: cells_new,
            width: width,
            height: height,
            cell_size_px: cell_size_px,
        }
    }

    //add neighbours to neighbours_to_check!!!!!!!!!!!
    pub fn update(&mut self) {
        let living_cells: Vec<Cell> = self
            .cells
            .clone()
            .into_iter()
            .filter(|cell| cell.is_alive == true)
            .collect();

        let mut neighbours_to_check: Vec<Cell> = vec![];

        for living_cell in living_cells {
            let living_neighbours_count = number_of_living_neighbours(
                &self.cells,
                self.width,
                self.height,
                &living_cell,
                &mut Some(&mut neighbours_to_check),
            );
            if cell_in_world_bounds(self.width, self.height, &living_cell) {
                match cell_state_from_neighbour_count(&living_cell, living_neighbours_count) {
                    CellState::ALIVE => change_cell_state(
                        &mut self.cells,
                        self.width,
                        self.height,
                        &living_cell,
                        true,
                    ),
                    CellState::DEAD => change_cell_state(
                        &mut self.cells,
                        self.width,
                        self.height,
                        &living_cell,
                        false,
                    ),
                }
            }
        }

        for dead_cell in neighbours_to_check {
            let living_neighbours_count = number_of_living_neighbours(
                &self.cells,
                self.width,
                self.height,
                &dead_cell,
                &mut None,
            );
        }
    }
}
fn change_cell_state(
    cells: &mut Vec<Cell>,
    width: u32,
    height: u32,
    cell: &Cell,
    desired_state: bool,
) {
    let position: usize = ((width * cell.x) + cell.y) as usize;
    let mut cell_to_change = cells.get_mut(position).unwrap();
    cell_to_change.is_alive = desired_state;
}
fn number_of_living_neighbours(
    cells: &Vec<Cell>,
    width: u32,
    height: u32,
    cell: &Cell,
    neighbours_to_check: &mut Option<&mut Vec<Cell>>,
) -> u32 {
    let neighbours_states: Vec<[i32; 2]> = vec![
        [-1, -1],
        [-1, 0],
        [-1, 1],
        [0, -1],
        [0, 1],
        [1, -1],
        [1, 0],
        [1, 1],
    ];

    let mut number_of_living_n = 0;

    for state in neighbours_states {
        let state_cell: Cell = Cell {
            x: (cell.x as i32 + state.get(0).unwrap()) as u32,
            y: (cell.y as i32 + state.get(0).unwrap()) as u32,
            is_alive: false,
        };
        if cell_in_world_bounds(width, height, &state_cell) {
            if is_cell_alive(&cells, &state_cell) {
                number_of_living_n += 1;
            } else if neighbours_to_check.is_some() {
                neighbours_to_check.unwrap().push(state_cell);
            }
        }
    }
    number_of_living_n
}

fn cell_in_world_bounds(width: u32, height: u32, cell: &Cell) -> bool {
    if cell.x < 0 {
        false
    } else if cell.x > width {
        false
    } else if cell.y < 0 {
        false
    } else if cell.y > height {
        false
    } else {
        true
    }
}

fn is_cell_alive(cells: &Vec<Cell>, cell: &Cell) -> bool {
    let position: usize = ((CELL_COLUMNS * cell.x) + cell.y) as usize;
    let world_cell = cells.get(position).unwrap();
    return world_cell.is_alive;
}

impl Drawable for World {
    fn draw(&self, draw: &Draw) {
        for cell in self.cells.iter() {
            let cell_color = if cell.is_alive == true { BLACK } else { WHITE };
            let mut x_pos = cell.x * CELL_PX_SIZE;
            x_pos -= SCREEN_WIDTH / 2;
            let mut y_pos = cell.y * CELL_PX_SIZE;
            y_pos -= SCREEN_HEIGHT / 2;

            draw.rect()
                .x_y(x_pos as f32, y_pos as f32)
                .color(cell_color)
                .width(CELL_PX_SIZE as f32 - 0.5)
                .height(CELL_PX_SIZE as f32 - 0.5)
                .stroke_color(BLACK)
                .stroke_weight(0.5);
        }
    }
}
