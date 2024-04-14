use crate::GAME_FIELD_HEIGHT;
use crate::GAME_FIELD_WIDTH;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameState {
    Playing,
    Paused,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CellState {
    Alive,
    Dead,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell {
    pub posx: i32,
    pub posy: i32,
    pub state: CellState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GameOfLife {
    pub state: GameState,
    field: [Cell; (GAME_FIELD_WIDTH * GAME_FIELD_HEIGHT) as usize],
}

impl GameOfLife {
    pub fn new() -> GameOfLife {
        let mut pg =
            [Cell::new(0, 0, CellState::Dead); (GAME_FIELD_WIDTH * GAME_FIELD_HEIGHT) as usize];
        for i in 0..GAME_FIELD_HEIGHT {
            for j in 0..GAME_FIELD_WIDTH {
                pg[(j as u32 + ((i as u32) * GAME_FIELD_WIDTH)) as usize] =
                    Cell::new(i as i32, j as i32, CellState::Dead);
            }
        }

        GameOfLife {
            state: GameState::Paused,
            field: pg,
        }
    }

    pub fn get(&self, x: i32, y: i32) -> Option<Cell> {
        if x >= 0 && y >= 0 && (x as u32) < GAME_FIELD_WIDTH && (y as u32) < GAME_FIELD_HEIGHT {
            return Some(self.field[(x as u32 + (y as u32) * GAME_FIELD_WIDTH) as usize]);
        } else {
            return None;
        }
    }

    pub fn get_mut(&mut self, x: i32, y: i32) -> Option<&mut Cell> {
        if x >= 0 && y >= 0 && (x as u32) < GAME_FIELD_WIDTH && (y as u32) < GAME_FIELD_HEIGHT {
            return Some(&mut self.field[(x as u32 + (y as u32) * GAME_FIELD_WIDTH) as usize]);
        } else {
            return None;
        }
    }

    pub fn toggle_state(&mut self) {
        self.state = match self.state {
            GameState::Paused => GameState::Playing,
            GameState::Playing => GameState::Paused,
        }
    }

    pub fn update(&mut self) {
        match self.state {
            GameState::Paused => {}
            GameState::Playing => {
                let mut tmp_field = self.field;
                for (u, cell) in tmp_field.iter_mut().enumerate() {
                    let u = u as u32;
                    let x = u % GAME_FIELD_WIDTH;
                    let y = u / GAME_FIELD_WIDTH;

                    let mut count: u32 = 0;
                    for i in -1..2 {
                        for j in -1..2 {
                            if !(i == 0 && j == 0) {
                                let px: i32 = (x as i32) + i;
                                let py: i32 = (y as i32) + j;

                                let c = self
                                    .get(px, py)
                                    .unwrap_or_else(|| Cell::new(0, 0, CellState::Dead));

                                if &c != cell {
                                    if c.state == CellState::Alive {
                                        count += 1;
                                    }
                                }
                            }
                        }
                    }
                    if count < 2 {
                        cell.state = CellState::Dead;
                    } else if count > 3 {
                        cell.state = CellState::Dead;
                    } else if count == 3 {
                        cell.state = CellState::Alive;
                    }
                }
                self.field = tmp_field;
            }
        }
    }
}

impl<'a> IntoIterator for &'a GameOfLife {
    type Item = &'a Cell;
    type IntoIter = ::std::slice::Iter<'a, Cell>;
    fn into_iter(self) -> ::std::slice::Iter<'a, Cell> {
        return self.field.iter();
    }
}

impl Cell {
    pub fn new(x: i32, y: i32, state: CellState) -> Cell {
        return Cell {
            posx: x,
            posy: y,
            state,
        };
    }

    pub fn toggle_state(&mut self) {
        self.state = match self.state {
            CellState::Dead => CellState::Alive,
            CellState::Alive => CellState::Dead,
        }
    }
}
