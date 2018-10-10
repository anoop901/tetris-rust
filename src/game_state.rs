extern crate itertools;

use ::bag;
use ::tetromino_data;

// TODO: make these constants configurable at runtime

/// The number of pieces that the player can see in advance.
pub const NEXT_PREVIEW_LENGTH: usize = 5;

/// Width of the play field.
pub const MATRIX_WIDTH: usize = 10;
/// Height of the play field (both visible and hidden parts).
pub const MATRIX_HEIGHT: usize = 22;

/// Describes a tetromino of some type at some position and orientation on the
/// matrix.
#[derive(Debug, Clone)]
pub struct Tetromino {
    ttype: ::TetrominoType,
    center: (isize, isize),
    orientation: u32
}

impl Tetromino {

    /// Returns a vector of the positions of each mino in this tetromino.
    pub fn minoes(&self) -> Vec<(isize, isize)> {

        let shape = tetromino_data::tetromino_shape_from(&self.ttype);
        let center = self.center;

        shape.iter().map(|mino_offset| {

            let rotated_offset = match self.orientation {
                0 => (mino_offset.0, mino_offset.1),
                1 => (mino_offset.1, -mino_offset.0),
                2 => (-mino_offset.0, -mino_offset.1),
                3 => (-mino_offset.1, mino_offset.0),
                _ => (mino_offset.0, mino_offset.1),
            };

            (
                center.0 + rotated_offset.0,
                center.1 + rotated_offset.1
            )
        }).collect()
    }
}

impl Tetromino {

    /// Creates a tetromino of the given type in spawn position.
    pub fn new(tt: ::TetrominoType) -> Tetromino {
        Tetromino {
            ttype: tt,
            center: (4, (MATRIX_HEIGHT - 2) as isize),
            orientation: 0
        }
    }

    /// Returns the grid coordinates of the center of this tetromino.
    /// 
    /// The first index identifies the row, where 0 is the bottom row. The second
    /// index identifies the column, where 0 is the leftmost column.
    pub fn center(&self) -> (isize, isize) {
        return self.center;
    }

    /// Returns the type of tetromino.
    pub fn ttype(&self) -> &::TetrominoType {
        return &self.ttype;
    }

}

/// Describes the state of a Tetris game.
///
/// Rows are indexed from bottom to top, where the bottom row is indexed as 0.
/// Columns are indexed from left to right, where the left-most row is indexed
/// as 0. Coordinates are specified as (col, row).
#[derive(Debug)]
pub struct GameState {
    placed_squares: Vec<Vec<Option<::TetrominoType>>>,

    falling_tetromino: Tetromino,

    /// The bag for determining the next tetrominoes.
    bag: bag::Bag,

    next_preview: Vec<::TetrominoType>,

    held: Option<::TetrominoType>
}

impl GameState {

    /// Create a new `GameState` representing the initial state of a tetris
    /// game.
    pub fn new() -> Self {

        let mut bag = bag::Bag::new();

        let initial_falling_tetromino_type = bag.draw();
        let initial_next_preview =
            (0..NEXT_PREVIEW_LENGTH)
            .map(|_| bag.draw())
            .collect();

        GameState {
            placed_squares: vec![vec![None; MATRIX_HEIGHT]; MATRIX_WIDTH],
            falling_tetromino: Tetromino::new(initial_falling_tetromino_type),
            bag: bag,
            next_preview: initial_next_preview,
            held: None
        }
    }

    // Getters

    /// Returns a representation of all the squares that have previously been
    /// placed the matrix. It is expressed as a `Vec<Vec<Option<::TetrominoType>>>`
    /// where:
    ///
    /// - `placed_squares[i][j] == Some(tt)` indicates that there has been a
    ///   square placed at coordinates (`i`, `j`), which came from a tetromino
    ///   of type `tt`.
    /// - `placed_squares[i][j] == None` indicates that there has not been any
    ///   square placed at coordinates (`i`, `j`).
    pub fn placed_squares(&self) -> &Vec<Vec<Option<::TetrominoType>>> {
        return &self.placed_squares;
    }

    /// A `Tetromino` object representing the currently falling tetromino.
    pub fn falling_tetromino(&self) -> &Tetromino {
        return &self.falling_tetromino
    }

    /// The next pieces that will drop.
    pub fn next_preview(&self) -> &[::TetrominoType] {
        return &self.next_preview.as_slice();
    }

    /// The held tetromino, if any.
    pub fn held(&self) -> &Option<::TetrominoType> {
        return &self.held;
    }

    // Actions that can be made by the player

    /// Moves the currently falling piece down, due to gravity. Returns `true`
    /// if the tetromino was moved successfully, or `false` if it hit the floor.
    pub fn apply_gravity(&mut self) -> bool {

        let new_center = (self.falling_tetromino.center.0,
                            self.falling_tetromino.center.1 - 1);
        let orientation = self.falling_tetromino.orientation;
        self.move_tetromino_if_fits(new_center, orientation)
    }

    /// Moves the current tetromino to the left, if it can. Returns `true` if
    /// the tetromino was moved successfully, else returns `false`.
    pub fn move_left(&mut self) -> bool {
        let new_center = (self.falling_tetromino.center.0 - 1,
                            self.falling_tetromino.center.1);
        let orientation = self.falling_tetromino.orientation;
        self.move_tetromino_if_fits(new_center, orientation)
    }

    /// Moves the current tetromino to the right, if it can. Returns `true` if
    /// the tetromino was moved successfully, else returns `false`.
    pub fn move_right(&mut self) -> bool {
        let new_center = (self.falling_tetromino.center.0 + 1,
                          self.falling_tetromino.center.1);
        let orientation = self.falling_tetromino.orientation;
        self.move_tetromino_if_fits(new_center, orientation)
    }

    /// Rotates the current tetromino counter-clockwise, if it can. All the
    /// kicks specified in the SRS kick tables will be attempted. Returns `true`
    /// if a rotation successfully occurred, else returns `false`.
    pub fn rotate_left(&mut self) -> bool {
        let old_orientation = self.falling_tetromino.orientation;
        self.rotate_to_orientation((old_orientation + 3) % 4)
    }

    /// Rotates the current tetromino clockwise, if it can. All the kicks
    /// specified in the SRS kick tables will be attempted. Returns `true` if a
    /// rotation successfully occurred, else returns `false`.
    pub fn rotate_right(&mut self) -> bool {
        let old_orientation = self.falling_tetromino.orientation;
        self.rotate_to_orientation((old_orientation + 1) % 4)
    }

    pub fn hard_drop(&mut self) {
        while self.apply_gravity() {}
        self.lock_piece();
    }

    pub fn hold(&mut self) {
        // TODO: this shouldn't be allowed twice in a row
        let new_held = self.falling_tetromino.ttype.clone();
        if let Some(ref old_held) = self.held {
            self.falling_tetromino = Tetromino::new(old_held.clone());
        } else {
            Some(self.falling_tetromino.ttype.clone());
            self.spawn_next_piece();
        }
        self.held = Some(new_held);
    }

    /// Locks the currently falling tetromino on the matrix and spawns the next
    /// tetromino.
    pub fn lock_piece(&mut self) {

        // place tetromino squares on matrix
        self.falling_tetromino.minoes().iter().for_each(|mino_position| {
            self.placed_squares[mino_position.0 as usize][mino_position.1 as usize] = Some(self.falling_tetromino.ttype.clone());
        });

        self.clear_lines();

        self.spawn_next_piece();

        // TODO: activate pending garbage lines
        // TODO: detect game over
    }

    // Helpers

    /// Clears any full lines that are on the matrix, then moves the above lines
    /// down.
    fn clear_lines(&mut self) {
        let mut num_cleared_rows = 0;

        // Write the contents of each row into the below row into which it fell. num_cleared_rows
        // keeps track of how many rows to move down.
        for row in 0..MATRIX_HEIGHT {
            let row_filled = (0..MATRIX_WIDTH).all(|col| {
                self.placed_squares[col][row].is_some()
            });
            if row_filled {
                // This row is filled, it will be overwritten by a higher row.
                num_cleared_rows += 1;
            } else {
                // Write this row num_cleared_rows below.
                for col in 0..MATRIX_WIDTH {
                    self.placed_squares[col][row - num_cleared_rows] = self.placed_squares[col][row].clone();
                }
            }
        }

        // Empty the top num_cleared_rows rows.
        for row in MATRIX_HEIGHT-num_cleared_rows..MATRIX_HEIGHT {
            for col in 0..MATRIX_WIDTH {
                self.placed_squares[col][row] = None;
            }
        }
    }

    fn spawn_next_piece(&mut self) {
        
        // add a new item to the end of the preview
        self.next_preview.push(self.bag.draw());

        // spawn the next tetromino
        self.falling_tetromino = Tetromino::new(self.next_preview.remove(0));
    }

    fn move_tetromino_if_fits(&mut self, new_center: (isize, isize), new_orientation: u32) -> bool {
        let candidate = Tetromino {
            ttype: self.falling_tetromino.ttype.clone(),
            center: new_center,
            orientation: new_orientation
        };
        let fits = self.tetromino_fits(&candidate);

        if fits {
            self.falling_tetromino = candidate;
        }

        fits
    }

    fn tetromino_fits(&self, t: &Tetromino) -> bool {
        t.minoes().iter().all(|mino_position| {
            mino_position.0 >= 0 &&
            mino_position.0 < (MATRIX_WIDTH as isize) &&
            mino_position.1 >= 0 &&
            mino_position.1 < (MATRIX_HEIGHT as isize) &&
            self.placed_squares[mino_position.0 as usize][mino_position.1 as usize].is_none()
        })
    }

    fn rotate_to_orientation(&mut self, new_orientation: u32) -> bool {
        let old_orientation = self.falling_tetromino.orientation;
        for offset_data in tetromino_data::tetromino_offset_data_from(&self.falling_tetromino.ttype) {

            let offset = (
                offset_data[old_orientation as usize].0 - offset_data[new_orientation as usize].0,
                offset_data[old_orientation as usize].1 - offset_data[new_orientation as usize].1
            );

            let candidate_tetromino = Tetromino {
                ttype: self.falling_tetromino.ttype.clone(),
                center: (
                    self.falling_tetromino.center.0 + offset.0,
                    self.falling_tetromino.center.1 + offset.1,
                ),
                orientation: new_orientation,
            };

            if self.tetromino_fits(&candidate_tetromino) {
                self.falling_tetromino = candidate_tetromino;
                return true;
            }
        }
        false
    }
}