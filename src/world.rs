use std::array;
use std::num;

use nannou::wgpu::PUSH_CONSTANT_ALIGNMENT;
use nannou::{prelude::*, rand::seq::index};

use crate::cell::*;
use crate::configs::*;

pub struct World {
    cells: Vec<Cell>,
    width: i32,
    height: i32,
    cell_size_px: i32,
}

struct NeighboursToCheck {
    neighbours: Vec<Cell>,
    already_checked: bool,
}

impl World {
    pub fn new(width: i32, height: i32, cell_size_px: i32) -> World {
        let mut cells_new: Vec<Cell> = vec![];

        for x in 0..CELL_COLUMNS {
            for y in 0..CELL_ROWS {
                if x == 50 && y == 50 {
                    cells_new.push(Cell {
                        x: x,
                        y: y,
                        is_alive: true,
                    });
                } else if x == 51 && y == 51 {
                    cells_new.push(Cell {
                        x: x,
                        y: y,
                        is_alive: true,
                    });
                } else if x == 50 && y == 51 {
                    cells_new.push(Cell {
                        x: x,
                        y: y,
                        is_alive: true,
                    });
                } else if x == 51 && y == 50 {
                    cells_new.push(Cell {
                        x: x,
                        y: y,
                        is_alive: true,
                    });
                } else if x == 11 && y == 12 {
                    cells_new.push(Cell {
                        x: x,
                        y: y,
                        is_alive: true,
                    });
                } else if x == 12 && y == 11 {
                    cells_new.push(Cell {
                        x: x,
                        y: y,
                        is_alive: true,
                    });
                } else if x == 10 && y == 10 {
                    cells_new.push(Cell {
                        x: x,
                        y: y,
                        is_alive: true,
                    });
                } else if x == 11 && y == 10 {
                    cells_new.push(Cell {
                        x: x,
                        y: y,
                        is_alive: true,
                    });
                } else if x == 12 && y == 10 {
                    cells_new.push(Cell {
                        x: x,
                        y: y,
                        is_alive: true,
                    });
                } else if x == 0 && y == 0 {
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

    pub fn add_living_cell(&mut self, living_cell: Cell) {
        let position: usize = ((self.width * living_cell.x) + living_cell.y) as usize;
        self.cells.get_mut(position).unwrap().is_alive = true;
    }

    pub fn add_dead_cell(&mut self, dead_cell: Cell) {
        let position: usize = ((self.width * dead_cell.x) + dead_cell.y) as usize;
        self.cells.get_mut(position).unwrap().is_alive = false;
    }

    //add neighbours to neighbours_to_check!!!!!!!!!!!
    pub fn update(&mut self) {
        let living_cells: Vec<Cell> = self
            .cells
            .clone()
            .into_iter()
            .filter(|cell| cell.is_alive == true)
            .collect();

        let mut neighbours_to_check = NeighboursToCheck {
            neighbours: vec![],
            already_checked: false,
        };

        let mut cells_to_change: Vec<Cell> = vec![];
        for living_cell in living_cells {
            let living_neighbours_count = number_of_living_neighbours(
                &self.cells,
                self.width,
                self.height,
                &living_cell,
                &mut neighbours_to_check,
            );
            // cell should not change state mid-update, state updates should occur after all states are
            // evaluated
            if cell_in_world_bounds(self.width, self.height, &living_cell) {
                match cell_state_from_neighbour_count(&living_cell, living_neighbours_count) {
                    CellState::ALIVE => cells_to_change.push(Cell {
                        x: living_cell.x,
                        y: living_cell.y,
                        is_alive: true,
                    }),
                    //change_cell_state(
                    //&mut self.cells,
                    //self.width,
                    //self.height,
                    //&living_cell,
                    //true,
                    //),
                    CellState::DEAD => cells_to_change.push(Cell {
                        x: living_cell.x,
                        y: living_cell.y,
                        is_alive: false,
                    }),
                    //change_cell_state(
                    //    &mut self.cells,
                    //    self.width,
                    //    self.height,
                    //    &living_cell,
                    //    false,
                    //),
                }
            }
        }

        for dead_neighbour_cell in neighbours_to_check.neighbours.clone() {
            let living_neighbours_count = number_of_living_neighbours(
                &self.cells,
                self.width,
                self.height,
                &dead_neighbour_cell,
                &mut neighbours_to_check,
            );
            match cell_state_from_neighbour_count(&dead_neighbour_cell, living_neighbours_count) {
                CellState::ALIVE => cells_to_change.push(Cell {
                    x: dead_neighbour_cell.x,
                    y: dead_neighbour_cell.y,
                    is_alive: true,
                }),

                CellState::DEAD => cells_to_change.push(Cell {
                    x: dead_neighbour_cell.x,
                    y: dead_neighbour_cell.y,
                    is_alive: false,
                }),
            }
        }

        for changing_cell in cells_to_change {
            change_cell_state(&mut self.cells, self.width, self.height, &changing_cell)
        }
    }
}
fn change_cell_state(cells: &mut Vec<Cell>, width: i32, height: i32, changing_cell: &Cell) {
    let position: usize = ((width * changing_cell.x) + changing_cell.y) as usize;
    let mut cell_to_change = cells.get_mut(position).unwrap();
    cell_to_change.is_alive = changing_cell.is_alive;
}
fn number_of_living_neighbours(
    cells: &Vec<Cell>,
    width: i32,
    height: i32,
    cell: &Cell,
    neighbours_to_check: &mut NeighboursToCheck,
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

    println!("##################################");
    println!("Checking states for cell: x:{},y:{}", cell.x, cell.y);
    for state in neighbours_states {
        let state_cell: Cell = Cell {
            x: cell.x + state.get(0).unwrap(),
            y: cell.y + state.get(1).unwrap(),
            is_alive: false,
        };
        if cell_in_world_bounds(width, height, &state_cell) {
            println!("Check cell in bound: x:{},y:{}", state_cell.x, state_cell.y);
            if is_cell_alive(&cells, &state_cell) {
                number_of_living_n += 1;
            } else if neighbours_to_check.already_checked == false {
                neighbours_to_check.neighbours.push(state_cell);
            }
        }
    }
    number_of_living_n
}

fn cell_in_world_bounds(width: i32, height: i32, cell: &Cell) -> bool {
    if cell.x >= width {
        false
    } else if cell.x < 0 {
        false
    } else if cell.y >= height {
        false
    } else if cell.y < 0 {
        false
    } else {
        true
    }
}

fn is_cell_alive(cells: &Vec<Cell>, cell: &Cell) -> bool {
    let position: usize = ((CELL_COLUMNS * cell.x) + cell.y) as usize;
    println!("pos: {}, size: {}", position, cells.len());
    let world_cell = cells.get(position).unwrap();
    println!("pos: x:{},y:{}", world_cell.x, world_cell.y);
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
